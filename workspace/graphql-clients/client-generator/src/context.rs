use crate::introspection::Kind;
use crate::model::{ClassModel, ClassType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Context {
    classes: HashMap<String, ClassReference>,
    class_interfaces: HashMap<String, Vec<ClassReference>>,
    interfaces: HashMap<String, Vec<ClassReference>>,
    base_interfaces: HashSet<String>,
}

impl Context {
    fn register_base_interface(&mut self, source_kind: Kind, type_name: &str, class_name: &str) {
        if self.base_interfaces.contains(class_name) {
            return;
        }
        self.base_interfaces.insert(class_name.to_owned());
        let iface_name = format!("I{}", class_name);
        if self.interfaces.contains_key(class_name) {
            return;
        }
        let iface = self.register_reference(&iface_name);
        if iface.get_model().is_none() {
            let iface_model = ClassModel::new(
                ClassType::Interface,
                source_kind,
                type_name.to_owned(),
                iface_name.clone(),
            );
            self.register_model_and_base(Arc::new(iface_model), false);
            let r = self.register_reference(class_name);
            self.register_interface_implementation(&iface_name, r);
        }
    }

    pub fn register_interface_implementation(
        &mut self,
        interface_name: &str,
        implementation: ClassReference,
    ) {
        if let Some(implementations) = self.interfaces.get_mut(interface_name) {
            implementations.push(implementation.clone());
        } else {
            self.interfaces
                .insert(interface_name.to_owned(), vec![implementation.clone()]);
        }
        let interface = self.register_reference(interface_name);
        if let Some(class) = self.class_interfaces.get_mut(&implementation.name) {
            class.push(interface);
        } else {
            self.class_interfaces
                .insert(implementation.name.to_owned(), vec![interface]);
        }
    }

    pub fn register_reference(&mut self, name: &str) -> ClassReference {
        if let Some(existing) = self.classes.get(name) {
            return existing.clone();
        }
        let new_class = ClassReference::new(name);
        self.classes.insert(name.to_owned(), new_class.clone());
        new_class
    }

    pub fn register_model(&mut self, model: Arc<ClassModel>) -> ClassReference {
        self.register_model_and_base(model, true)
    }

    pub fn register_model_and_base(
        &mut self,
        model: Arc<ClassModel>,
        base_interfaces: bool,
    ) -> ClassReference {
        if base_interfaces {
            self.register_base_interface(model.source_kind.clone(), &model.type_name, &model.name);
        }
        if let Some(existing) = self.classes.get(&model.name) {
            existing.set_model(model);
            return existing.clone();
        }
        let new_class = ClassReference::new(&model.name);
        self.classes
            .insert(model.name.to_owned(), new_class.clone());
        new_class.set_model(model);
        new_class
    }

    pub fn is_class_interface(&self, name: &str) -> bool {
        self.interfaces.contains_key(name)
    }

    pub fn get_class_interfaces(&self, name: &str) -> Vec<ClassReference> {
        self.class_interfaces.get(name).unwrap_or(&vec![]).to_vec()
    }

    pub fn get_interface_implementations(&self, name: &str) -> Vec<ClassReference> {
        self.interfaces.get(name).unwrap_or(&vec![]).to_vec()
    }

    pub fn build_interface_fields(&self) {
        for model in self.classes.values() {
            if let Some(model) = model.get_model() {
                if model.class_type == ClassType::Interface {
                    #[allow(clippy::mutable_key_type)]
                    let mut fields = HashSet::new();
                    let impls = self.interfaces.get(&model.name);
                    if impls.is_none() || impls.unwrap().is_empty() {
                        continue;
                    }
                    let impls = impls.unwrap().iter().filter(|i| i.get_model().is_some() && i.get_model().unwrap().has_fields());
                    let mut fields_set = false;
                    for i in impls {
                        if let Some(i) = i.get_model() {
                            if !i.has_fields() || i.get_fields().unwrap().is_empty() {
                                fields.clear();
                                break;
                            }
                            #[allow(clippy::mutable_key_type)]
                            let class_fields = i
                                .get_fields()
                                .unwrap()
                                .iter()
                                .cloned()
                                .collect::<HashSet<_>>();
                            if !fields_set && i.has_fields() {
                                fields_set = true;
                                fields.extend(class_fields);
                            } else {
                                #[allow(clippy::mutable_key_type)]
                                let new_fields = fields.intersection(&class_fields).cloned().collect::<HashSet<_>>();
                                fields = new_fields;
                            }
                        }
                    }
                    model.set_fields(fields.into_iter().collect());
                }
            }
        }
    }

    pub fn get_classes(&self) -> Vec<ClassReference> {
        self.classes.iter().map(|x| x.1.clone()).collect()
    }

    pub fn get_class_models(&self) -> Vec<Arc<ClassModel>> {
        self.get_classes()
            .iter()
            .map(|x| {
                if let Some(x) = x.get_model() {
                    return x;
                }
                panic!("missing model: {}", x.name)
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassReference {
    pub name: String,
    #[serde(skip)]
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
        self.model.lock().unwrap().as_ref().map(Arc::clone)
    }

    fn set_model(&self, model: Arc<ClassModel>) {
        *self.model.lock().unwrap() = Some(model);
    }
}

impl Hash for ClassReference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq<ClassReference> for ClassReference {
    fn eq(&self, other: &ClassReference) -> bool {
        if self.name == other.name {
            return true;
        }

        if let Some(self_model) = self.get_model() {
            if let Some(other_model) = other.get_model() {
                if self_model.type_name == other_model.type_name {
                    return true;
                }
            }
        }

        false
    }
}

impl Eq for ClassReference {}
