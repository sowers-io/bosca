use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::UsxItem;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Text {
    pub text: Option<String>,
    pub verse: Option<String>,
}

impl Text {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
    ) -> Arc<Mutex<UsxItem>> {
        let item = Arc::new(Mutex::new(UsxItem::Text(Text {
            text: None,
            verse: context.verse().clone(),
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }

    pub fn html_attributes(&self) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        if let Some(verse) = &self.verse {
            attrs.insert("data-verse".to_string(), verse.clone());
        }
        attrs
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::Text(TextFactory {
        base: BaseFactory::new("#text", None),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct TextFactory {
    base: BaseFactory,
}

impl TextFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for TextFactory {
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
        _: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Text::create(context, parent)
    }
}
