use anyhow::Result;
use baml_types::LiteralValue;
use itertools::Itertools;
use std::borrow::Cow;

use crate::{field_type_attributes, type_check_attributes, TypeCheckAttributes};

use super::python_language_features::ToPython;
use internal_baml_core::ir::{
    repr::{Docstring, IntermediateRepr, Walker},
    ClassWalker, EnumWalker, FieldType, IRHelper,
};

#[derive(askama::Template)]
#[template(path = "types.py.j2", escape = "none")]
pub(crate) struct PythonTypes<'ir> {
    enums: Vec<PythonEnum<'ir>>,
    classes: Vec<PythonClass<'ir>>,
    structural_recursive_alias_cycles: Vec<PythonTypeAlias<'ir>>,
}

#[derive(askama::Template)]
#[template(path = "type_builder.py.j2", escape = "none")]
pub(crate) struct TypeBuilder<'ir> {
    enums: Vec<PythonEnum<'ir>>,
    classes: Vec<PythonClass<'ir>>,
}

struct PythonEnum<'ir> {
    name: &'ir str,
    values: Vec<(&'ir str, Option<String>)>,
    dynamic: bool,
    docstring: Option<String>,
}

struct PythonClass<'ir> {
    name: Cow<'ir, str>,
    /// The docstring for the class, including comment delimiters.
    docstring: Option<String>,
    // the name, type and docstring of the field.
    fields: Vec<(Cow<'ir, str>, String, Option<String>)>,
    dynamic: bool,
}

struct PythonTypeAlias<'ir> {
    name: Cow<'ir, str>,
    target: String,
}

#[derive(askama::Template)]
#[template(path = "partial_types.py.j2", escape = "none")]
pub(crate) struct PythonStreamTypes<'ir> {
    partial_classes: Vec<PartialPythonClass<'ir>>,
}

/// The Python class corresponding to Partial<TypeDefinedInBaml>
struct PartialPythonClass<'ir> {
    name: &'ir str,
    dynamic: bool,
    /// The docstring for the class, including comment delimiters.
    docstring: Option<String>,
    // the name, type and docstring of the field.
    fields: Vec<(&'ir str, String, Option<String>)>,
}

impl<'ir> TryFrom<(&'ir IntermediateRepr, &'_ crate::GeneratorArgs)> for PythonTypes<'ir> {
    type Error = anyhow::Error;

    fn try_from(
        (ir, _): (&'ir IntermediateRepr, &'_ crate::GeneratorArgs),
    ) -> Result<PythonTypes<'ir>> {
        Ok(PythonTypes {
            enums: ir.walk_enums().map(PythonEnum::from).collect::<Vec<_>>(),
            classes: ir.walk_classes().map(PythonClass::from).collect::<Vec<_>>(),
            structural_recursive_alias_cycles: ir
                .walk_alias_cycles()
                .map(PythonTypeAlias::from)
                .collect::<Vec<_>>(),
        })
    }
}

impl<'ir> TryFrom<(&'ir IntermediateRepr, &'_ crate::GeneratorArgs)> for TypeBuilder<'ir> {
    type Error = anyhow::Error;

    fn try_from(
        (ir, _): (&'ir IntermediateRepr, &'_ crate::GeneratorArgs),
    ) -> Result<TypeBuilder<'ir>> {
        Ok(TypeBuilder {
            enums: ir.walk_enums().map(PythonEnum::from).collect::<Vec<_>>(),
            classes: ir.walk_classes().map(PythonClass::from).collect::<Vec<_>>(),
        })
    }
}

impl<'ir> From<EnumWalker<'ir>> for PythonEnum<'ir> {
    fn from(e: EnumWalker<'ir>) -> PythonEnum<'ir> {
        PythonEnum {
            name: e.name(),
            dynamic: e.item.attributes.get("dynamic_type").is_some(),
            values: e
                .item
                .elem
                .values
                .iter()
                .map(|v| (v.0.elem.0.as_str(), v.1.as_ref().map(render_docstring)))
                .collect(),
            docstring: e.item.elem.docstring.as_ref().map(render_docstring),
        }
    }
}

impl<'ir> From<ClassWalker<'ir>> for PythonClass<'ir> {
    fn from(c: ClassWalker<'ir>) -> Self {
        PythonClass {
            name: Cow::Borrowed(c.name()),
            dynamic: c.item.attributes.get("dynamic_type").is_some(),
            fields: c
                .item
                .elem
                .static_fields
                .iter()
                .map(|f| {
                    (
                        Cow::Borrowed(f.elem.name.as_str()),
                        add_default_value(
                            &f.elem.r#type.elem.to_type_ref(c.db, false),
                        ),
                        f.elem.docstring.as_ref().map(render_docstring),
                    )
                })
                .collect(),
            docstring: c.item.elem.docstring.as_ref().map(render_docstring),
        }
    }
}

// TODO: Define AliasWalker to simplify type.
impl<'ir> From<Walker<'ir, (&'ir String, &'ir FieldType)>> for PythonTypeAlias<'ir> {
    fn from(
        Walker {
            db,
            item: (name, target),
        }: Walker<(&'ir String, &'ir FieldType)>,
    ) -> Self {
        PythonTypeAlias {
            name: Cow::Borrowed(name),
            target: target.to_type_ref(db, false),
        }
    }
}

impl<'ir> TryFrom<(&'ir IntermediateRepr, &'_ crate::GeneratorArgs)> for PythonStreamTypes<'ir> {
    type Error = anyhow::Error;

    fn try_from((ir, _): (&'ir IntermediateRepr, &'_ crate::GeneratorArgs)) -> Result<Self> {
        Ok(Self {
            partial_classes: ir
                .walk_classes()
                .map(PartialPythonClass::from)
                .collect::<Vec<_>>(),
        })
    }
}

impl<'ir> From<ClassWalker<'ir>> for PartialPythonClass<'ir> {
    fn from(c: ClassWalker<'ir>) -> PartialPythonClass<'ir> {
        PartialPythonClass {
            name: c.name(),
            dynamic: c.item.attributes.get("dynamic_type").is_some(),
            fields: c
                .item
                .elem
                .static_fields
                .iter()
                .map(|f| {
                    // Fields with @stream.done should take their type from
                    let needed: bool = f.attributes.get("stream.not_null").is_some();
                    let (_, metadata) = c.db.distribute_metadata(&f.elem.r#type.elem);
                    let done: bool = metadata.1.done;
                    let field = match (done, needed) {
                        // A normal partial field.
                        (false, false) => add_default_value(
                            &f.elem.r#type.elem.to_partial_type_ref(c.db, false, false)),
                        // A field with @stream.done and no @stream.not_null
                        (true, false) => add_default_value(
                            &optional(&f.elem.r#type.elem.to_type_ref(c.db, true))
                        ),
                        (false, true) => add_default_value(
                            &f.elem.r#type.elem.to_partial_type_ref(c.db, false, true)),
                        (true, true) => f.elem.r#type.elem.to_type_ref(c.db, true), // TODO: Fix.
                    };
                    (
                        f.elem.name.as_str(),
                        field,
                        f.elem.docstring.as_ref().map(render_docstring),
                    )
                })
                .collect(),
            docstring: c.item.elem.docstring.as_ref().map(render_docstring),
        }
    }
}

/// For a field whose type
pub fn add_default_value(type_str: &String) -> String {
    if type_str.starts_with("Optional[") {
        format!("{} = None", type_str)
    } else {
        type_str.clone()
    }
}

pub fn type_name_for_checks(checks: &TypeCheckAttributes) -> String {
    let check_names = checks
        .0
        .iter()
        .map(|check| format!("\"{check}\""))
        .sorted()
        .join(", ");

    format!["Literal[{check_names}]"]
}

/// Returns the Python `Literal` representation of `self`.
pub fn to_python_literal(literal: &LiteralValue) -> String {
    // Python bools are a little special...
    let value = match literal {
        LiteralValue::Bool(bool) => String::from(match *bool {
            true => "True",
            false => "False",
        }),

        // Rest of types match the fmt::Display impl.
        other => other.to_string(),
    };

    format!("Literal[{value}]")
}

trait ToTypeReferenceInTypeDefinition {
    fn to_type_ref(&self, ir: &IntermediateRepr, module_prefix: bool) -> String;
    fn to_partial_type_ref(&self, ir: &IntermediateRepr, wrapped: bool, needed: bool) -> String;
}

impl ToTypeReferenceInTypeDefinition for FieldType {
    // TODO: use_module_prefix boolean blindness. Replace with str?
    fn to_type_ref(&self, ir: &IntermediateRepr, use_module_prefix: bool) -> String {
        let module_prefix = if use_module_prefix { "types." } else {""};
        match self {
            FieldType::Enum(name) => {
                if ir
                    .find_enum(name)
                    .map(|e| e.item.attributes.get("dynamic_type").is_some())
                    .unwrap_or(false)
                {
                    format!("Union[\"{module_prefix}{name}\", str]")
                } else {
                    format!("\"{module_prefix}{name}\"")
                }
            }
            FieldType::RecursiveTypeAlias(name) => format!("\"{name}\""),
            FieldType::Literal(value) => to_python_literal(value),
            FieldType::Class(name) => format!("\"{module_prefix}{name}\""),
            FieldType::List(inner) => format!("List[{}]", inner.to_type_ref(ir, use_module_prefix)),
            FieldType::Map(key, value) => {
                format!("Dict[{}, {}]", key.to_type_ref(ir, use_module_prefix), value.to_type_ref(ir, use_module_prefix))
            }
            FieldType::Primitive(r#type) => r#type.to_python(),
            FieldType::Union(inner) => format!(
                "Union[{}]",
                inner
                    .iter()
                    .map(|t| t.to_type_ref(ir, use_module_prefix))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            FieldType::Tuple(inner) => format!(
                "Tuple[{}]",
                inner
                    .iter()
                    .map(|t| t.to_type_ref(ir, use_module_prefix))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            FieldType::Optional(inner) => format!("Optional[{}]", inner.to_type_ref(ir, use_module_prefix)),
            FieldType::WithMetadata { base, .. } => match field_type_attributes(self) {
                Some(checks) => {
                    let base_type_ref = base.to_type_ref(ir, use_module_prefix);
                    let checks_type_ref = type_name_for_checks(&checks);
                    format!("Checked[{base_type_ref},{checks_type_ref}]")
                }
                None => base.to_type_ref(ir, use_module_prefix),
            },
        }
    }


    fn to_partial_type_ref(&self, ir: &IntermediateRepr, wrapped: bool, needed: bool) -> String {
        let (base_type, metadata) = ir.distribute_metadata(self);
        let is_partial_type = !metadata.1.done;
        let use_module_prefix = !is_partial_type;
        let with_state = metadata.1.state;
        let constraints = metadata.0;
        let module_prefix = if is_partial_type { "" } else { "types." };
        let base_rep = match base_type {
            FieldType::Class(name) => {
                if wrapped || needed {
                    format!("\"{module_prefix}{name}\"")
                } else {
                    format!("Optional[\"{module_prefix}{name}\"]")
                }
            }
            FieldType::Enum(name) => {
                if ir
                    .find_enum(name)
                    .map(|e| e.item.attributes.get("dynamic_type").is_some())
                    .unwrap_or(false)
                {
                    format!("Optional[Union[types.{name}, str]]")
                } else {
                    if needed {
                        format!("types.{name}")
                    } else {
                      format!("Optional[types.{name}]")
                    }
                }
            }
            FieldType::RecursiveTypeAlias(name) => {
                if wrapped {
                    format!("\"{name}\"")
                } else {
                    format!("Optional[\"{name}\"]")
                }
            }
            FieldType::Literal(value) => format!("Optional[{}]", to_python_literal(value)), // TODO: Handle `needed` here.
            FieldType::List(inner) => format!("List[{}]", inner.to_partial_type_ref(ir, true, false)),
            FieldType::Map(key, value) => {
                format!(
                    "Dict[{}, {}]",
                    key.to_type_ref(ir, use_module_prefix),
                    value.to_partial_type_ref(ir, false, false)
                )
            }
            FieldType::Primitive(r#type) => {
                if needed {
                    r#type.to_python()
                } else {
                format!("Optional[{}]", r#type.to_python())
                }
            },
            FieldType::Union(inner) => {
                let union_contents =
                inner
                    .iter()
                    .map(|t| t.to_partial_type_ref(ir, true, false))
                    .collect::<Vec<_>>()
                    .join(", ");
                if needed {
                    format!("Union[{union_contents}]")
                } else {
                    format!("Optional[Union[{union_contents}]]")
                }
            },
            FieldType::Tuple(inner) => {
                let tuple_contents =
                inner
                    .iter()
                    .map(|t| t.to_partial_type_ref(ir, false, false))
                    .collect::<Vec<_>>()
                    .join(", ");
                if needed { format!("Tuple[{tuple_contents}]") } else { format!("Optional[Tuple[{tuple_contents}]]")
                }
            },
            FieldType::Optional(inner) => inner.to_partial_type_ref(ir, false, false),
            FieldType::WithMetadata{..} => unreachable!("distribute_metadata makes this branch unreachable."),
        };
        let base_type_ref = if is_partial_type {
            base_rep
        } else {
            if needed {
                base_type.to_type_ref(ir, use_module_prefix)
            } else {
                base_rep
            }
        };
        let rep_with_checks = match field_type_attributes(self) {
          Some(checks) => {
            let checks_type_ref = type_name_for_checks(&checks);
            format!("Checked[{base_type_ref},{checks_type_ref}]")
          },
          None => base_type_ref
        };
        let rep_with_stream_state = if with_state {
          stream_state(&rep_with_checks)
        } else {
          rep_with_checks
        };
        rep_with_stream_state
    }
}

/// Render the BAML documentation (a bare string with padding stripped)
/// into a Python docstring. (Indented once and surrounded by """).
fn render_docstring(d: &Docstring) -> String {
    let lines = d.0.as_str().replace("\n", "\n    ");
    format!("\"\"\"{lines}\"\"\"")
}

fn optional(base: &str) -> String {
    format!("Optional[{base}]")
}

fn stream_state(base: &str) -> String {
    format!("StreamState[{base}]")
}

