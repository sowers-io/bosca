use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::UsxItem;
use crate::usx::position::Position;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Break {
    pub verse: Option<String>,
    pub position: Arc<Mutex<Position>>,
}

impl Break {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
    ) -> Arc<Mutex<UsxItem>> {
        let item = Arc::new(Mutex::new(UsxItem::Break(Break {
            verse: context.verse().clone(),
            position: context.position().unwrap().clone(),
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }
}
//     toHtml(context: HtmlContext): string {
//         return context.render('br', this)
//     }
//
//     toString(): string {
//         return ''
//     }

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::Break(BreakFactory {
        base: BaseFactory::new("optbreak", None),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct BreakFactory {
    base: BaseFactory,
}

impl BreakFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for BreakFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(TextFactory::get())
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        _: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Break::create(context, parent)
    }
}
