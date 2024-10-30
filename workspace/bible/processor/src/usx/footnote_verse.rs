use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::styles::FootnoteVerseStyle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct FootnoteVerse {
    pub style: FootnoteVerseStyle,
    pub container: UsxItemContainer,
}

impl FootnoteVerse {
    pub fn create(
        context: &mut UsxContext,
        _parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let style = FootnoteVerseStyle::from(attributes.get("style").unwrap().as_str());
        let container = UsxItemContainer::new(context);
        
        Arc::new(Mutex::new(UsxItem::FootnoteVerse(FootnoteVerse {
            style,
            container,
        })))
    }
}

//     toHtml(context: HtmlContext): string {
//         if (!context.includeFootNotes) return ''
//         return super.toHtml(context)
//     }
//
//     toString(context: StringContext | undefined = undefined): string {
//         const ctx = context || StringContext.defaultContext
//         if (!ctx.includeFootNotes) return ''
//         return super.toString(context)
//     }
// }

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::FootnoteVerse(FootnoteVerseFactory {
        base: BaseFactory::new(
            "char",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                FootnoteVerseStyle::to_str_name(),
            ))),
        ),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct FootnoteVerseFactory {
    base: BaseFactory,
}

impl FootnoteVerseFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for FootnoteVerseFactory {
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
        FootnoteVerse::create(context, parent, attributes)
    }
}
