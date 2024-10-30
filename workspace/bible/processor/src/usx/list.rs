use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::char::CharFactory;
use crate::usx::cross_reference::CrossReferenceFactory;
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::figure::FigureFactory;
use crate::usx::footnote::FootnoteFactory;
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::list_char::ListCharFactory;
use crate::usx::milestone::MilestoneFactory;
use crate::usx::optbreak::BreakFactory;
use crate::usx::reference::ReferenceFactory;
use crate::usx::styles::ListStyle;
use crate::usx::text::TextFactory;
use crate::usx::verse_end::VerseEndFactory;
use crate::usx::verse_start::VerseStartFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct List {
    pub style: ListStyle,
    pub vid: Option<String>,
    pub container: UsxItemContainer,
}

impl List {
    pub fn create(
        context: &mut UsxContext,
        _parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let container = UsxItemContainer::new(context);
        let item = Arc::new(Mutex::new(UsxItem::List(List {
            style: attributes.get("style").unwrap().as_str().into(),
            vid: attributes.get("vid").cloned(),
            container,
        })));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::List(ListFactory {
        base: BaseFactory::new(
            "para",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                ListStyle::to_str_name(),
            ))),
        ),
    }))
}

#[derive(Debug)]
pub struct ListFactory {
    base: BaseFactory,
}

impl ListFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for ListFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(ReferenceFactory::get());
        self.register(FootnoteFactory::get());
        self.register(CrossReferenceFactory::get());
        self.register(CharFactory::get());
        self.register(ListCharFactory::get());
        self.register(MilestoneFactory::get());
        self.register(FigureFactory::get());
        self.register(VerseStartFactory::get());
        self.register(VerseEndFactory::get());
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
