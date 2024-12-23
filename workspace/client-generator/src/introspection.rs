use crate::context::Context;
use crate::model::{ClassModel, FieldModel, FieldType};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Kind {
    #[serde(rename = "NON_NULL")]
    NonNull,
    #[serde(rename = "NULL")]
    Null,
    #[serde(rename = "OBJECT")]
    Object,
    #[serde(rename = "INPUT_OBJECT")]
    InputObject,
    #[serde(rename = "SCALAR")]
    Scalar,
    #[serde(rename = "ENUM")]
    Enum,
    #[serde(rename = "INTERFACE")]
    Interface,
    #[serde(rename = "UNION")]
    Union,
    #[serde(rename = "LIST")]
    List,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: TypeReference,
    pub description: Option<String>,
    pub locations: Option<Vec<String>>,
    pub args: Option<Vec<TypeReference>>,
}

impl Field {
    pub fn to_field_model(&self, context: &mut Context) -> FieldModel {
        let mut model = FieldModel {
            name: self.name.clone(),
            field_type: FieldType::Unknown,
            field_type_references: vec![],
            nullable: true,
        };
        Field::update_type(context, &self.field_type, &mut model);
        model
    }

    fn update_type(context: &mut Context, reference: &TypeReference, model: &mut FieldModel) {
        if let Some(ref kind) = reference.kind {
            match kind {
                Kind::NonNull => {
                    model.nullable = false;
                    if let Some(x) = &reference.of_type {
                        Field::update_type(context, x, model);
                    }
                }
                Kind::Null => {
                    model.nullable = true;
                    if let Some(x) = &reference.of_type {
                        Field::update_type(context, x, model);
                    }
                }
                Kind::Object => {
                    if model.field_type == FieldType::Unknown {
                        model.field_type = FieldType::Object;
                    }
                    model
                        .field_type_references
                        .push(context.register_reference(reference.name.as_ref().unwrap().as_str()))
                }
                Kind::InputObject => {
                    model.field_type = FieldType::Object;
                    model
                        .field_type_references
                        .push(context.register_reference(reference.name.as_ref().unwrap().as_str()))
                }
                Kind::Scalar => match reference.name.as_ref().unwrap().as_str() {
                    "Boolean" => {
                        model.field_type = FieldType::Boolean;
                    }
                    "String" => {
                        model.field_type = FieldType::String;
                    }
                    "JSON" => {
                        model.field_type = FieldType::JSON;
                    }
                    "DateTime" => {
                        model.field_type = FieldType::DateTime;
                    }
                    "Int" => {
                        model.field_type = FieldType::Int;
                    }
                    "Float" => {
                        model.field_type = FieldType::Float;
                    }
                    "Double" => {
                        model.field_type = FieldType::Double;
                    }
                    _ => {
                        todo!("scalar: {:?}", reference.name)
                    }
                },
                Kind::Enum => {
                    model.field_type = FieldType::Enum;
                    model
                        .field_type_references
                        .push(context.register_reference(reference.name.as_ref().unwrap().as_str()))
                }
                Kind::Interface => {
                    model.field_type = FieldType::Interface;
                    model
                        .field_type_references
                        .push(context.register_reference(reference.name.as_ref().unwrap().as_str()))
                }
                Kind::Union => {
                    model.field_type = FieldType::Union;
                    let name = reference.name.as_ref().unwrap().as_str();
                    let r = context.register_reference(name);
                    if r.get_model().is_none() {
                        let model = ClassModel {
                            type_name: name.to_owned(),
                            name: name.to_owned(),
                            fields: Some(vec![]),
                            enum_values: None
                        };
                        context.register_model(Arc::new(model));
                    }
                    model.field_type_references.push(r)
                }
                Kind::List => {
                    model.field_type = FieldType::List;
                    if let Some(x) = &reference.of_type {
                        Field::update_type(context, x, model);
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypeReference {
    pub kind: Option<Kind>,
    pub name: Option<String>,
    #[serde(rename = "ofType")]
    pub of_type: Option<Box<TypeReference>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Type {
    pub kind: Kind,
    pub name: String,
    pub description: Option<String>,
    pub fields: Option<Vec<Field>>,
    #[serde(rename = "inputFields")]
    pub input_fields: Option<Vec<Field>>,
    pub interfaces: Option<Vec<TypeReference>>,
    #[serde(rename = "enumValues")]
    pub enum_values: Option<Vec<TypeReference>>,
    #[serde(rename = "possibleTypes")]
    pub possible_types: Option<Vec<TypeReference>>,
}

impl Type {
    pub fn to_class_model(&self, context: &mut Context) -> Arc<ClassModel> {
        let mut model = ClassModel {
            type_name: self.name.clone(),
            name: self.name.clone(),
            fields: None,
            enum_values: None
        };
        if let Some(ref fields) = self.fields {
            if model.fields.is_none() {
                model.fields = Some(vec![]);
            }
            for field in fields {
                model.fields.as_mut().unwrap().push(field.to_field_model(context));
            }
        }
        if let Some(ref fields) = self.input_fields {
            if model.fields.is_none() {
                model.fields = Some(vec![]);
            }
            for field in fields {
                model.fields.as_mut().unwrap().push(field.to_field_model(context));
            }
        }
        if let Some(ref fields) = self.enum_values {
            if model.enum_values.is_none() {
                model.enum_values = Some(vec![]);
            }
            for field in fields {
                model.enum_values.as_mut().unwrap().push(field.name.as_ref().unwrap().to_string());
            }
        }
        if model.fields.is_none() && model.enum_values.is_none() {
            model.fields = Some(vec![]);
        }
        let model = Arc::new(model);
        context.register_model(Arc::clone(&model));
        if let Some(ref types) = self.possible_types {
            for t in types.iter() {
                let r = context.register_reference(t.name.as_ref().unwrap());
                context.register_interface_implementation(&self.name, Arc::clone(&r))
            }
        }
        model
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SchemaType {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schema {
    #[serde(rename = "queryType")]
    pub query_type: Option<SchemaType>,
    #[serde(rename = "mutationType")]
    pub mutation_type: Option<SchemaType>,
    #[serde(rename = "subscriptionType")]
    pub subscription_type: Option<SchemaType>,
    pub types: Vec<Type>,
}

impl Schema {
    pub fn to_class_models(&self, context: &mut Context) -> Vec<Arc<ClassModel>> {
        let mut models = vec![];
        for t in self.types.iter() {
            models.push(t.to_class_model(context));
        }
        models
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub __schema: Schema,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    pub data: Data,
}
