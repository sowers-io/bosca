use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::ItemFactoryFilter::Style;
use crate::usx::item::{StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::styles::BookTitleStyle;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct BookTitle {
    pub style: BookTitleStyle,
    pub container: UsxItemContainer,
}

impl BookTitle {
    pub fn create(
        context: &mut UsxContext,
        _: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Arc::new(Mutex::new(UsxItem::BookTitle(BookTitle {
            style: attributes.get("style").unwrap().as_str().into(),
            container: UsxItemContainer::new(context),
        })))
    }
}

// type BookTitleType = Footnote | CrossReference | Char | Break | Text

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::BookTitle(BookTitleFactory {
        base: BaseFactory::new(
            "para",
            Some(Style(
                StyleFactoryFilter::new(BookTitleStyle::to_str_name()),
            )),
        ),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct BookTitleFactory {
    base: BaseFactory,
}

impl BookTitleFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for BookTitleFactory {
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
        BookTitle::create(context, parent, attributes)
    }
}
