use crate::context::{UsxContext, UsxNode};
use crate::error::Error;
use crate::singleton::FactoryHandle;
use crate::usx::book_chapter_label::BookChapterLabelFactory;
use crate::usx::book_header::BookHeaderFactory;
use crate::usx::book_identification::BookIdentificationFactory;
use crate::usx::book_introduction::{BookIntroductionFactory, BookIntroductionTableFactory};
use crate::usx::book_introduction_end_titles::BookIntroductionEndTitleFactory;
use crate::usx::book_title::BookTitleFactory;
use crate::usx::cell::CellFactory;
use crate::usx::chapter_end::ChapterEndFactory;
use crate::usx::chapter_start::ChapterStartFactory;
use crate::usx::char::CharFactory;
use crate::usx::cross_reference::CrossReferenceFactory;
use crate::usx::cross_reference_char::CrossReferenceCharFactory;
use crate::usx::figure::FigureFactory;
use crate::usx::footnote::FootnoteFactory;
use crate::usx::footnote_char::FootnoteCharFactory;
use crate::usx::footnote_verse::FootnoteVerseFactory;
use crate::usx::intro_char::IntroCharFactory;
use crate::usx::item::{ItemFactoryFilter, UsxItem};
use crate::usx::list::ListFactory;
use crate::usx::list_char::ListCharFactory;
use crate::usx::milestone::MilestoneFactory;
use crate::usx::optbreak::BreakFactory;
use crate::usx::paragraph::ParagraphFactory;
use crate::usx::reference::ReferenceFactory;
use crate::usx::row::RowFactory;
use crate::usx::table::TableFactory;
use crate::usx::text::TextFactory;
use crate::usx::usx::UsxFactory;
use crate::usx::verse_end::VerseEndFactory;
use crate::usx::verse_start::VerseStartFactory;
use std::collections::HashMap;
use std::string::ToString;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum UsxItemFactory {
    Usx(UsxFactory),
    VerseStart(VerseStartFactory),
    VerseEnd(VerseEndFactory),
    BookIdentification(BookIdentificationFactory),
    BookHeader(BookHeaderFactory),
    BookTitle(BookTitleFactory),
    BookIntroduction(BookIntroductionFactory),
    BookIntroductionTable(BookIntroductionTableFactory),
    BookIntroductionEndTitle(BookIntroductionEndTitleFactory),
    BookChapterLabel(BookChapterLabelFactory),
    Text(TextFactory),
    Break(BreakFactory),
    Char(CharFactory),
    Milestone(MilestoneFactory),
    ChapterStart(ChapterStartFactory),
    ChapterEnd(ChapterEndFactory),
    CrossReference(CrossReferenceFactory),
    CrossReferenceChar(CrossReferenceCharFactory),
    Table(TableFactory),
    Row(RowFactory),
    Cell(CellFactory),
    Paragraph(ParagraphFactory),
    Figure(FigureFactory),
    Reference(ReferenceFactory),
    IntroChar(IntroCharFactory),
    List(ListFactory),
    ListChar(ListCharFactory),
    Footnote(FootnoteFactory),
    FootnoteChar(FootnoteCharFactory),
    FootnoteVerse(FootnoteVerseFactory),
}

pub fn find_child_factory(
    factory: &Arc<UsxItemFactory>,
    context: &mut UsxContext,
    parent: &Arc<Mutex<UsxNode>>,
    tag_name: &String,
    attributes: &HashMap<String, String>,
) -> Result<Arc<UsxItemFactory>, Error> {
    let factories = factory.factories();
    if factories.is_empty() {
        return Err(Error::new(format!(
            "unsupported tag, missing factories: {} in {} - {attributes:?}",
            tag_name,
            factory.tag_name(),
        )));
    }
    let factories = factories.get(&tag_name.to_lowercase());
    if factories.is_none() || factories.unwrap().is_empty() {
        return Err(Error::new(format!(
            "unsupported tag, no factories: {} in {} - {attributes:?}",
            tag_name,
            factory.tag_name(),
        )));
    }
    let factories = factories.unwrap();
    let supported: Vec<_> = factories
        .iter()
        .filter(|f| context.supports(&f.get(), parent, tag_name, attributes, &None))
        .collect();
    if supported.is_empty() {
        return Err(Error::new(format!(
            "zero supported items: {} in {} - {attributes:?}",
            tag_name,
            factory.tag_name()
        )));
    } else if supported.len() > 1 {
        let s: Vec<String> = supported.iter().map(|s| s.tag_name().clone()).collect();
        let f: Vec<String> = supported.iter().map(|s| s.name().to_string()).collect();
        return Err(Error::new(format!(
            "multiple supported items in {} : {s:?} : {f:?} : {attributes:?}",
            factory.tag_name()
        )));
    }
    let f = supported.first().unwrap().get();
    let factory = Arc::clone(&f);
    Ok(factory)
}

impl UsxItemFactory {
    pub fn initialize(&mut self) {
        match self {
            UsxItemFactory::Usx(factory) => factory.initialize(),
            UsxItemFactory::VerseStart(factory) => factory.initialize(),
            UsxItemFactory::VerseEnd(factory) => factory.initialize(),
            UsxItemFactory::Text(factory) => factory.initialize(),
            UsxItemFactory::BookIdentification(factory) => factory.initialize(),
            UsxItemFactory::BookHeader(factory) => factory.initialize(),
            UsxItemFactory::BookIntroduction(factory) => factory.initialize(),
            UsxItemFactory::BookIntroductionTable(factory) => factory.initialize(),
            UsxItemFactory::BookTitle(factory) => factory.initialize(),
            UsxItemFactory::BookIntroductionEndTitle(factory) => factory.initialize(),
            UsxItemFactory::BookChapterLabel(factory) => factory.initialize(),
            UsxItemFactory::Break(factory) => factory.initialize(),
            UsxItemFactory::Char(factory) => factory.initialize(),
            UsxItemFactory::ChapterStart(factory) => factory.initialize(),
            UsxItemFactory::ChapterEnd(factory) => factory.initialize(),
            UsxItemFactory::CrossReference(factory) => factory.initialize(),
            UsxItemFactory::CrossReferenceChar(factory) => factory.initialize(),
            UsxItemFactory::Paragraph(factory) => factory.initialize(),
            UsxItemFactory::Figure(factory) => factory.initialize(),
            UsxItemFactory::Reference(factory) => factory.initialize(),
            UsxItemFactory::Footnote(factory) => factory.initialize(),
            UsxItemFactory::FootnoteVerse(factory) => factory.initialize(),
            UsxItemFactory::FootnoteChar(factory) => factory.initialize(),
            UsxItemFactory::IntroChar(factory) => factory.initialize(),
            UsxItemFactory::List(factory) => factory.initialize(),
            UsxItemFactory::ListChar(factory) => factory.initialize(),
            UsxItemFactory::Milestone(factory) => factory.initialize(),
            UsxItemFactory::Table(factory) => factory.initialize(),
            UsxItemFactory::Row(factory) => factory.initialize(),
            UsxItemFactory::Cell(factory) => factory.initialize(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            UsxItemFactory::Usx(_) => "usx",
            UsxItemFactory::VerseStart(_) => "verse_start",
            UsxItemFactory::VerseEnd(_) => "verse_end",
            UsxItemFactory::Text(_) => "text",
            UsxItemFactory::BookIdentification(_) => "book_ident",
            UsxItemFactory::BookHeader(_) => "book_hdr",
            UsxItemFactory::BookIntroduction(_) => "book_intro",
            UsxItemFactory::BookIntroductionTable(_) => "book_intro_table",
            UsxItemFactory::BookTitle(_) => "book_title",
            UsxItemFactory::BookIntroductionEndTitle(_) => "book_intro_end",
            UsxItemFactory::BookChapterLabel(_) => "book_chap_lbl",
            UsxItemFactory::Break(_) => "break",
            UsxItemFactory::Char(_) => "char",
            UsxItemFactory::ChapterStart(_) => "chap_start",
            UsxItemFactory::ChapterEnd(_) => "chap_end",
            UsxItemFactory::CrossReference(_) => "cross_ref",
            UsxItemFactory::CrossReferenceChar(_) => "cross_ref_char",
            UsxItemFactory::Paragraph(_) => "paragraph",
            UsxItemFactory::Figure(_) => "figure",
            UsxItemFactory::Reference(_) => "reference",
            UsxItemFactory::Footnote(_) => "footnote",
            UsxItemFactory::FootnoteVerse(_) => "footnote_verse",
            UsxItemFactory::FootnoteChar(_) => "footnote_char",
            UsxItemFactory::IntroChar(_) => "intro_char",
            UsxItemFactory::List(_) => "list",
            UsxItemFactory::ListChar(_) => "list_char",
            UsxItemFactory::Milestone(_) => "milestone",
            UsxItemFactory::Table(_) => "table",
            UsxItemFactory::Row(_) => "row",
            UsxItemFactory::Cell(_) => "cell",
        }
    }

    pub fn tag_name(&self) -> String {
        match self {
            UsxItemFactory::Usx(factory) => factory.tag_name(),
            UsxItemFactory::VerseStart(factory) => factory.tag_name(),
            UsxItemFactory::VerseEnd(factory) => factory.tag_name(),
            UsxItemFactory::Text(factory) => factory.tag_name(),
            UsxItemFactory::BookIdentification(factory) => factory.tag_name(),
            UsxItemFactory::BookHeader(factory) => factory.tag_name(),
            UsxItemFactory::BookIntroduction(factory) => factory.tag_name(),
            UsxItemFactory::BookIntroductionTable(factory) => factory.tag_name(),
            UsxItemFactory::BookTitle(factory) => factory.tag_name(),
            UsxItemFactory::BookIntroductionEndTitle(factory) => factory.tag_name(),
            UsxItemFactory::BookChapterLabel(factory) => factory.tag_name(),
            UsxItemFactory::Break(factory) => factory.tag_name(),
            UsxItemFactory::Char(factory) => factory.tag_name(),
            UsxItemFactory::ChapterStart(factory) => factory.tag_name(),
            UsxItemFactory::ChapterEnd(factory) => factory.tag_name(),
            UsxItemFactory::CrossReference(factory) => factory.tag_name(),
            UsxItemFactory::CrossReferenceChar(factory) => factory.tag_name(),
            UsxItemFactory::Paragraph(factory) => factory.tag_name(),
            UsxItemFactory::Figure(factory) => factory.tag_name(),
            UsxItemFactory::Reference(factory) => factory.tag_name(),
            UsxItemFactory::Footnote(factory) => factory.tag_name(),
            UsxItemFactory::FootnoteVerse(factory) => factory.tag_name(),
            UsxItemFactory::FootnoteChar(factory) => factory.tag_name(),
            UsxItemFactory::IntroChar(factory) => factory.tag_name(),
            UsxItemFactory::List(factory) => factory.tag_name(),
            UsxItemFactory::ListChar(factory) => factory.tag_name(),
            UsxItemFactory::Milestone(factory) => factory.tag_name(),
            UsxItemFactory::Table(factory) => factory.tag_name(),
            UsxItemFactory::Row(factory) => factory.tag_name(),
            UsxItemFactory::Cell(factory) => factory.tag_name(),
        }
        .clone()
    }

    pub fn factories(&self) -> &HashMap<String, Vec<&'static FactoryHandle>> {
        match self {
            UsxItemFactory::Usx(factory) => factory.factories(),
            UsxItemFactory::VerseStart(factory) => factory.factories(),
            UsxItemFactory::VerseEnd(factory) => factory.factories(),
            UsxItemFactory::Text(factory) => factory.factories(),
            UsxItemFactory::BookIdentification(factory) => factory.factories(),
            UsxItemFactory::BookHeader(factory) => factory.factories(),
            UsxItemFactory::BookIntroduction(factory) => factory.factories(),
            UsxItemFactory::BookIntroductionTable(factory) => factory.factories(),
            UsxItemFactory::BookTitle(factory) => factory.factories(),
            UsxItemFactory::BookIntroductionEndTitle(factory) => factory.factories(),
            UsxItemFactory::BookChapterLabel(factory) => factory.factories(),
            UsxItemFactory::Break(factory) => factory.factories(),
            UsxItemFactory::Char(factory) => factory.factories(),
            UsxItemFactory::ChapterStart(factory) => factory.factories(),
            UsxItemFactory::ChapterEnd(factory) => factory.factories(),
            UsxItemFactory::CrossReference(factory) => factory.factories(),
            UsxItemFactory::CrossReferenceChar(factory) => factory.factories(),
            UsxItemFactory::Paragraph(factory) => factory.factories(),
            UsxItemFactory::Figure(factory) => factory.factories(),
            UsxItemFactory::Reference(factory) => factory.factories(),
            UsxItemFactory::Footnote(factory) => factory.factories(),
            UsxItemFactory::FootnoteVerse(factory) => factory.factories(),
            UsxItemFactory::FootnoteChar(factory) => factory.factories(),
            UsxItemFactory::IntroChar(factory) => factory.factories(),
            UsxItemFactory::List(factory) => factory.factories(),
            UsxItemFactory::ListChar(factory) => factory.factories(),
            UsxItemFactory::Milestone(factory) => factory.factories(),
            UsxItemFactory::Table(factory) => factory.factories(),
            UsxItemFactory::Row(factory) => factory.factories(),
            UsxItemFactory::Cell(factory) => factory.factories(),
        }
    }

    pub fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>> {
        match self {
            UsxItemFactory::Usx(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::VerseStart(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::VerseEnd(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::BookIdentification(factory) => {
                factory.create(context, parent, attributes)
            }
            UsxItemFactory::BookHeader(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::BookTitle(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::BookIntroduction(factory) => {
                factory.create(context, parent, attributes)
            }
            UsxItemFactory::BookIntroductionTable(factory) => {
                factory.create(context, parent, attributes)
            }
            UsxItemFactory::BookIntroductionEndTitle(factory) => {
                factory.create(context, parent, attributes)
            }
            UsxItemFactory::BookChapterLabel(factory) => {
                factory.create(context, parent, attributes)
            }
            UsxItemFactory::Text(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::Break(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::Char(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::ChapterStart(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::ChapterEnd(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::CrossReference(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::CrossReferenceChar(factory) => {
                factory.create(context, parent, attributes)
            }
            UsxItemFactory::Paragraph(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::Figure(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::Reference(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::Footnote(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::FootnoteVerse(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::FootnoteChar(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::IntroChar(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::List(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::ListChar(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::Milestone(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::Table(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::Row(factory) => factory.create(context, parent, attributes),
            UsxItemFactory::Cell(factory) => factory.create(context, parent, attributes),
        }
    }

    pub fn supports(
        &self,
        tag_name: &str,
        attributes: &HashMap<String, String>,
        progression: &Option<usize>,
    ) -> bool {
        match self {
            UsxItemFactory::Usx(factory) => factory.supports(tag_name, attributes, progression),
            UsxItemFactory::VerseStart(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::VerseEnd(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::Text(factory) => factory.supports(tag_name, attributes, progression),
            UsxItemFactory::BookIdentification(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::BookHeader(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::BookTitle(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::BookIntroduction(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::BookIntroductionTable(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::BookIntroductionEndTitle(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::BookChapterLabel(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::Break(factory) => factory.supports(tag_name, attributes, progression),
            UsxItemFactory::Char(factory) => factory.supports(tag_name, attributes, progression),
            UsxItemFactory::ChapterStart(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::ChapterEnd(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::CrossReference(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::CrossReferenceChar(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::Paragraph(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::Figure(factory) => factory.supports(tag_name, attributes, progression),
            UsxItemFactory::Reference(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::Footnote(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::FootnoteVerse(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::FootnoteChar(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::IntroChar(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::List(factory) => factory.supports(tag_name, attributes, progression),
            UsxItemFactory::ListChar(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::Milestone(factory) => {
                factory.supports(tag_name, attributes, progression)
            }
            UsxItemFactory::Table(factory) => factory.supports(tag_name, attributes, progression),
            UsxItemFactory::Row(factory) => factory.supports(tag_name, attributes, progression),
            UsxItemFactory::Cell(factory) => factory.supports(tag_name, attributes, progression),
        }
    }
}

#[derive(Debug)]
pub struct BaseFactory {
    #[allow(dead_code)]
    tag_name: String,
    pub initialized: bool,
    factories: HashMap<String, Vec<&'static FactoryHandle>>,
    filter: Option<ItemFactoryFilter>,
}

pub fn new_base_factory(tag_name: &str, filter: Option<ItemFactoryFilter>) -> BaseFactory {
    BaseFactory {
        tag_name: tag_name.to_string(),
        initialized: false,
        factories: Default::default(),
        filter,
    }
}

impl BaseFactory {
    pub fn new(tag_name: &str, filter: Option<ItemFactoryFilter>) -> Self {
        Self {
            tag_name: tag_name.to_string(),
            initialized: false,
            factories: Default::default(),
            filter,
        }
    }
}

#[allow(dead_code)]
pub trait IUsxItemFactory {
    fn base_factory(&self) -> &BaseFactory;
    fn base_factory_mut(&mut self) -> &mut BaseFactory;
    fn initialized(&self) -> bool {
        self.base_factory().initialized
    }
    fn factories(&self) -> &HashMap<String, Vec<&'static FactoryHandle>> {
        &self.base_factory().factories
    }
    fn factories_mut(&mut self) -> &mut HashMap<String, Vec<&'static FactoryHandle>> {
        &mut self.base_factory_mut().factories
    }
    fn filter(&self) -> &Option<ItemFactoryFilter> {
        &self.base_factory().filter
    }
    fn tag_name(&self) -> &String {
        &self.base_factory().tag_name
    }
    fn initialize(&mut self) {
        if self.initialized() {
            return;
        }
        let base = self.base_factory_mut();
        base.initialized = true;
        self.on_initialize();
    }
    fn register(&mut self, factory: &'static FactoryHandle) {
        let all_factories = self.factories_mut();
        let tag_name = factory.tag_name();
        let mut factories = all_factories.get_mut(tag_name);
        if factories.is_none() {
            all_factories.insert(tag_name.clone(), Vec::new());
            factories = all_factories.get_mut(tag_name);
        }
        factories.unwrap().push(factory);
    }
    fn on_initialize(&mut self);
    fn create(
        &self,
        context: &mut UsxContext,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        attributes: &HashMap<String, String>,
    ) -> Arc<Mutex<UsxItem>>;
    fn supports(
        &self,
        tag_name: &str,
        attributes: &HashMap<String, String>,
        progression: &Option<usize>,
    ) -> bool {
        let filter = self.filter();
        if filter.is_none() {
            return true;
        }
        filter
            .as_ref()
            .unwrap()
            .supports(tag_name, attributes, progression)
    }
}
