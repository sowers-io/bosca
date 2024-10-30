use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{CodeFactoryFilter, ItemFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::styles::BookIdentificationCode;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::{Arc, Mutex};

pub struct BookIdentification {
    pub id: String,
    pub code: BookIdentificationCode,
    pub container: UsxItemContainer,
}

impl BookIdentification {
    pub fn create(
        context: &mut UsxContext,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let mut chap = "Chap".to_string();
        let _ = chap.write_str(attributes.get("code").unwrap().as_str());
        let item = Arc::new(Mutex::new(UsxItem::BookIdentification(Self {
            id: attributes.get("style").unwrap().to_string(),
            code: BookIdentificationCode::from(chap.as_str()),
            container: UsxItemContainer::new(context),
        })));
        item
    }

    pub fn html_attributes(&self) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        attrs.insert("data-id".to_string(), self.id.to_string());
        attrs.insert("data-code".to_string(), self.code.to_string());
        attrs
    }
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::BookIdentification(BookIdentificationFactory {
        base: BaseFactory::new(
            "book",
            Some(ItemFactoryFilter::Code(CodeFactoryFilter::new(
                BookIdentificationCode::to_str_name(),
            ))),
        ),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct BookIdentificationFactory {
    base: BaseFactory,
}

impl BookIdentificationFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for BookIdentificationFactory {
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
        _: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        BookIdentification::create(context, attributes)
    }
}
