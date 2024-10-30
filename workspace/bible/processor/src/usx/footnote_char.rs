use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::char::CharFactory;
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::styles::FootnoteCharStyle;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct FootnoteChar {
    pub style: FootnoteCharStyle,
    pub container: UsxItemContainer,
    // char.link?,
    // char.closed?,
}

impl FootnoteChar {
    pub fn create(
        context: &mut UsxContext,
        _parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let item = Arc::new(Mutex::new(UsxItem::FootnoteChar(FootnoteChar {
            style: attributes.get("style").unwrap().as_str().into(),
            container: UsxItemContainer::new(context),
        })));
        item
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
    Arc::new(UsxItemFactory::FootnoteChar(FootnoteCharFactory {
        base: BaseFactory::new(
            "char",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                FootnoteCharStyle::to_str_name(),
            ))),
        ),
    }))
}

#[derive(Debug)]
pub struct FootnoteCharFactory {
    base: BaseFactory,
}

impl FootnoteCharFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for FootnoteCharFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(CharFactory::get());
        self.register(TextFactory::get());
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        FootnoteChar::create(context, parent, attributes)
    }
}
