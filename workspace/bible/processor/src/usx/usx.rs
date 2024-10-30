use crate::context::UsxContext;
use crate::singleton::{FactoryHandle, FactorySingleton};
use crate::usx::book_chapter_label::BookChapterLabelFactory;
use crate::usx::book_header::BookHeaderFactory;
use crate::usx::book_identification::BookIdentificationFactory;
use crate::usx::book_introduction::{BookIntroductionFactory, BookIntroductionTableFactory};
use crate::usx::book_introduction_end_titles::BookIntroductionEndTitleFactory;
use crate::usx::book_title::BookTitleFactory;
use crate::usx::chapter_end::ChapterEndFactory;
use crate::usx::chapter_start::ChapterStartFactory;
use crate::usx::cross_reference::CrossReferenceFactory;
use crate::usx::factory::{BaseFactory, IUsxItemFactory, UsxItemFactory};
use crate::usx::footnote::FootnoteFactory;
use crate::usx::item::{UsxItem, UsxItemContainer};
use crate::usx::list::ListFactory;
use crate::usx::paragraph::ParagraphFactory;
use crate::usx::table::TableFactory;
use crate::usx::text::TextFactory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Usx {
    pub container: UsxItemContainer,
}

impl Usx {
    pub fn create(context: &mut UsxContext) -> Arc<Mutex<UsxItem>> {
        Arc::new(Mutex::new(UsxItem::Usx(Self {
            container: UsxItemContainer::new(context),
        })))
    }
}

#[derive(Debug)]
pub struct UsxFactory {
    base: BaseFactory,
}

static mut INSTANCE: FactorySingleton = FactorySingleton::new();

fn factory() -> Arc<UsxItemFactory> {
    let factory = UsxItemFactory::Usx(UsxFactory {
        base: BaseFactory::new("usx", None),
    });
    Arc::new(factory)
}

impl UsxFactory {
    pub fn get() -> &'static FactoryHandle {
        unsafe {
            INSTANCE.initialize(factory);
            INSTANCE.get()
        }
    }
}

impl IUsxItemFactory for UsxFactory {
    fn base_factory(&self) -> &BaseFactory {
        &self.base
    }

    fn base_factory_mut(&mut self) -> &mut BaseFactory {
        &mut self.base
    }

    fn on_initialize(&mut self) {
        self.register(BookIdentificationFactory::get());
        self.register(BookHeaderFactory::get());
        self.register(BookTitleFactory::get());
        self.register(BookIntroductionFactory::get());
        self.register(BookIntroductionTableFactory::get());
        self.register(BookIntroductionEndTitleFactory::get());
        self.register(BookChapterLabelFactory::get());
        self.register(ChapterStartFactory::get());
        self.register(ChapterEndFactory::get());
        self.register(ParagraphFactory::get());
        self.register(ListFactory::get());
        self.register(TableFactory::get());
        self.register(FootnoteFactory::get());
        self.register(CrossReferenceFactory::get());
        //         // this.register(Sidebar)
        self.register(TextFactory::get());
    }

    fn create(
        &self,
        context: &mut UsxContext,
        _: &Option<Arc<Mutex<UsxItem>>>,
        _: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        Usx::create(context)
    }
}
