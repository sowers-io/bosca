use std::fmt::format;
use crate::context::Context;
use crate::model::{ClassModel, ClassType, FieldModel, FieldType};
use std::io::Write;
use std::sync::Arc;

pub fn generate(context: &Context, writer: &mut impl Write) {
    writer
        .write_all("#![allow(unused_imports)]\r\n".as_bytes())
        .unwrap();
    writer
        .write_all("#![allow(non_camel_case_types)]\r\n".as_bytes())
        .unwrap();
    writer
        .write_all("#![allow(clippy::wrong_self_convention)]\r\n".as_bytes())
        .unwrap();
    writer
        .write_all("use chrono::{DateTime, Utc};\r\n".as_bytes())
        .unwrap();
    writer
        .write_all("use serde::{Serialize, Deserialize};\r\n".as_bytes())
        .unwrap();
    writer
        .write_all("use serde_json::Value;\r\n\r\n".as_bytes())
        .unwrap();

    let models = context.get_class_models();
    for model in models {
        if model.name.starts_with("__")
            || model.name.starts_with("I__")
            || model.name == "IJSON"
            || model.name == "JSON"
            || model.name == "IString"
            || model.name == "String"
            || model.name == "IDateTime"
            || model.name == "DateTime"
            || model.name == "IFloat"
            || model.name == "Float"
            || model.name == "ILong"
            || model.name == "Long"
            || model.name == "IInt"
            || model.name == "Int"
            || model.name == "IBoolean"
            || model.name == "Boolean"
            || model.name == "IID"
            || model.name == "ID"
            || model.name == "IUpload"
            || model.name == "Upload"
        {
            continue;
        }
        if model.class_type == ClassType::Interface {
            writer
                .write_all("#[derive(Serialize, Deserialize)]\r\n".as_bytes())
                .unwrap();
            writer
                .write_all(format!("pub enum {} {{\r\n", model.name.replace('.', "_")).as_bytes())
                .unwrap();
            interface_enum(context, &model, writer);
            writer.write_all("}\r\n".as_bytes()).unwrap();
            writer.write_all(format!("impl {} {{\r\n", model.name.replace('.', "_")).as_bytes());
            if let Some(fields) = model.get_fields() {
                for field in fields {
                    interface_enum_field(context, &model, &field, writer);
                }
            }
            writer.write_all("}\r\n".as_bytes()).unwrap();
        } else if model.class_type == ClassType::Enum {
            writer
                .write_all("#[derive(Serialize, Deserialize)]\r\n".as_bytes())
                .unwrap();
            writer
                .write_all(format!("pub enum {} {{\r\n", model.name.replace('.', "_")).as_bytes())
                .unwrap();
            if let Some(enum_values) = model.get_enum_values() {
                for enum_value in enum_values {
                    writer
                        .write_all(format!("  {},", enum_value).as_bytes())
                        .unwrap();
                    writer.write_all("\r\n".as_bytes()).unwrap();
                }
            }
            writer.write_all("}\r\n".as_bytes()).unwrap();
        } else if model.class_type == ClassType::Class {
            writer
                .write_all("#[derive(Serialize, Deserialize)]\r\n".as_bytes())
                .unwrap();
            writer
                .write_all(format!("pub struct {} {{\r\n", model.name.replace('.', "_")).as_bytes())
                .unwrap();
            if let Some(fields) = model.get_fields() {
                for field in fields {
                    writer.write_all("  ".as_bytes()).unwrap();
                    field_name(&field, writer);
                    writer.write_all(": ".as_bytes()).unwrap();
                    if field.nullable {
                        writer.write_all("Option<".as_bytes()).unwrap();
                    }
                    field_type(context, &model, &field, &field.field_type, writer);
                    if field.nullable {
                        writer.write_all(">".as_bytes()).unwrap();
                    }
                    writer.write_all(",\r\n".as_bytes()).unwrap();
                }
            }
            writer.write_all("}\r\n".as_bytes()).unwrap();
        } else if model.class_type == ClassType::Scalar {
            panic!("scalar type not supported: {}", model.name);
        }
    }
}

fn field_name(field: &FieldModel, writer: &mut impl Write) {
    let mut name = field.name.to_owned();
    let mut has_upper_case = true;
    while has_upper_case {
        has_upper_case = false;
        for (i, char) in name.chars().enumerate() {
            if i > 0 && char.is_ascii_uppercase() {
                name.replace_range(
                    i..i + 1,
                    &format!("_{}", char.to_ascii_lowercase()),
                );
                has_upper_case = true;
                break;
            }
        }
    }
    match name.as_str() {
        "type" | "trait" => writer
            .write_all(format!("{}_", name).as_bytes())
            .unwrap(),
        _ => writer
            .write_all(format!("{}", name).as_bytes())
            .unwrap(),
    }
}

fn interface_enum(context: &Context, model: &Arc<ClassModel>, writer: &mut impl Write) {
    let ifaces = context.get_interface_implementations(&model.name);
    for iface in ifaces {
        if let Some(m) = iface.get_model() {
            if m.class_type == ClassType::Interface {
                interface_enum(context, &m, writer);
            } else {
                writer
                    .write_all(format!("  {}(", m.name.replace('.', "_")).as_bytes())
                    .unwrap();
                writer
                    .write_all(m.name.replace('.', "_").as_bytes())
                    .unwrap();
                writer.write_all("),\r\n".as_bytes()).unwrap();
            }
        }
    }
}

fn interface_enum_field(context: &Context, model: &Arc<ClassModel>, field: &FieldModel, writer: &mut impl Write) {
    writer.write_all("  fn ".to_string().as_bytes()).unwrap();
    field_name(&field, writer);
    writer.write_all("(&self) -> &".as_bytes()).unwrap();
    if field.nullable {
        writer.write_all("Option<".as_bytes()).unwrap();
    }
    field_type(context, model, field, &field.field_type, writer);
    if field.nullable {
        writer.write_all(">".as_bytes()).unwrap();
    }
    writer.write_all(" {\r\n".as_bytes()).unwrap();
    let ifaces = context.get_interface_implementations(&model.name);
    writer.write_all("    match self {\r\n".as_bytes()).unwrap();
    for iface in ifaces {
        if let Some(m) = iface.get_model() {
            writer.write_all(format!("      {}::{}(m) => &m.", model.name.replace(".", "_"), m.name.replace(".", "_")).to_string().as_bytes()).unwrap();
            field_name(&field, writer);
            writer.write_all(",\r\n".as_bytes()).unwrap();
        }
    }
    writer.write_all("    }\r\n".as_bytes()).unwrap();
    writer.write_all("  }\r\n".as_bytes()).unwrap();
}

fn field_type_struct(model: &ClassModel, field: &FieldModel, writer: &mut impl Write) {
    if field.field_type_references.is_empty() {
        panic!(
            "field must have a type reference: {}.{}",
            model.name, field.name
        );
    }
    if let Some(model) = field.field_type_references[0].get_model() {
        writer
            .write_all(
                model.name
                    .replace('.', "_")
                    .as_bytes(),
            )
            .unwrap();
    } else {
        panic!(
            "field must have a type reference with a model: {}.{}",
            model.name, field.name
        );
    }
}

fn field_type(
    context: &Context,
    model: &ClassModel,
    field: &FieldModel,
    ftype: &FieldType,
    writer: &mut impl Write,
) {
    match ftype {
        FieldType::Unknown => {
            panic!("unknown field type: {}.{}", model.name, field.name)
        }
        FieldType::List => {
            writer.write_all("Vec<".as_bytes()).unwrap();
            if field.field_type_references.is_empty() {
                if field.field_type_scalar != FieldType::Unknown {
                    field_type(
                        context,
                        model,
                        field,
                        &field.field_type_scalar,
                        writer,
                    );
                } else {
                    panic!(
                        "field must have a type reference: {}.{}",
                        model.name, field.name
                    );
                }
            } else {
                field_type_struct(model, field, writer);
            }
            writer.write_all(">".as_bytes()).unwrap();
        }
        FieldType::Double => writer.write_all("f64".as_bytes()).unwrap(),
        FieldType::Float => writer.write_all("f32".as_bytes()).unwrap(),
        FieldType::Long => writer.write_all("i64".as_bytes()).unwrap(),
        FieldType::Int => writer.write_all("i32".as_bytes()).unwrap(),
        FieldType::String => writer.write_all("String".as_bytes()).unwrap(),
        FieldType::Json => writer.write_all("Value".as_bytes()).unwrap(),
        FieldType::Boolean => writer.write_all("bool".as_bytes()).unwrap(),
        FieldType::DateTime => writer.write_all("DateTime<Utc>".as_bytes()).unwrap(),
        FieldType::Union => field_type_struct(model, field, writer),
        FieldType::Object | FieldType::Interface | FieldType::Enum => field_type_struct(model, field, writer),
    }
}
