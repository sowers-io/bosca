use std::io::Write;
use crate::context::Context;
use crate::model::{ClassModel, ClassType, FieldModel, FieldType};
use std::sync::Arc;

pub fn generate(context: &Context, models: &Vec<Arc<ClassModel>>, writer: &mut impl Write) {
    for model in models {
        if model.name.starts_with("__")
            || model.name.starts_with("I__")
            || model.name == "IJSON"
            || model.name == "JSON"
            || model.name == "String"
            || model.name == "IString"
            || model.name == "DateTime"
        {
            continue;
        }
        if model.class_type == ClassType::Enum {
            writer.write_all(format!("export enum {} {{", model.name.replace('.', "_")).as_bytes()).unwrap();
            if let Some(enum_values) = model.get_enum_values() {
                for enum_value in enum_values {
                    writer.write_all(format!("  {} = \"{}\",", enum_value, enum_value).as_bytes()).unwrap();
                    writer.write_all("\r\n".as_bytes()).unwrap();
                }
            }
            writer.write_all("}\r\n".as_bytes()).unwrap();
        } else if model.class_type == ClassType::Interface || model.class_type == ClassType::Class {
            writer.write_all(format!("export interface {}", model.name.replace('.', "_")).as_bytes()).unwrap();

            let ifaces = context.get_class_interfaces(&model.name);
            if !ifaces.is_empty() {
                writer.write_all(" extends ".as_bytes()).unwrap();
                for (i, iface) in ifaces.iter().enumerate() {
                    if i > 0 {
                        writer.write_all(", ".as_bytes()).unwrap();
                    }
                    writer.write_all(iface.name.replace('.', "_").as_bytes()).unwrap();
                }
            }
            writer.write_all(" {\r\n".as_bytes()).unwrap();
            if !context.is_class_interface(model.name.as_str()) {
                writer.write_all(format!("  __typename?: \"{}\"", model.type_name).as_bytes()).unwrap();
            } else if model.class_type == ClassType::Interface {
                writer.write_all("  __typename?: string | null".as_bytes()).unwrap();
            }
            writer.write_all("\r\n".as_bytes()).unwrap();
            if let Some(fields) = model.get_fields() {
                for field in fields {
                    writer.write_all(format!("  {}", field.name).as_bytes()).unwrap();
                    if field.nullable {
                        writer.write_all("?".as_bytes()).unwrap();
                    }
                    writer.write_all(": ".as_bytes()).unwrap();
                    field_type(model, &field, &field.field_type, writer);
                    if field.nullable {
                        writer.write_all(" | null".as_bytes()).unwrap();
                    }
                    writer.write_all("\r\n".as_bytes()).unwrap();
                }
            }
            writer.write_all("}\r\n".as_bytes()).unwrap();
        }
    }
}

fn field_type(model: &ClassModel, field: &FieldModel, ftype: &FieldType, writer: &mut impl Write) {
    match ftype {
        FieldType::Unknown => {
            panic!("unknown field type: {}.{}", model.name, field.name)
        }
        FieldType::List => {
            if field.field_type_references.is_empty() {
                if field.field_type_scalar != FieldType::Unknown {
                    field_type(model, field, &field.field_type_scalar, writer);
                } else {
                    panic!(
                        "field must have a type reference: {}.{}",
                        model.name, field.name
                    );
                }
                writer.write_all("[]".as_bytes()).unwrap();
            } else {
                writer.write_all(field.field_type_references[0].name.replace('.', "_").as_bytes()).unwrap();
                writer.write_all("[]".as_bytes()).unwrap();
            }
        }
        FieldType::Double | FieldType::Int | FieldType::Float => {
            writer.write_all("number".as_bytes()).unwrap()
        }
        FieldType::String => {
            writer.write_all("string".as_bytes()).unwrap()
        }
        FieldType::Json => {
            writer.write_all("any".as_bytes()).unwrap()
        }
        FieldType::Boolean => {
            writer.write_all("boolean".as_bytes()).unwrap()
        }
        FieldType::DateTime => {
            writer.write_all("Date".as_bytes()).unwrap()
        }
        FieldType::Union
        | FieldType::Object
        | FieldType::Interface
        | FieldType::Enum => {
            if field.field_type_references.is_empty() {
                panic!(
                    "field must have a type reference: {}.{}",
                    model.name, field.name
                );
            }
            writer.write_all(field.field_type_references[0].name.replace('.', "_").as_bytes()).unwrap()
        }
    }
}
