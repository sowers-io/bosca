use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{EndIdFactoryFilter, ItemFactoryFilter, UsxItem};
use crate::usx::position::Position;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct VerseEnd {
    pub eid: String,
    pub verse: Option<String>,
    pub position: Arc<Mutex<Position>>,
}

impl VerseEnd {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let eid = attributes.get("eid").unwrap_or(&"".to_string()).clone();
        let verse = context.verse().clone();
        let position = context.position().map(|p| Arc::clone(&p)).unwrap();
        let item = Arc::new(Mutex::new(UsxItem::VerseEnd(VerseEnd {
            eid,
            verse,
            position,
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::VerseEnd(VerseEndFactory {
        base: BaseFactory::new(
            "verse",
            Some(ItemFactoryFilter::EndId(EndIdFactoryFilter {})),
        ),
    }))
}

#[derive(Debug)]
pub struct VerseEndFactory {
    base: BaseFactory,
}

impl VerseEndFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for VerseEndFactory {
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
        VerseEnd::create(context, parent, attributes)
    }
}
