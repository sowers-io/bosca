use crate::context::Context;
use crate::model::{ClassModel, ClassType, FieldModel, FieldType};
use std::sync::Arc;

pub fn generate(context: &Context, models: &Vec<Arc<ClassModel>>) {
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
            println!("export enum {} {{", model.name.replace('.', "_"));
            if let Some(enum_values) = model.get_enum_values() {
                for enum_value in enum_values {
                    print!("  {} = \"{}\",", enum_value, enum_value);
                    println!();
                }
            }
            println!("}}")
        } else if model.class_type == ClassType::Interface || model.class_type == ClassType::Class {
            print!("export interface {}", model.name.replace('.', "_"));

            let ifaces = context.get_class_interfaces(&model.name);
            if !ifaces.is_empty() {
                print!(" extends ");
                for (i, iface) in ifaces.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    print!("{}", iface.name.replace('.', "_"));
                }
            }

            println!(" {{");
            if !context.is_class_interface(model.name.as_str()) {
                println!("  __typename?: \"{}\"", model.type_name);
            } else if model.class_type == ClassType::Interface {
                println!("  __typename?: string | null");
            }
            if let Some(fields) = model.get_fields() {
                for field in fields {
                    print!("  {}", field.name);
                    if field.nullable {
                        print!("?")
                    }
                    print!(": ");
                    field_type(model, &field, &field.field_type);
                    if field.nullable {
                        print!(" | null")
                    }
                    println!()
                }
            }
            println!("}}")
        }
    }
}

fn field_type(model: &ClassModel, field: &FieldModel, ftype: &FieldType) {
    match ftype {
        FieldType::Unknown => {
            panic!("unknown field type: {}.{}", model.name, field.name)
        }
        FieldType::List => {
            if field.field_type_references.is_empty() {
                if field.field_type_scalar != FieldType::Unknown {
                    field_type(model, field, &field.field_type_scalar);
                } else {
                    panic!(
                        "field must have a type reference: {}.{}",
                        model.name, field.name
                    );
                }
                print!("[]")
            } else {
                print!("{}", field.field_type_references[0].name);
                print!("[]")
            }
        }
        FieldType::Double | FieldType::Int | FieldType::Float => {
            print!("number")
        }
        FieldType::String => {
            print!("string")
        }
        FieldType::Json => {
            print!("any")
        }
        FieldType::Boolean => {
            print!("boolean")
        }
        FieldType::DateTime => {
            print!("Date")
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
            print!("{}", field.field_type_references[0].name.replace('.', "_"));
        }
    }
}
