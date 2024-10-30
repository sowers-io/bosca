use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::char::CharFactory;
use crate::usx::cross_reference::CrossReferenceFactory;
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::figure::FigureFactory;
use crate::usx::footnote::FootnoteFactory;
use crate::usx::intro_char::IntroCharFactory;
use crate::usx::item::{ItemFactoryFilter, StyleFactoryFilter, UsxItem, UsxItemContainer};
use crate::usx::milestone::MilestoneFactory;
use crate::usx::reference::ReferenceFactory;
use crate::usx::styles::BookIntroductionStyle;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct BookIntroduction {
    pub style: BookIntroductionStyle,
    pub container: UsxItemContainer,
}

impl BookIntroduction {
    pub fn create(
        context: &mut UsxContext,
        _: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Arc::new(Mutex::new(UsxItem::BookIntroduction(Self {
            style: attributes.get("style").unwrap().as_str().into(),
            container: UsxItemContainer::new(context),
        })))
    }
}

// type BookIntroductionType = Reference | Footnote | CrossReference | Char | IntroChar | Milestone | Figure | Text
//
// export class BookIntroduction extends UsxItemContainer<BookIntroductionType> {
//     style: BookIntroductionStyle
//
//     constructor(context: UsxContext, parent: UsxItem | null, attributes: Attributes) {
//         super(context, parent, attributes)
//         this.style = attributes.STYLE.toString() as BookIntroductionStyle
//     }
//
//     get htmlClass(): string {
//         return 'book-introduction ' + this.style
//     }
// }

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::BookIntroduction(BookIntroductionFactory {
        base: BaseFactory::new(
            "para",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                BookIntroductionStyle::to_str_name(),
            ))),
        ),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct BookIntroductionFactory {
    base: BaseFactory,
}

impl BookIntroductionFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for BookIntroductionFactory {
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
        self.register(IntroCharFactory::get());
        self.register(MilestoneFactory::get());
        self.register(FigureFactory::get());
        self.register(TextFactory::get());
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        BookIntroduction::create(context, parent, attributes)
    }
}

static mut INSTANCE_TABLE: FactorySingleton = FactorySingleton::new();

fn factory_table() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::BookIntroductionTable(BookIntroductionTableFactory {
        base: BaseFactory::new(
            "table",
            Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                BookIntroductionStyle::to_str_name(),
            ))),
        ),
    });
    Arc::new(factory)
}

#[derive(Debug)]
pub struct BookIntroductionTableFactory {
    base: BaseFactory,
}

impl BookIntroductionTableFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE_TABLE.initialize(factory_table);
            INSTANCE_TABLE.get()
        }
    }
}

impl Default for BookIntroductionTableFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl BookIntroductionTableFactory {
    pub fn new() -> Self {
        Self {
            base: BaseFactory::new(
                "table",
                Some(ItemFactoryFilter::Style(StyleFactoryFilter::new(
                    BookIntroductionStyle::to_str_name(),
                ))),
            ),
        }
    }
}

impl IUsxItemFactory for BookIntroductionTableFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        //         this.register(TableFactory.instance)
    }

    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        BookIntroduction::create(context, parent, attributes)
    }
}
