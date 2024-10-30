use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem};
use crate::usx::position::Position;
use crate::usx::styles::VerseStartStyle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct VerseStart {
    pub style: VerseStartStyle,
    pub number: String,
    pub altnumber: Option<String>,
    pub pubnumber: Option<String>,
    pub sid: String,
    pub position: Arc<Mutex<Position>>,
}

impl VerseStart {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let item = Arc::new(Mutex::new(UsxItem::VerseStart(Self {
            style: attributes.get("style").unwrap().as_str().into(),
            number: attributes.get("number").unwrap().clone(),
            sid: attributes.get("sid").unwrap().clone(),
            altnumber: attributes.get("altnumber").cloned(),
            pubnumber: attributes.get("pubnumber").cloned(),
            position: context.position().map(|p| Arc::clone(&p)).unwrap(),
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }

    pub fn html_attributes(&self) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        attrs.insert("data-verse".to_string(), self.number.clone());
        attrs
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::VerseStart(VerseStartFactory {
        base: BaseFactory::new(
            "verse",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                VerseStartStyle::to_str_name(),
            ))),
        ),
    }))
}

#[derive(Debug)]
pub struct VerseStartFactory {
    base: BaseFactory,
}

impl VerseStartFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for VerseStartFactory {
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
        VerseStart::create(context, parent, attributes)
    }
}
