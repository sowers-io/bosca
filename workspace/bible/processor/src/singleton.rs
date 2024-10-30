use crate::usx::factory::UsxItemFactory;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Once};

pub struct FactorySingleton {
    once: Once,
    instance: FactoryHandle,
    initialized: AtomicBool,
}

#[derive(Debug)]
pub struct FactoryHandle {
    name: Option<String>,
    tag_name: Option<String>,
    factory: Option<Arc<UsxItemFactory>>,
}

impl FactoryHandle {
    pub fn name(&self) -> &String {
        self.name.as_ref().unwrap()
    }
    pub fn tag_name(&self) -> &String {
        self.tag_name.as_ref().unwrap()
    }
    pub fn get(&self) -> Arc<UsxItemFactory> {
        Arc::clone(self.factory.as_ref().unwrap())
    }
}

impl FactorySingleton {
    pub const fn new() -> Self {
        Self {
            once: Once::new(),
            instance: FactoryHandle {
                name: None,
                tag_name: None,
                factory: None,
            },
            initialized: AtomicBool::new(false),
        }
    }

    pub fn initialize<F>(&mut self, f: F)
    where
        F: Fn() -> Arc<UsxItemFactory>,
    {
        if self.initialized.load(Ordering::Relaxed) {
            return;
        }
        self.initialized.store(true, Ordering::Relaxed);
        self.once.call_once(|| {
            let mut factory = f();
            let handle = &mut self.instance;
            handle.name = Some(factory.name().to_string());
            handle.tag_name = Some(factory.tag_name().clone());
            let f = Arc::get_mut(&mut factory).unwrap();
            f.initialize();
            handle.factory = Some(factory);
        })
    }

    pub fn get(&self) -> &FactoryHandle {
        &self.instance
    }
}
