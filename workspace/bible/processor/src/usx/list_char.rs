use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::char::CharFactory;
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::footnote::FootnoteFactory;
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::list::List;
use crate::usx::milestone::MilestoneFactory;
use crate::usx::optbreak::BreakFactory;
use crate::usx::reference::ReferenceFactory;
use crate::usx::styles::ListCharStyle;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ListChar {
    pub style: ListCharStyle,
    pub container: UsxItemContainer,
    // char.link?
    // char.closed?
}

impl ListChar {
    pub fn create(
        context: &mut UsxContext,
        _parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let container = UsxItemContainer::new(context);
        let item = Arc::new(Mutex::new(UsxItem::ListChar(ListChar {
            style: attributes.get("style").unwrap().as_str().into(),
            container,
        })));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::ListChar(ListCharFactory {
        base: BaseFactory::new(
            "char",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                ListCharStyle::to_str_name(),
            ))),
        ),
    }))
}

#[derive(Debug)]
pub struct ListCharFactory {
    base: BaseFactory,
}

impl ListCharFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for ListCharFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(ReferenceFactory::get());
        self.register(CharFactory::get());
        self.register(MilestoneFactory::get());
        self.register(FootnoteFactory::get());
        self.register(BreakFactory::get());
        self.register(TextFactory::get());
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        List::create(context, parent, attributes)
    }
}
