use crate::model::{ClassModel, FieldModel};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub struct Context {
    classes: HashMap<String, Arc<ClassReference>>,
    class_interfaces: HashMap<String, Vec<Arc<ClassReference>>>,
    interfaces: HashMap<String, Vec<Arc<ClassReference>>>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            classes: HashMap::new(),
            class_interfaces: HashMap::new(),
            interfaces: HashMap::new(),
        }
    }
}

impl Context {
    fn register_base_interface(&mut self, class_name: &str) {
        if class_name.starts_with("Base") {
            return;
        }
        let iface_name = format!("Base{}", class_name);
        if self.interfaces.contains_key(class_name) {
            return;
        }
        let iface = self.register_reference(&iface_name);
        if iface.get_model().is_none() {
            let iface_model = ClassModel {
                type_name: class_name.to_owned(),
                name: iface_name.clone(),
                fields: Some(Vec::new()),
                enum_values: None,
            };
            self.register_model(Arc::new(iface_model));
            let r = self.register_reference(&class_name);
            self.register_interface_implementation(&iface_name, r);
        }
    }

    pub fn register_interface_implementation(
        &mut self,
        interface_name: &str,
        implementation: Arc<ClassReference>,
    ) {
        if let Some(implementations) = self.interfaces.get_mut(interface_name) {
            implementations.push(Arc::clone(&implementation));
        } else {
            self.interfaces
                .insert(interface_name.to_owned(), vec![Arc::clone(&implementation)]);
        }
        let interface = self.register_reference(interface_name);
        if let Some(class) = self.class_interfaces.get_mut(&implementation.name) {
            class.push(Arc::clone(&interface));
        } else {
            self.class_interfaces
                .insert(implementation.name.to_owned(), vec![interface]);
        }
    }

    pub fn register_reference(&mut self, name: &str) -> Arc<ClassReference> {
        if let Some(existing) = self.classes.get(name) {
            return Arc::clone(&existing);
        }
        let new_class = Arc::new(ClassReference::new(name));
        self.classes.insert(name.to_owned(), Arc::clone(&new_class));
        new_class
    }

    pub fn register_model(&mut self, model: Arc<ClassModel>) -> Arc<ClassReference> {
        self.register_base_interface(&model.name);
        if let Some(existing) = self.classes.get(&model.name) {
            existing.set_model(model);
            return Arc::clone(&existing);
        }
        let new_class = Arc::new(ClassReference::new(&model.name));
        self.classes
            .insert(model.name.to_owned(), Arc::clone(&new_class));
        new_class.set_model(model);
        new_class
    }

    pub fn get_class_interfaces(&self, name: &str) -> Vec<Arc<ClassReference>> {
        self.class_interfaces.get(name).unwrap_or(&vec![]).to_vec()
    }

    pub fn get_all_class_interfaces(&self) -> Vec<Arc<ClassReference>> {
        self.class_interfaces.values().flatten().cloned().collect()
    }

    pub fn get_classes(&self) -> Vec<Arc<ClassReference>> {
        self.classes.iter().map(|x| Arc::clone(x.1)).collect()
    }
}

#[derive(Clone, Debug)]
pub struct ClassReference {
    pub name: String,
    model: Arc<Mutex<Option<Arc<ClassModel>>>>,
}

impl ClassReference {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            model: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_model(&self) -> Option<Arc<ClassModel>> {
        if let Some(model) = self.model.lock().unwrap().as_ref() {
            Some(Arc::clone(model))
        } else {
            None
        }
    }

    fn set_model(&self, model: Arc<ClassModel>) {
        *self.model.lock().unwrap() = Some(model);
    }
}

impl PartialEq<ClassReference> for ClassReference {
    fn eq(&self, other: &ClassReference) -> bool {
        self.name == other.name
    }
}

impl Eq for ClassReference {}
