use std::io::Write;
use crate::context::Context;
use crate::model::{ClassModel, ClassType, FieldModel, FieldType};
use std::sync::Arc;
use crate::introspection::Kind;
use crate::parser::{Document, DocumentType};

pub fn generate_client(_: &Context, documents: &Vec<Document>, writer: &mut impl Write) {
    writer.write_all("// noinspection JSUnusedGlobalSymbols\r\n\r\n".as_bytes()).unwrap();
    writer.write_all("import { BaseClient } from '@bosca/graphql-client-base'\r\n".as_bytes()).unwrap();
    writer.write_all("import { ".as_bytes()).unwrap();
    let mut first = true;
    for document in documents {
        if document.input_object.is_none() || document.output_object.is_none() {
            continue;
        }
        if !first {
            writer.write_all(", ".as_bytes()).unwrap();
        } else {
            first = false;
        }
        writer.write_all(document.input_object.as_ref().unwrap().name.replace('.', "_").as_bytes()).unwrap();
        writer.write_all(", ".as_bytes()).unwrap();
        writer.write_all(document.output_object.as_ref().unwrap().name.replace('.', "_").as_bytes()).unwrap();
    }
    writer.write_all(" } from './models'\r\n\r\n".as_bytes()).unwrap();
    writer.write_all("export class Client extends BaseClient {\r\n".to_string().as_bytes()).unwrap();
    for document in documents {
        if document.input_object.is_none() || document.output_object.is_none() {
            continue;
        }
        writer.write_all("  async ".as_bytes()).unwrap();
        writer.write_all(document.name.as_bytes()).unwrap();
        writer.write_all("(".as_bytes()).unwrap();
        writer.write_all("variables: ".as_bytes()).unwrap();
        writer.write_all(document.input_object.as_ref().unwrap().name.as_bytes()).unwrap();
        writer.write_all("): Promise<".as_bytes()).unwrap();
        writer.write_all(document.output_object.as_ref().unwrap().name.replace(".", "_").as_bytes()).unwrap();
        writer.write_all("> {\r\n".as_bytes()).unwrap();
        writer.write_all("    const extensions = {\r\n".as_bytes()).unwrap();
        writer.write_all("      persistedQuery: {\r\n".as_bytes()).unwrap();
        writer.write_all("        version: 1,\r\n".as_bytes()).unwrap();
        writer.write_all(format!("        sha256Hash: '{}',\r\n", document.sha256).as_bytes()).unwrap();
        writer.write_all("      },\r\n".as_bytes()).unwrap();
        writer.write_all("    }\r\n".as_bytes()).unwrap();
        writer.write_all("    const data = {\r\n".as_bytes()).unwrap();
        writer.write_all("      extensions,\r\n".as_bytes()).unwrap();
        writer.write_all("      variables,\r\n".as_bytes()).unwrap();
        writer.write_all("    }\r\n".as_bytes()).unwrap();
        writer.write_all("    return await this.execute<".as_bytes()).unwrap();
        writer.write_all(document.output_object.as_ref().unwrap().name.replace(".", "_").as_bytes()).unwrap();
        writer.write_all(format!(">(data, {})\r\n", document.document_type.unwrap() == DocumentType::Mutation).as_bytes()).unwrap();
        writer.write_all("  }\r\n".as_bytes()).unwrap();
    }
    writer.write_all("}\r\n".as_bytes()).unwrap();
}

pub fn generate(context: &Context, writer: &mut impl Write) {
    writer.write_all("// noinspection JSUnusedGlobalSymbols\r\n\r\n".as_bytes()).unwrap();
    for model in context.get_class_models() {
        if model.is_internal() {
            continue;
        }
        if model.class_type == ClassType::Enum {
            writer.write_all(format!("export enum {} {{\r\n", model.name.replace('.', "_")).as_bytes()).unwrap();
            if let Some(enum_values) = model.get_enum_values() {
                for enum_value in enum_values {
                    writer.write_all(format!("  {} = '{}',", enum_value, enum_value).as_bytes()).unwrap();
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
            writer.write_all(" {".as_bytes()).unwrap();
            if model.source_kind != Kind::InputObject {
                writer.write_all("\r\n".as_bytes()).unwrap();
                if !context.is_class_interface(model.name.as_str()) {
                    writer.write_all(format!("  __typename?: '{}'", model.type_name).as_bytes()).unwrap();
                } else if model.class_type == ClassType::Interface {
                    writer.write_all("  __typename?: string | null".as_bytes()).unwrap();
                }
            }
            writer.write_all("\r\n".as_bytes()).unwrap();
            if let Some(fields) = model.get_fields() {
                for field in fields {
                    writer.write_all(format!("  {}", field.name).as_bytes()).unwrap();
                    if field.nullable {
                        writer.write_all("?".as_bytes()).unwrap();
                    }
                    writer.write_all(": ".as_bytes()).unwrap();
                    field_type(&model, &field, &field.field_type, writer);
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

fn field_type(model: &Arc<ClassModel>, field: &FieldModel, ftype: &FieldType, writer: &mut impl Write) {
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
        FieldType::Double | FieldType::Float | FieldType::Long | FieldType::Int => {
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
            let m = field.field_type_references[0].get_model().unwrap();
            if m.is_internal() {
                match m.name.as_str() { 
                    "String" => writer.write_all("string".as_bytes()).unwrap(),
                    "JSON" => writer.write_all("any".as_bytes()).unwrap(),
                    "Boolean" => writer.write_all("boolean".as_bytes()).unwrap(),
                    "Int" | "Long" | "Float" | "Double" => writer.write_all("number".as_bytes()).unwrap(),
                    _ => panic!("unknown field type: {}.{} -> {}", model.name, field.name, m.name),
                }   
            } else {
                writer.write_all(m.name.replace('.', "_").as_bytes()).unwrap()
            }
        }
    }
}
