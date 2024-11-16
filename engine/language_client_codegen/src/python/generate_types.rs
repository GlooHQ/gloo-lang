use anyhow::Result;
use baml_types::LiteralValue;
use itertools::Itertools;
use std::borrow::Cow;

use crate::{field_type_attributes, type_check_attributes, TypeCheckAttributes};

use super::python_language_features::ToPython;
use internal_baml_core::ir::{
    repr::{Docstring, IntermediateRepr}, ClassWalker, EnumWalker, FieldType, IRHelper,
};

#[derive(askama::Template)]
#[template(path = "types.py.j2", escape = "none")]
pub(crate) struct PythonTypes<'ir> {
    enums: Vec<PythonEnum<'ir>>,
    classes: Vec<PythonClass<'ir>>,
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
                .map(|v| (v.0.elem.0.as_str(), v.1.as_ref().map(|d| render_docstring(d))))
                .collect(),
            docstring: e.item.elem.docstring.as_ref().map(|s| render_docstring(s))
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
                            &f.elem.r#type.elem,
                            &f.elem.r#type.elem.to_type_ref(&c.db),
                        ),
                        f.elem.docstring.as_ref().map(|d| render_docstring(d)),
                    )
                })
                .collect(),
            docstring: c.item.elem.docstring.as_ref().map(|d| render_docstring(d)),
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
                    (
                        f.elem.name.as_str(),
                        add_default_value(
                            &f.elem.r#type.elem,
                            &f.elem.r#type.elem.to_partial_type_ref(&c.db, false),
                        ),
                        f.elem.docstring.as_ref().map(|d| render_docstring(d)),
                    )
                })
                .collect(),
            docstring: c.item.elem.docstring.as_ref().map(|d| render_docstring(d)),
        }
    }
}

pub fn add_default_value(node: &FieldType, type_str: &String) -> String {
    if type_str.starts_with("Optional[") {
        return format!("{} = None", type_str);
    } else {
        return type_str.clone();
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
    fn to_type_ref(&self, ir: &IntermediateRepr) -> String;
    fn to_partial_type_ref(&self, ir: &IntermediateRepr, wrapped: bool) -> String;
}

impl ToTypeReferenceInTypeDefinition for FieldType {
    fn to_type_ref(&self, ir: &IntermediateRepr) -> String {
        match self {
            FieldType::Enum(name) => {
                if ir
                    .find_enum(name)
                    .map(|e| e.item.attributes.get("dynamic_type").is_some())
                    .unwrap_or(false)
                {
                    format!("Union[\"{name}\", str]")
                } else {
                    format!("\"{name}\"")
                }
            }
            FieldType::Literal(value) => to_python_literal(value),
            FieldType::Class(name) => format!("\"{name}\""),
            FieldType::List(inner) => format!("List[{}]", inner.to_type_ref(ir)),
            FieldType::Map(key, value) => {
                format!("Dict[{}, {}]", key.to_type_ref(ir), value.to_type_ref(ir))
            }
            FieldType::Primitive(r#type) => r#type.to_python(),
            FieldType::Union(inner) => format!(
                "Union[{}]",
                inner
                    .iter()
                    .map(|t| t.to_type_ref(ir))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            FieldType::Tuple(inner) => format!(
                "Tuple[{}]",
                inner
                    .iter()
                    .map(|t| t.to_type_ref(ir))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            FieldType::Optional(inner) => format!("Optional[{}]", inner.to_type_ref(ir)),
            FieldType::Constrained { base, .. } => match field_type_attributes(self) {
                Some(checks) => {
                    let base_type_ref = base.to_type_ref(ir);
                    let checks_type_ref = type_name_for_checks(&checks);
                    format!("Checked[{base_type_ref},{checks_type_ref}]")
                }
                None => base.to_type_ref(ir),
            },
        }
    }

    fn to_partial_type_ref(&self, ir: &IntermediateRepr, wrapped: bool) -> String {
        match self {
            FieldType::Class(name) => {
                if wrapped {
                    format!("\"{name}\"")
                } else {
                    format!("Optional[\"{name}\"]")
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
                    format!("Optional[types.{name}]")
                }
            }
            FieldType::Literal(value) => to_python_literal(value),
            FieldType::List(inner) => format!("List[{}]", inner.to_partial_type_ref(ir, true)),
            FieldType::Map(key, value) => {
                format!(
                    "Dict[{}, {}]",
                    key.to_type_ref(ir),
                    value.to_partial_type_ref(ir, false)
                )
            }
            FieldType::Primitive(r#type) => format!("Optional[{}]", r#type.to_python()),
            FieldType::Union(inner) => format!(
                "Optional[Union[{}]]",
                inner
                    .iter()
                    .map(|t| t.to_partial_type_ref(ir, true))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            FieldType::Tuple(inner) => format!(
                "Optional[Tuple[{}]]",
                inner
                    .iter()
                    .map(|t| t.to_partial_type_ref(ir, false))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            FieldType::Optional(inner) => inner.to_partial_type_ref(ir, false),
            FieldType::Constrained { base, .. } => {
                let base_type_ref = base.to_partial_type_ref(ir, false);
                match field_type_attributes(self) {
                    Some(checks) => {
                        let base_type_ref = base.to_partial_type_ref(ir, false);
                        let checks_type_ref = type_name_for_checks(&checks);
                        format!("Checked[{base_type_ref},{checks_type_ref}]")
                    }
                    None => base_type_ref,
                }
            }
        }
    }
}

/// Render the BAML documentation (a bare string with padding stripped)
/// into a Python docstring. (Indented once and surrounded by """).
fn render_docstring(d: &Docstring) -> String {
    let lines = d.0.as_str().replace("\n", "\n    ");
    format!("\"\"\"{lines}\"\"\"")
}
