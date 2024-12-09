use anyhow::Result;
use indexmap::IndexMap;
use itertools::join;

use baml_types::{BamlValueWithMeta, BamlValue, FieldType, TypeValue, Constraint};
use internal_baml_core::ir::repr::IntermediateRepr;
use internal_baml_core::ir::IRHelper;
use crate::runtime_wasm::{WasmInputForm, WasmInputField, WasmInputItem};

/// Derive an HTML form from 
// TODO: Return UIInput, and let typescript handle the rendering
// into actual html.
pub fn form_from_input(ir: &IntermediateRepr, field_types: &Vec<(String, FieldType)>, arguments: IndexMap<String, BamlValue>) -> Result<WasmInputForm> {
    let form_items = field_types.iter().map(|(arg_name, arg_type)| {
        let argument = arguments.get(arg_name);
        let item = form_item(ir, arg_name, arg_type, argument)?;
        Ok( WasmInputFormArgument { arg_name, item })
    }).collect::<Result<Vec<_>>>()?;
    Ok(WasmInputForm {
        items: form_items
    })
}


// fn form_arg(
//     ir: &IntermediateRepr,
//     arg_name: &str,
//     arg_type: &FieldType,
//     argument: Option<&BamlValue>
// ) -> Result<WasmInputItem> {
//     
// }

/// The html of a single argument.
/// `arg_name` is only present for top-level arguments. For arguments
/// that are fields of a class, the "arg_name" should be the class name.
fn form_item(
    ir: &IntermediateRepr,
    field_name: Option<&str>,
    field_type: &FieldType,
    argument: Option<&BamlValue>
) -> Result<WasmInputItem> {

    let (r#type, constraints) = ir.distribute_constraints(arg_type);
    let arg_str = argument.map(|baml_value| baml_value.to_string());
    match r#type {
        FieldType::Primitive(TypeValue::Int) => Ok(input_el(arg_name, "number", arg_str)),
        FieldType::Primitive(TypeValue::String) => Ok(input_el(arg_name, "text", arg_str)),
        FieldType::Primitive(TypeValue::Bool) => Ok(input_el(arg_name, "checkbox", arg_str)),
        FieldType::Class(class_name) => {
            let class_field_items = ir.find_class(name)?.walk_fields().map(|field| {
                let field_arg = match argument {
                    Some(BamlValue::Class(_name, fields)) => fields.get(arg_name),
                    _ => None,
                };
                form_item(ir, field.name(), field.r#type(), field_arg)
            }).collect::<Result<Vec<_>>>()?;
            Ok(WasmInputItem {
                r#type: WasmInputItemType::Class,
                name: name.to_string(),
                fields: class_field_items,
            })
        },
        _ => todo!(),
    }
}

fn input_el(name: &str, r#type: &str, default: Option<String>) -> WasmInputItem {
    WasmInputItem {
        name: name.to_string(),
        field: Some( WasmInputField {
            r#type: r#type.to_string(),
            default: default,
        }),
        subform: Vec::new()
    }
}