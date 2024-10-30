use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::styles::IntroCharStyle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct IntroChar {
    pub style: IntroCharStyle,
    pub container: UsxItemContainer,
    // char.closed?
}

impl IntroChar {
    pub fn create(
        context: &mut UsxContext,
        _parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let container = UsxItemContainer::new(context);
        let item = Arc::new(Mutex::new(UsxItem::IntroChar(IntroChar {
            style: attributes.get("style").unwrap().as_str().into(),
            container,
        })));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::IntroChar(IntroCharFactory {
        base: BaseFactory::new(
            "char",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                IntroCharStyle::to_str_name(),
            ))),
        ),
    }))
}

#[derive(Debug)]
pub struct IntroCharFactory {
    base: BaseFactory,
}

impl IntroCharFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for IntroCharFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {}

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        IntroChar::create(context, parent, attributes)
    }
}
