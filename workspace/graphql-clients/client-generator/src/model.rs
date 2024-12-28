use crate::context::{ClassReference, Context};
use crate::introspection::Kind;
use crate::parser::Fields;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum FieldType {
    Unknown,
    List,
    Float,
    Double,
    #[allow(dead_code)]
    Int,
    Long,
    String,
    Json,
    Boolean,
    DateTime,
    Object,
    Enum,
    Interface,
    Union,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ClassType {
    Interface,
    Class,
    Scalar,
    Enum,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassModel {
    pub class_type: ClassType,
    pub source_kind: Kind,
    pub type_name: String,
    pub name: String,
    pub class_type_references: Vec<ClassReference>,
    fields: Option<Arc<Mutex<Vec<FieldModel>>>>,
    enum_values: Option<Vec<String>>,
}

impl ClassModel {
    pub fn new(class_type: ClassType, source_kind: Kind, type_name: String, name: String) -> Self {
        Self::new_with_references(class_type, source_kind, type_name, name, Vec::new())
    }

    pub fn new_with_references(class_type: ClassType, source_kind: Kind, type_name: String, name: String, class_type_references: Vec<ClassReference>) -> Self {
        let fields = if class_type == ClassType::Class || class_type == ClassType::Interface {
            Some(Arc::new(Mutex::new(vec![])))
        } else {
            None
        };
        let enum_values = if class_type == ClassType::Enum {
            Some(vec![])
        } else {
            None
        };
        ClassModel {
            class_type,
            source_kind,
            type_name,
            class_type_references,
            name,
            fields,
            enum_values,
        }
    }
    
    pub fn is_internal(&self) -> bool {
        self.name.starts_with("__")
            || self.name.starts_with("I__")
            || self.name == "IJSON"
            || self.name == "JSON"
            || self.name == "IString"
            || self.name == "String"
            || self.name == "IDateTime"
            || self.name == "DateTime"
            || self.name == "IFloat"
            || self.name == "Float"
            || self.name == "ILong"
            || self.name == "Long"
            || self.name == "IInt"
            || self.name == "Int"
            || self.name == "IBoolean"
            || self.name == "Boolean"
            || self.name == "IID"
            || self.name == "ID"
            || self.name == "IUpload"
            || self.name == "Upload"
    }

    pub fn has_fields(&self) -> bool {
        self.fields.is_some()
    }

    pub fn get_enum_values(&self) -> &Option<Vec<String>> {
        if self.enum_values.is_none() {
            return &None;
        }
        &self.enum_values
    }

    pub fn get_fields(&self) -> Option<Vec<FieldModel>> {
        let fields = self.fields.as_ref()?.lock().unwrap();
        Some(fields.iter().cloned().collect())
    }

    pub fn set_fields(&self, fields: Vec<FieldModel>) {
        let mut f = self.fields.as_ref().unwrap().lock().unwrap();
        f.clear();
        f.extend(fields);
    }

    pub fn add_field_model(&mut self, field: FieldModel) {
        self.fields.as_mut().unwrap().lock().unwrap().push(field);
    }

    pub fn add_enum_value(&mut self, value: String) {
        self.enum_values.as_mut().unwrap().push(value);
    }
}

impl PartialEq<ClassModel> for ClassModel {
    fn eq(&self, other: &ClassModel) -> bool {
        self.name == other.name || self.type_name == other.type_name
    }
}

impl Eq for ClassModel {}

impl ClassModel {
    pub fn apply(&self, context: &mut Context, field: &impl Fields) -> Option<Arc<ClassModel>> {
        let fields = field.get_fields()?;
        let mut field_names = HashMap::new();

        for f in fields.iter() {
            field_names.insert(f.name.to_owned(), f);
        }

        let mut model = ClassModel::new(
            self.class_type.clone(),
            self.source_kind.clone(),
            self.name.clone(),
            format!("{}.{}", field.name(), self.name),
        );
        let model_ref = context.register_reference(&model.name);
        let iface_name = format!("I{}", self.name);
        context.register_interface_implementation(&iface_name, model_ref);

        for f in self.fields.as_ref().unwrap().lock().unwrap().iter() {
            if !field_names.contains_key(&f.name) {
                continue;
            }
            let field = *field_names.get(&f.name).unwrap();
            let mut new_field = f.clone();
            new_field.field_type_references = new_field
                .field_type_references
                .into_iter()
                .filter_map(|x| {
                    let model = x.get_model().unwrap();
                    if model.class_type == ClassType::Enum {
                        Some(context.register_reference(model.name.as_str()))
                    } else {
                        let filtered = model.apply(context, field);
                        Some(context.register_reference(filtered?.name.as_str()))
                    }
                })
                .collect();
            model
                .fields
                .as_mut()
                .unwrap()
                .lock()
                .unwrap()
                .push(new_field)
        }

        let model = Arc::new(model);
        context.register_model(Arc::clone(&model));

        Some(model)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldModel {
    pub name: String,
    pub field_type: FieldType,
    pub field_type_scalar: FieldType,
    pub field_type_references: Vec<ClassReference>,
    pub nullable: bool,
}

impl Hash for FieldModel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.field_type.hash(state);
        self.field_type_scalar.hash(state);
        self.field_type_references.hash(state);
        self.nullable.hash(state);
    }
}

impl PartialEq for FieldModel {
    fn eq(&self, other: &Self) -> bool {
        if !(self.name == other.name
            && self.field_type == other.field_type
            && self.field_type_scalar == other.field_type_scalar
            && self.field_type_references == other.field_type_references)
        {
            if self.name == other.name {
                return false;
            }
            return false;
        }
        true
    }
}

impl Eq for FieldModel {}
