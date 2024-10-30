use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::char::CharFactory;
use crate::usx::cross_reference::CrossReferenceFactory;
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::figure::FigureFactory;
use crate::usx::footnote::FootnoteFactory;
use crate::usx::item::{UsxItem, UsxItemContainer};
use crate::usx::milestone::MilestoneFactory;
use crate::usx::optbreak::BreakFactory;
use crate::usx::text::TextFactory;
use crate::usx::verse_end::VerseEndFactory;
use crate::usx::verse_start::VerseStartFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Cell {
    pub style: String,
    pub align: Option<String>,
    pub colspan: Option<String>,
    pub container: UsxItemContainer,
}

impl Cell {
    pub fn create(
        context: &mut UsxContext,
        _parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let item = Arc::new(Mutex::new(UsxItem::Cell(Cell {
            style: attributes.get("style").unwrap().to_string(),
            align: attributes.get("align").cloned(),
            colspan: attributes.get("colspan").cloned(),
            container: UsxItemContainer::new(context),
        })));
        item
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::Cell(CellFactory {
        base: BaseFactory::new("cell", None),
    }))
}

#[derive(Debug)]
pub struct CellFactory {
    base: BaseFactory,
}

impl CellFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for CellFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(FootnoteFactory::get());
        self.register(CrossReferenceFactory::get());
        self.register(CharFactory::get());
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
        Cell::create(context, parent, attributes)
    }
}
