use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{UsxItem, UsxItemContainer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Figure {
    pub style: String,
    pub alt: Option<String>,
    pub file: String,
    pub size: Option<String>,
    pub loc: Option<String>,
    pub copy: Option<String>,
    pub figure_ref: Option<String>,
    pub container: UsxItemContainer,
}

impl Figure {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let container = UsxItemContainer::new(context);
        let item = Arc::new(Mutex::new(UsxItem::Figure(Figure {
            style: attributes.get("style").unwrap().to_string(),
            alt: attributes.get("alt").cloned(),
            file: attributes.get("file").unwrap().to_string(),
            size: attributes.get("size").cloned(),
            loc: attributes.get("loc").cloned(),
            copy: attributes.get("copy").cloned(),
            figure_ref: attributes.get("ref").cloned(),
            container,
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::Figure(FigureFactory {
        base: BaseFactory::new("figure", None),
    }))
}

#[derive(Debug)]
pub struct FigureFactory {
    base: BaseFactory,
}

impl FigureFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for FigureFactory {
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
        Figure::create(context, parent, attributes)
    }
}
