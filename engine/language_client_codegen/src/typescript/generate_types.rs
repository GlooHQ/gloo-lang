use std::borrow::Cow;

use anyhow::Result;
use itertools::Itertools;

use internal_baml_core::ir::{repr::{Docstring, IntermediateRepr}, ClassWalker, EnumWalker};

use crate::{type_check_attributes, GeneratorArgs, TypeCheckAttributes};

use super::ToTypeReferenceInClientDefinition;

#[derive(askama::Template)]
#[template(path = "type_builder.ts.j2", escape = "none")]
pub(crate) struct TypeBuilder<'ir> {
    enums: Vec<TypescriptEnum<'ir>>,
    classes: Vec<TypescriptClass<'ir>>,
}

#[derive(askama::Template)]
#[template(path = "types.ts.j2", escape = "none")]
pub(crate) struct TypescriptTypes<'ir> {
    enums: Vec<TypescriptEnum<'ir>>,
    classes: Vec<TypescriptClass<'ir>>,
}

struct TypescriptEnum<'ir> {
    pub name: &'ir str,
    pub values: Vec<(&'ir str, Option<String>)>,
    pub dynamic: bool,
    pub docstring: Option<String>,
}

pub struct TypescriptClass<'ir> {
    pub name: Cow<'ir, str>,
    pub fields: Vec<(Cow<'ir, str>, bool, String, Option<String>)>,
    pub dynamic: bool,
    pub docstring: Option<String>,
}

impl<'ir> TryFrom<(&'ir IntermediateRepr, &'ir GeneratorArgs)> for TypescriptTypes<'ir> {
    type Error = anyhow::Error;

    fn try_from(
        (ir, _): (&'ir IntermediateRepr, &'ir GeneratorArgs),
    ) -> Result<TypescriptTypes<'ir>> {
        Ok(TypescriptTypes {
            enums: ir
                .walk_enums()
                .map(|e| Into::<TypescriptEnum>::into(&e))
                .collect::<Vec<_>>(),
            classes: ir
                .walk_classes()
                .map(|e| Into::<TypescriptClass>::into(&e))
                .collect::<Vec<_>>(),
        })
    }
}

impl<'ir> TryFrom<(&'ir IntermediateRepr, &'ir GeneratorArgs)> for TypeBuilder<'ir> {
    type Error = anyhow::Error;

    fn try_from((ir, _): (&'ir IntermediateRepr, &'ir GeneratorArgs)) -> Result<TypeBuilder<'ir>> {
        Ok(TypeBuilder {
            enums: ir
                .walk_enums()
                .map(|e| Into::<TypescriptEnum>::into(&e))
                .collect::<Vec<_>>(),
            classes: ir
                .walk_classes()
                .map(|e| Into::<TypescriptClass>::into(&e))
                .collect::<Vec<_>>(),
        })
    }
}

impl<'ir> From<&EnumWalker<'ir>> for TypescriptEnum<'ir> {
    fn from(e: &EnumWalker<'ir>) -> TypescriptEnum<'ir> {
        TypescriptEnum {
            name: e.name(),
            dynamic: e.item.attributes.get("dynamic_type").is_some(),
            values: e
                .item
                .elem
                .values
                .iter()
                .map(|v| (v.0.elem.0.as_str(), v.1.as_ref().map(|s| render_docstring(s, true))))
                .collect(),
            docstring: e.item.elem.docstring.as_ref().map(|d| render_docstring(d, false)),
        }
    }
}

impl<'ir> From<&ClassWalker<'ir>> for TypescriptClass<'ir> {
    fn from(c: &ClassWalker<'ir>) -> TypescriptClass<'ir> {
        TypescriptClass {
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
                        f.elem.r#type.elem.is_optional(),
                        f.elem.r#type.elem.to_type_ref(&c.db),
                        f.elem.docstring.as_ref().map(|d| render_docstring(d, true)),
                    )
                })
                .collect(),
            docstring: c.item.elem.docstring.as_ref().map(|d| render_docstring(d, false)),
        }
    }
}

pub fn type_name_for_checks(checks: &TypeCheckAttributes) -> String {
    checks.0.iter().map(|check| format!("\"{check}\"")).sorted().join(" | ")
}

/// Render the BAML documentation (a bare string with padding stripped)
/// into a TS docstring.
/// (Optionally indented and formatted as a TS block comment).
fn render_docstring(d: &Docstring, indented: bool) -> String {
    if indented {
        let lines = d.0.as_str().replace("\n", "\n   * ");
        format!("/**\n   * {lines}\n   */")
    } else {
        let lines = d.0.as_str().replace("\n", "\n * ");
        format!("/**\n * {lines}\n */")
    }
}
