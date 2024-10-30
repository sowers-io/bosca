use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::char::CharFactory;
use crate::usx::cross_reference::CrossReferenceFactory;
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::figure::FigureFactory;
use crate::usx::footnote::FootnoteFactory;
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::milestone::MilestoneFactory;
use crate::usx::optbreak::BreakFactory;
use crate::usx::reference::ReferenceFactory;
use crate::usx::styles::ParaStyle;
use crate::usx::text::TextFactory;
use crate::usx::verse_end::VerseEndFactory;
use crate::usx::verse_start::VerseStartFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Paragraph {
    pub style: ParaStyle,
    pub vid: Option<String>,
    pub container: UsxItemContainer,
}

impl Paragraph {
    pub fn create(
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        let style = ParaStyle::from(attributes.get("style").unwrap().as_str());
        let container = UsxItemContainer::new(context);
        let item = Arc::new(Mutex::new(UsxItem::Paragraph(Paragraph {
            style,
            container,
            vid: attributes.get("vid").cloned(),
        })));
        context.add_verse_item(parent, Arc::clone(&item));
        item
    }
}

//
//     toHtml(context: HtmlContext): string {
//         return context.render('p', this)
//     }
// }

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::Paragraph(ParagraphFactory {
        base: BaseFactory::new(
            "para",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                ParaStyle::to_str_name(),
            ))),
        ),
    }))
}

#[derive(Debug)]
pub struct ParagraphFactory {
    base: BaseFactory,
}

impl ParagraphFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for ParagraphFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(ReferenceFactory::get());
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
        Paragraph::create(context, parent, attributes)
    }
}
