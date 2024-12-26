use crate::context::{ClassReference, Context};
use crate::parser::Fields;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum FieldType {
    Unknown,
    List,
    Float,
    Double,
    Int,
    String,
    Json,
    Boolean,
    DateTime,
    Object,
    Enum,
    Interface,
    Union,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ClassType {
    Interface,
    Class,
    Scalar,
    Enum,
}

#[derive(Clone, Debug)]
pub struct ClassModel {
    pub class_type: ClassType,
    pub type_name: String,
    pub name: String,
    fields: Option<Arc<Mutex<Vec<FieldModel>>>>,
    enum_values: Option<Vec<String>>,
}

impl ClassModel {
    pub fn new(class_type: ClassType, type_name: String, name: String) -> Self {
        let fields = if class_type == ClassType::Class || class_type == ClassType::Interface { Some(Arc::new(Mutex::new(vec![]))) } else { None };
        let enum_values = if class_type == ClassType::Enum { Some(vec![]) } else { None };
        ClassModel {
            class_type,
            type_name,
            name,
            fields,
            enum_values,
        }
    }

    pub fn has_fields(&self) -> bool {
        self.fields.is_some()
    }

    pub fn get_enum_values(&self) -> &Option<Vec<String>> {
        if self.enum_values.is_none() {
            return &None
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
        self.name == other.name
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
            model.fields.as_mut().unwrap().lock().unwrap().push(new_field)
        }

        let model = Arc::new(model);
        context.register_model(Arc::clone(&model));

        Some(model)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct FieldModel {
    pub name: String,
    pub field_type: FieldType,
    pub field_type_scalar: FieldType,
    pub field_type_references: Vec<Arc<ClassReference>>,
    pub nullable: bool,
}
