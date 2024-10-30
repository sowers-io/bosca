use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{UsxItem, UsxItemContainer};
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Reference {
    pub loc: String,
    pub container: UsxItemContainer,
}

impl Reference {
    pub fn create(
        context: &mut UsxContext,
        _: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Arc::new(Mutex::new(UsxItem::Reference(Reference {
            loc: attributes.get("style").unwrap().as_str().to_string(),
            container: UsxItemContainer::new(context),
        })))
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::Reference(ReferenceFactory {
        base: BaseFactory::new("ref", None),
    }))
}

#[derive(Debug)]
pub struct ReferenceFactory {
    base: BaseFactory,
}

impl ReferenceFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for ReferenceFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(TextFactory::get());
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Reference::create(context, parent, attributes)
    }
}
