use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::footnote::FootnoteFactory;
use crate::usx::item::{UsxItem, UsxItemContainer};
use crate::usx::milestone::MilestoneFactory;
use crate::usx::optbreak::BreakFactory;
use crate::usx::reference::ReferenceFactory;
use crate::usx::styles::CharStyle;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Char {
    pub style: CharStyle,
    pub container: UsxItemContainer,
}

impl Char {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let item = Arc::new(Mutex::new(UsxItem::Char(Char {
            style: attributes.get("style").unwrap().as_str().into(),
            container: UsxItemContainer::new(context),
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::Char(CharFactory {
        base: BaseFactory::new("char", None),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct CharFactory {
    base: BaseFactory,
}

impl CharFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for CharFactory {
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
        self.register(TextFactory::get())
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Char::create(context, parent, attributes)
    }
}
