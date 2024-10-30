use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::styles::BookHeaderStyle;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct BookHeader {
    pub style: BookHeaderStyle,
    pub container: UsxItemContainer,
}

impl BookHeader {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let style = BookHeaderStyle::from(attributes.get("style").unwrap().as_str());
        let container = UsxItemContainer::new(context);
        let item = Arc::new(Mutex::new(UsxItem::BookHeader(BookHeader {
            style,
            container,
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::BookHeader(BookHeaderFactory {
        base: BaseFactory::new(
            "para",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                BookHeaderStyle::to_str_name(),
            ))),
        ),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct BookHeaderFactory {
    base: BaseFactory,
}

impl BookHeaderFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for BookHeaderFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(TextFactory::get())
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        BookHeader::create(context, parent, attributes)
    }
}
