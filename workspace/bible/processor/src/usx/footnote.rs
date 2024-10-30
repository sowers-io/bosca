use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::footnote_char::FootnoteCharFactory;
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::styles::FootnoteStyle;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Footnote {
    pub style: FootnoteStyle,
    pub caller: String,
    pub category: Option<String>,
    pub container: UsxItemContainer,
}

impl Footnote {
    pub fn create(
        context: &mut UsxContext,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let item = Arc::new(Mutex::new(UsxItem::Footnote(Footnote {
            style: attributes.get("style").unwrap().as_str().into(),
            caller: attributes.get("caller").unwrap().as_str().to_string(),
            category: attributes.get("category").cloned(),
            container: UsxItemContainer::new(context),
        })));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::Footnote(FootnoteFactory {
        base: BaseFactory::new(
            "note",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                FootnoteStyle::to_str_name(),
            ))),
        ),
    }))
}

#[derive(Debug)]
pub struct FootnoteFactory {
    base: BaseFactory,
}

impl FootnoteFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for FootnoteFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(FootnoteCharFactory::get());
        self.register(TextFactory::get());
    }

    fn create(
        &self,
        context: &mut UsxContext,
        _: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Footnote::create(context, attributes)
    }
}
