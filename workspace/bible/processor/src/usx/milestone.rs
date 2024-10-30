use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::UsxItem;
use crate::usx::position::Position;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Milestone {
    pub style: String,
    pub sid: String,
    pub eid: String,
    pub position: Option<Arc<Mutex<Position>>>,
    pub verse: Option<String>,
}

impl Milestone {
    pub fn create(
        context: &mut UsxContext,
        _parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let item = Arc::new(Mutex::new(UsxItem::Milestone(Milestone {
            style: attributes.get("style").unwrap().as_str().to_string(),
            sid: attributes.get("style").unwrap().as_str().to_string(),
            eid: attributes.get("style").unwrap().as_str().to_string(),
            position: context.position().clone(),
            verse: attributes.get("verse").cloned(),
        })));
        item
    }
}

//     toHtml(context: HtmlContext): string {
//         return context.render('milestone', this)
//     }
//
//     toString(): string {
//         return ''
//     }
// }

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::Milestone(MilestoneFactory {
        base: BaseFactory::new("ms", None),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct MilestoneFactory {
    base: BaseFactory,
}

impl MilestoneFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for MilestoneFactory {
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
        Milestone::create(context, parent, attributes)
    }
}
