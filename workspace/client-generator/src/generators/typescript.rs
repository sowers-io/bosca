use std::sync::Arc;
use crate::context::Context;
use crate::model::{ClassModel, FieldType};

pub fn generate(context: &Context, models: &Vec<Arc<ClassModel>>) {
    for model in models {
        if model.name.starts_with("__") || model.name == "BaseJSON" || model.name == "JSON" {
            continue;
        }
        if model.enum_values.is_some() && !model.enum_values.as_ref().unwrap().is_empty() {
            println!("export enum {} {{", model.name);
            for enum_value in model.enum_values.as_ref().unwrap().iter() {
                print!("  {} = \"{}\",", enum_value, enum_value);
                println!();
            }
            println!("}}")
        } else if model.fields.is_some() {
            print!("export interface {}", model.name);

            let ifaces = context.get_class_interfaces(&model.name);
            if !ifaces.is_empty() {
                print!(" extends ");
                for (i, iface) in ifaces.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    print!("{}", iface.name);
                }
            }

            println!(" {{");
            println!("  __typename?: \"{}\"", model.type_name);
            for field in model.fields.as_ref().unwrap() {
                print!("  {}", field.name);
                if field.nullable {
                    print!("?")
                }
                print!(": ");
                match field.field_type {
                    FieldType::Unknown => panic!("unknown field type: {}.{}", model.name, field.name),
                    FieldType::List => {
                        if field.field_type_references.is_empty() {
                            panic!("field must have a type reference: {}.{}", model.name, field.name);
                        }
                        print!("{}", field.field_type_references[0].name);
                        print!("[]")
                    }
                    FieldType::Map => {}
                    FieldType::Double | FieldType::Int | FieldType::Float => {
                        print!("number")
                    }
                    FieldType::String => {
                        print!("string")
                    }
                    FieldType::JSON => {
                        print!("any")
                    }
                    FieldType::Boolean => {
                        print!("boolean")
                    }
                    FieldType::Date | FieldType::DateTime => {
                        print!("Date")
                    }
                    FieldType::Union | FieldType::Object | FieldType::Interface | FieldType::Enum => {
                        if field.field_type_references.is_empty() {
                            panic!("field must have a type reference: {}.{}", model.name, field.name);
                        }
                        print!("{}", field.field_type_references[0].name);
                    }
                }
                if field.nullable {
                    print!(" | null")
                }
                println!()
            }
            println!("}}")
        }
    }
}