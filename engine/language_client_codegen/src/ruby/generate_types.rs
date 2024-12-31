use std::borrow::Cow;
use std::collections::HashSet;

use anyhow::Result;
use baml_types::LiteralValue;
use itertools::Itertools;

use crate::{field_type_attributes, type_check_attributes, TypeCheckAttributes};

use super::ruby_language_features::ToRuby;
use internal_baml_core::ir::{
    repr::{Docstring, IntermediateRepr},
    ClassWalker, EnumWalker, FieldType,
};

#[derive(askama::Template)]
#[template(path = "types.rb.j2", escape = "none")]
pub(crate) struct RubyTypes<'ir> {
    enums: Vec<RubyEnum<'ir>>,
    classes: Vec<RubyStruct<'ir>>,
}

struct RubyEnum<'ir> {
    pub name: &'ir str,
    pub values: Vec<&'ir str>,
    dynamic: bool,
    docstring: Option<String>,
}

struct RubyStruct<'ir> {
    name: Cow<'ir, str>,
    fields: Vec<(Cow<'ir, str>, String, Option<String>)>,
    dynamic: bool,
    docstring: Option<String>,
}

#[derive(askama::Template)]
#[template(path = "partial-types.rb.j2", escape = "none")]
pub(crate) struct RubyStreamTypes<'ir> {
    partial_classes: Vec<PartialRubyStruct<'ir>>,
}

/// The Python class corresponding to Partial<TypeDefinedjInBaml>
struct PartialRubyStruct<'ir> {
    name: &'ir str,
    // the name, type and docstring of the field
    fields: Vec<(&'ir str, String, Option<String>)>,
    docstring: Option<String>,
}

#[derive(askama::Template)]
#[template(path = "type-registry.rb.j2", escape = "none")]
pub(crate) struct TypeRegistry<'ir> {
    enums: Vec<RubyEnum<'ir>>,
    classes: Vec<RubyStruct<'ir>>,
}

impl<'ir> TryFrom<(&'ir IntermediateRepr, &'ir crate::GeneratorArgs)> for RubyTypes<'ir> {
    type Error = anyhow::Error;

    fn try_from((ir, _): (&'ir IntermediateRepr, &'ir crate::GeneratorArgs)) -> Result<Self> {
        Ok(RubyTypes {
            enums: ir.walk_enums().map(|e| e.into()).collect(),
            classes: ir.walk_classes().map(|c| c.into()).collect(),
        })
    }
}

impl<'ir> From<EnumWalker<'ir>> for RubyEnum<'ir> {
    fn from(e: EnumWalker<'ir>) -> RubyEnum<'ir> {
        RubyEnum {
            name: e.name(),
            dynamic: e.item.attributes.get("dynamic_type").is_some(),
            values: e
                .item
                .elem
                .values
                .iter()
                .map(|v| v.0.elem.0.as_str())
                .collect(),
            docstring: e
                .item
                .elem
                .docstring
                .as_ref()
                .map(|d| render_docstring(d, true)),
        }
    }
}

impl<'ir> From<ClassWalker<'ir>> for RubyStruct<'ir> {
    fn from(c: ClassWalker<'ir>) -> RubyStruct<'ir> {
        RubyStruct {
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
                        f.elem.r#type.elem.to_type_ref(),
                        f.elem.docstring.as_ref().map(|d| render_docstring(d, true)),
                    )
                })
                .collect(),
            docstring: c
                .item
                .elem
                .docstring
                .as_ref()
                .map(|d| render_docstring(d, false)),
        }
    }
}

impl<'ir> TryFrom<(&'ir IntermediateRepr, &'ir crate::GeneratorArgs)> for RubyStreamTypes<'ir> {
    type Error = anyhow::Error;

    fn try_from((ir, _): (&'ir IntermediateRepr, &'ir crate::GeneratorArgs)) -> Result<Self> {
        Ok(RubyStreamTypes {
            partial_classes: ir.walk_classes().map(|c| c.into()).collect(),
        })
    }
}

impl<'ir> From<ClassWalker<'ir>> for PartialRubyStruct<'ir> {
    fn from(c: ClassWalker<'ir>) -> PartialRubyStruct<'ir> {
        PartialRubyStruct {
            name: c.name(),
            fields: c
                .item
                .elem
                .static_fields
                .iter()
                .map(|f| {
                    (
                        f.elem.name.as_str(),
                        f.elem.r#type.elem.to_partial_type_ref(),
                        f.elem.docstring.as_ref().map(|d| render_docstring(d, true)),
                    )
                })
                .collect(),
            docstring: c
                .item
                .elem
                .docstring
                .as_ref()
                .map(|d| render_docstring(d, false)),
        }
    }
}

pub(super) trait ToTypeReferenceInTypeDefinition {
    fn to_type_ref(&self) -> String;
    fn to_partial_type_ref(&self) -> String;
}

impl ToTypeReferenceInTypeDefinition for FieldType {
    fn to_type_ref(&self) -> String {
        use ToRuby;
        self.to_ruby()
    }

    fn to_partial_type_ref(&self) -> String {
        match self {
            FieldType::Class(name) => format!("Baml::PartialTypes::{}", name.clone()),
            FieldType::Enum(name) => format!("T.nilable(Baml::Types::{})", name.clone()),
            // TODO: Can we define recursive aliases in Ruby with Sorbet?
            FieldType::RecursiveTypeAlias(_name) => "T.anything".to_string(),
            // TODO: Temporary solution until we figure out Ruby literals.
            FieldType::Literal(value) => value.literal_base_type().to_partial_type_ref(),
            // https://sorbet.org/docs/stdlib-generics
            FieldType::List(inner) => format!("T::Array[{}]", inner.to_partial_type_ref()),
            FieldType::Map(key, value) => format!(
                "T::Hash[{}, {}]",
                match key.as_ref() {
                    // For enums just default to strings.
                    FieldType::Enum(_)
                    | FieldType::Literal(LiteralValue::String(_))
                    | FieldType::Union(_) => FieldType::string().to_type_ref(),
                    _ => key.to_type_ref(),
                },
                value.to_partial_type_ref()
            ),
            FieldType::Primitive(_) => format!("T.nilable({})", self.to_type_ref()),
            FieldType::Union(inner) => format!(
                // https://sorbet.org/docs/union-types
                "T.nilable(T.any({}))",
                inner
                    .iter()
                    .map(|t| t.to_partial_type_ref())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            FieldType::Tuple(inner) => format!(
                // https://sorbet.org/docs/tuples
                "T.nilable([{}])",
                inner
                    .iter()
                    .map(|t| t.to_partial_type_ref())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            FieldType::Optional(inner) => inner.to_partial_type_ref(),
            FieldType::Constrained { base, .. } => match field_type_attributes(self) {
                Some(checks) => {
                    let base_type_ref = base.to_partial_type_ref();
                    format!("Baml::Checked[{base_type_ref}]")
                }
                None => base.to_partial_type_ref(),
            },
        }
    }
}

impl<'ir> TryFrom<(&'ir IntermediateRepr, &'_ crate::GeneratorArgs)> for TypeRegistry<'ir> {
    type Error = anyhow::Error;

    fn try_from(
        (ir, _): (&'ir IntermediateRepr, &'_ crate::GeneratorArgs),
    ) -> Result<TypeRegistry<'ir>> {
        Ok(TypeRegistry {
            enums: ir.walk_enums().map(RubyEnum::from).collect::<Vec<_>>(),
            classes: ir.walk_classes().map(RubyStruct::from).collect::<Vec<_>>(),
        })
    }
}

/// Render the BAML documentation (a bare string with padding stripped)
/// into a Ruby docstring.
fn render_docstring(d: &Docstring, indented: bool) -> String {
    if indented {
        let lines = d.0.as_str().replace("\n", "\n      # ");
        format!("# {lines}")
    } else {
        let lines = d.0.as_str().replace("\n", "\n    # ");
        format!("# {lines}")
    }
}
