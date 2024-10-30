use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::cross_reference_char::{CrossReferenceChar, CrossReferenceCharFactory};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::styles::CrossReferenceStyle;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct CrossReference {
    pub style: CrossReferenceStyle,
    pub container: UsxItemContainer,
    pub caller: String,
}

impl CrossReference {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let style = CrossReferenceStyle::from(attributes.get("style").unwrap().as_str());
        let container = UsxItemContainer::new(context);
        let item = Arc::new(Mutex::new(UsxItem::CrossReference(CrossReference {
            style,
            container,
            caller: attributes.get("caller").unwrap().as_str().to_string(),
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }

    pub fn html_attributes(&self) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        attrs.insert("data-caller".to_string(), self.caller.to_string());
        attrs
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::CrossReference(CrossReferenceFactory {
        base: BaseFactory::new(
            "note",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                CrossReferenceStyle::to_str_name(),
            ))),
        ),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct CrossReferenceFactory {
    base: BaseFactory,
}

impl CrossReferenceFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for CrossReferenceFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(CrossReferenceCharFactory::get());
        self.register(TextFactory::get());
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        CrossReferenceChar::create(context, parent, attributes)
    }
}
