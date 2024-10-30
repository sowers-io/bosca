use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::styles::BookChapterLabelStyle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct BookChapterLabel {
    pub style: BookChapterLabelStyle,
    pub container: UsxItemContainer,
}

impl BookChapterLabel {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let style = BookChapterLabelStyle::from(attributes.get("style").unwrap().as_str());
        let container = UsxItemContainer::new(context);
        let item = Arc::new(Mutex::new(UsxItem::BookChapterLabel(BookChapterLabel {
            style,
            container,
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::BookChapterLabel(BookChapterLabelFactory {
        base: BaseFactory::new(
            "para",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                BookChapterLabelStyle::to_str_name(),
            ))),
        ),
    });
    Arc::new(factory)
}

impl BookChapterLabelFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

#[derive(Debug)]
pub struct BookChapterLabelFactory {
    base: BaseFactory,
}

impl IUsxItemFactory for BookChapterLabelFactory {
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
        BookChapterLabel::create(context, parent, attributes)
    }
}
