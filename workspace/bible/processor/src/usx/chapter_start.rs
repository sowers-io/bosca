use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{EndIdFactoryFilter, ItemFactoryFilter, NegateFactoryFilter, UsxItem};
use crate::usx::position::Position;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ChapterStart {
    pub number: String,
    pub sid: String,
    pub altnumber: Option<String>,
    pub pubnumber: Option<String>,
    pub position: Arc<Mutex<Position>>,
    pub verse: Option<String>,
}

impl ChapterStart {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let item = Arc::new(Mutex::new(UsxItem::ChapterStart(Self {
            number: attributes.get("number").unwrap().to_string(),
            sid: attributes.get("sid").unwrap().to_string(),
            altnumber: attributes.get("altnumber").cloned(),
            pubnumber: attributes.get("pubnumber").cloned(),
            position: Arc::clone(&context.position().unwrap()),
            verse: context.verse().clone(),
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::ChapterStart(ChapterStartFactory {
        base: BaseFactory::new(
            "chapter",
            Some(ItemFactoryFilter::Negate(NegateFactoryFilter::new(
                Arc::new(ItemFactoryFilter::EndId(EndIdFactoryFilter {})),
            ))),
        ),
    }))
}

#[derive(Debug)]
pub struct ChapterStartFactory {
    base: BaseFactory,
}

impl ChapterStartFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for ChapterStartFactory {
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
        ChapterStart::create(context, parent, attributes)
    }
}
