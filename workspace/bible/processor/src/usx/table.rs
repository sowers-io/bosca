use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{UsxItem, UsxItemContainer};
use crate::usx::row::RowFactory;
use crate::usx::verse_end::VerseEndFactory;
use crate::usx::verse_start::VerseStartFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Table {
    pub vid: String,
    pub container: UsxItemContainer,
}

impl Table {
    pub fn create(
        context: &mut UsxContext,
        _parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let item = Arc::new(Mutex::new(UsxItem::Table(Table {
            vid: attributes.get("number").unwrap().to_string(),
            container: UsxItemContainer::new(context),
        })));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::Table(TableFactory {
        base: BaseFactory::new("table", None),
    }))
}

#[derive(Debug)]
pub struct TableFactory {
    base: BaseFactory,
}

impl TableFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for TableFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(RowFactory::get());
        self.register(VerseStartFactory::get());
        self.register(VerseEndFactory::get());
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Table::create(context, parent, attributes)
    }
}
