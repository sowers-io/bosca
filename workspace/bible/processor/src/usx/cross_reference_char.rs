use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::char::CharFactory;
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::reference::ReferenceFactory;
use crate::usx::styles::CrossReferenceCharStyle;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct CrossReferenceChar {
    pub style: CrossReferenceCharStyle,
    pub container: UsxItemContainer,
}

impl CrossReferenceChar {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let style = CrossReferenceCharStyle::from(attributes.get("style").unwrap().as_str());
        let container = UsxItemContainer::new(context);
        let item = Arc::new(Mutex::new(UsxItem::CrossReferenceChar(
            CrossReferenceChar { style, container },
        )));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }
}

//     toHtml(context: HtmlContext): string {
//         if (!context.includeCrossReferences) return ''
//         return super.toHtml(context)
//     }
//
//     toString(context: StringContext | undefined = undefined): string {
//         const ctx = context || StringContext.defaultContext
//         if (!ctx.includeCrossReferences) return ''
//         return super.toString(context)
//     }
// }

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::CrossReferenceChar(CrossReferenceCharFactory {
        base: BaseFactory::new(
            "char",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                CrossReferenceCharStyle::to_str_name(),
            ))),
        ),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct CrossReferenceCharFactory {
    base: BaseFactory,
}

impl CrossReferenceCharFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for CrossReferenceCharFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(CharFactory::get());
        self.register(CrossReferenceCharFactory::get());
        self.register(ReferenceFactory::get());
        self.register(TextFactory::get())
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        CrossReferenceChar::create(context, parent, attributes)
    }
}
