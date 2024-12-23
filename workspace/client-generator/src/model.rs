use std::collections::HashMap;
use std::sync::Arc;
use crate::context::{ClassReference, Context};
use crate::parser::Fields;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FieldType {
    Unknown,
    List,
    Map,
    Float,
    Double,
    Int,
    String,
    JSON,
    Boolean,
    Date,
    DateTime,
    Object,
    Enum,
    Interface,
    Union
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ClassModel {
    pub type_name: String,
    pub name: String,
    pub fields: Option<Vec<FieldModel>>,
    pub enum_values: Option<Vec<String>>,
}

impl ClassModel {
    pub fn apply(&self, context: &mut Context, field: &impl Fields) -> Option<Arc<ClassModel>> {
        if self.fields.is_none() {
            return None;
        }

        let fields = field.get_fields();
        if fields.is_none() {
            return None;
        }
        let fields = fields.unwrap();
        let mut field_names = HashMap::new();

        for f in fields.iter() {
            field_names.insert(f.name.to_owned(), f);
        }

        let mut model = ClassModel {
            type_name: self.name.clone(),
            name: format!("{}.{}", field.name(), self.name),
            fields: Some(Vec::new()),
            enum_values: None
        };
        let model_ref = context.register_reference(&model.name);
        let iface_name = format!("Base{}", self.name);
        context.register_interface_implementation(&iface_name, model_ref);

        for f in self.fields.as_ref().unwrap().iter() {
            if !field_names.contains_key(&f.name) {
                continue;
            }
            let field = *field_names.get(&f.name).unwrap();
            let mut new_field = f.clone();
            new_field.field_type_references = new_field
                .field_type_references
                .into_iter()
                .map(|x| {
                    let model = x.get_model().unwrap();
                    let filtered = model.apply(context, field);
                    if filtered.is_none() {
                        return None;
                    }
                    Some(context.register_reference(filtered.unwrap().name.as_str()))
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect();
            model.fields.as_mut().unwrap().push(new_field)
        }

        let model = Arc::new(model);
        context.register_model(Arc::clone(&model));

        Some(model)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FieldModel {
    pub name: String,
    pub field_type: FieldType,
    pub field_type_references: Vec<Arc<ClassReference>>,
    pub nullable: bool
}
