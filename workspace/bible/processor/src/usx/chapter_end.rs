use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{EndIdFactoryFilter, ItemFactoryFilter, UsxItem};
use crate::usx::position::Position;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ChapterEnd {
    pub eid: String,
    pub position: Arc<Mutex<Position>>,
    pub verse: Option<String>,
}

impl ChapterEnd {
    pub fn create(
        context: &mut UsxContext,
        _: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Arc::new(Mutex::new(UsxItem::ChapterEnd(Self {
            eid: attributes.get("eid").unwrap().to_string(),
            verse: context.verse().clone(),
            position: Arc::clone(&context.position().unwrap()),
        })))
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::ChapterEnd(ChapterEndFactory {
        base: BaseFactory::new(
            "chapter",
            Some(ItemFactoryFilter::EndId(EndIdFactoryFilter {})),
        ),
    }))
}

#[derive(Debug)]
pub struct ChapterEndFactory {
    base: BaseFactory,
}

impl ChapterEndFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for ChapterEndFactory {
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
        ChapterEnd::create(context, parent, attributes)
    }
}
