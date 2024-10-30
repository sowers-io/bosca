use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::char::CharFactory;
use crate::usx::cross_reference::CrossReferenceFactory;
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::footnote::FootnoteFactory;
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::milestone::MilestoneFactory;
use crate::usx::optbreak::BreakFactory;
use crate::usx::styles::BookIntroductionEndTitleStyle;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct BookIntroductionEndTitle {
    pub style: BookIntroductionEndTitleStyle,
    pub container: UsxItemContainer,
}

impl BookIntroductionEndTitle {
    pub fn create(
        context: &mut UsxContext,
        _: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Arc::new(Mutex::new(UsxItem::BookIntroductionEndTitle(Self {
            style: attributes.get("style").unwrap().as_str().into(),
            container: UsxItemContainer::new(context),
        })))
    }
}

// type BookIntroductionEndTitleType = Footnote | CrossReference | Char | Milestone | Break | Text

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    Arc::new(UsxItemFactory::BookIntroductionEndTitle(
        BookIntroductionEndTitleFactory {
            base: BaseFactory::new(
                "para",
                Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                    BookIntroductionEndTitleStyle::to_str_name(),
                ))),
            ),
        },
    ))
}

#[derive(Debug)]
pub struct BookIntroductionEndTitleFactory {
    base: BaseFactory,
}

impl BookIntroductionEndTitleFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for BookIntroductionEndTitleFactory {
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
        self.register(BreakFactory::get());
        self.register(TextFactory::get());
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        BookIntroductionEndTitle::create(context, parent, attributes)
    }
}
