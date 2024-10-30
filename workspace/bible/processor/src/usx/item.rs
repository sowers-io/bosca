use crate::book::Book;
use crate::context::UsxContext;
use crate::html_context::HtmlContext;
use crate::string_context::{StringContext, DEFAULT_CONTEXT};
use crate::usx::book_chapter_label::BookChapterLabel;
use crate::usx::book_header::BookHeader;
use crate::usx::book_identification::BookIdentification;
use crate::usx::book_introduction::BookIntroduction;
use crate::usx::book_introduction_end_titles::BookIntroductionEndTitle;
use crate::usx::book_title::BookTitle;
use crate::usx::cell::Cell;
use crate::usx::chapter::Chapter;
use crate::usx::chapter_end::ChapterEnd;
use crate::usx::chapter_start::ChapterStart;
use crate::usx::char::Char;
use crate::usx::cross_reference::CrossReference;
use crate::usx::cross_reference_char::CrossReferenceChar;
use crate::usx::figure::Figure;
use crate::usx::footnote::Footnote;
use crate::usx::footnote_char::FootnoteChar;
use crate::usx::footnote_verse::FootnoteVerse;
use crate::usx::intro_char::IntroChar;
use crate::usx::list::List;
use crate::usx::list_char::ListChar;
use crate::usx::milestone::Milestone;
use crate::usx::optbreak::Break;
use crate::usx::paragraph::Paragraph;
use crate::usx::position::Position;
use crate::usx::reference::Reference;
use crate::usx::row::Row;
use crate::usx::table::Table;
use crate::usx::text::Text;
use crate::usx::usx::Usx;
use crate::usx::verse_end::VerseEnd;
use crate::usx::verse_start::VerseStart;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::{Arc, Mutex};

#[allow(clippy::wrong_self_convention)]
pub trait IUsxItem {
    fn verse(&self) -> Option<String>;
    fn usfm(&self) -> Option<String>;
    fn position(&self) -> Option<Position>;
    fn children(&self) -> Option<&Vec<Arc<Mutex<UsxItem>>>>;
    fn add_child(&mut self, child: Arc<Mutex<UsxItem>>);
    fn html_class(&self) -> Option<String>;
    fn html_attributes(&self) -> Option<HashMap<String, String>>;
    fn to_html(&self, context: &mut HtmlContext) -> String;
    fn to_string(&self, context: &Option<StringContext>) -> String;
}

pub enum UsxItem {
    Usx(Usx),
    Break(Break),
    Footnote(Footnote),
    FootnoteChar(FootnoteChar),
    Char(Char),
    CrossReference(CrossReference),
    FootnoteVerse(FootnoteVerse),
    CrossReferenceChar(CrossReferenceChar),
    VerseStart(VerseStart),
    VerseEnd(VerseEnd),
    ChapterStart(ChapterStart),
    ChapterEnd(ChapterEnd),
    Chapter(Chapter),
    Table(Table),
    Row(Row),
    Cell(Cell),
    Book(Book),
    BookTitle(BookTitle),
    BookIntroduction(BookIntroduction),
    BookIntroductionEndTitle(BookIntroductionEndTitle),
    BookChapterLabel(BookChapterLabel),
    BookIdentification(BookIdentification),
    BookHeader(BookHeader),
    Paragraph(Paragraph),
    Figure(Figure),
    Text(Text),
    IntroChar(IntroChar),
    ListChar(ListChar),
    Milestone(Milestone),
    Reference(Reference),
    List(List),
    VerseItems(UsxVerseItems),
    Unknown,
}

impl UsxItem {
    fn to_string_internal(&self, context: &Option<StringContext>) -> String {
        let ctx = if context.is_none() {
            &DEFAULT_CONTEXT
        } else {
            context.as_ref().unwrap()
        };
        match self {
            UsxItem::Text(item) => {
                if !ctx.include_new_lines {
                    let re = Regex::new(r"/\r?\n/g").unwrap();
                    let result = re.replace_all(item.text.as_ref().unwrap(), "");
                    return result.to_string();
                }
                item.text.as_ref().unwrap().to_string()
            }
            _ => {
                if let Some(items) = self.children() {
                    let mut result = String::new();
                    for child in items {
                        result.push_str(&child.lock().unwrap().to_string(context));
                    }
                    result
                } else {
                    "".to_string()
                }
            }
        }
    }
}

impl IUsxItem for UsxItem {
    fn verse(&self) -> Option<String> {
        match self {
            UsxItem::Usx(item) => item.container.verse.clone(),
            UsxItem::Text(item) => item.verse.clone(),
            UsxItem::Break(item) => item.verse.clone(),
            UsxItem::Footnote(item) => item.container.verse.clone(),
            UsxItem::FootnoteChar(item) => item.container.verse.clone(),
            UsxItem::Char(item) => item.container.verse.clone(),
            UsxItem::CrossReference(item) => item.container.verse.clone(),
            UsxItem::FootnoteVerse(item) => item.container.verse.clone(),
            UsxItem::CrossReferenceChar(item) => item.container.verse.clone(),
            UsxItem::VerseStart(item) => Some(item.number.clone()),
            UsxItem::VerseEnd(item) => item.verse.clone(),
            UsxItem::ChapterStart(item) => item.verse.clone(),
            UsxItem::ChapterEnd(item) => item.verse.clone(),
            UsxItem::Chapter(item) => item.container.verse.clone(),
            UsxItem::Table(item) => item.container.verse.clone(),
            UsxItem::Row(item) => item.container.verse.clone(),
            UsxItem::Cell(item) => item.container.verse.clone(),
            UsxItem::BookTitle(item) => item.container.verse.clone(),
            UsxItem::BookIntroduction(item) => item.container.verse.clone(),
            UsxItem::BookIntroductionEndTitle(item) => item.container.verse.clone(),
            UsxItem::BookChapterLabel(item) => item.container.verse.clone(),
            UsxItem::BookIdentification(item) => item.container.verse.clone(),
            UsxItem::BookHeader(item) => item.container.verse.clone(),
            UsxItem::Paragraph(item) => item.container.verse.clone(),
            UsxItem::Figure(item) => item.container.verse.clone(),
            UsxItem::IntroChar(item) => item.container.verse.clone(),
            UsxItem::ListChar(item) => item.container.verse.clone(),
            UsxItem::Reference(item) => item.container.verse.clone(),
            UsxItem::List(item) => item.container.verse.clone(),
            UsxItem::VerseItems(item) => item.verse.clone(),
            _ => panic!("cannot have children"),
        }
    }

    fn usfm(&self) -> Option<String> {
        match self {
            UsxItem::Chapter(item) => Some(item.usfm.clone()),
            _ => todo!(),
        }
    }

    fn position(&self) -> Option<Position> {
        match self {
            UsxItem::Chapter(item) => Some(
                item.container
                    .position
                    .as_ref()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .clone(),
            ),
            _ => todo!(),
        }
    }

    fn children(&self) -> Option<&Vec<Arc<Mutex<UsxItem>>>> {
        match self {
            UsxItem::Usx(item) => Some(&item.container.items),
            UsxItem::Footnote(item) => Some(&item.container.items),
            UsxItem::FootnoteChar(item) => Some(&item.container.items),
            UsxItem::Char(item) => Some(&item.container.items),
            UsxItem::CrossReference(item) => Some(&item.container.items),
            UsxItem::FootnoteVerse(item) => Some(&item.container.items),
            UsxItem::CrossReferenceChar(item) => Some(&item.container.items),
            UsxItem::Chapter(item) => Some(&item.container.items),
            UsxItem::Table(item) => Some(&item.container.items),
            UsxItem::Row(item) => Some(&item.container.items),
            UsxItem::Cell(item) => Some(&item.container.items),
            UsxItem::BookTitle(item) => Some(&item.container.items),
            UsxItem::BookIntroduction(item) => Some(&item.container.items),
            UsxItem::BookIntroductionEndTitle(item) => Some(&item.container.items),
            UsxItem::BookChapterLabel(item) => Some(&item.container.items),
            UsxItem::BookIdentification(item) => Some(&item.container.items),
            UsxItem::BookHeader(item) => Some(&item.container.items),
            UsxItem::Paragraph(item) => Some(&item.container.items),
            UsxItem::Figure(item) => Some(&item.container.items),
            UsxItem::IntroChar(item) => Some(&item.container.items),
            UsxItem::ListChar(item) => Some(&item.container.items),
            UsxItem::Reference(item) => Some(&item.container.items),
            UsxItem::List(item) => Some(&item.container.items),
            UsxItem::VerseItems(item) => Some(&item.items),
            _ => None,
        }
    }

    fn add_child(&mut self, child: Arc<Mutex<UsxItem>>) {
        match self {
            UsxItem::Usx(item) => item.container.add(child),
            UsxItem::Footnote(item) => item.container.add(child),
            UsxItem::FootnoteChar(item) => item.container.add(child),
            UsxItem::Char(item) => item.container.add(child),
            UsxItem::CrossReference(item) => item.container.add(child),
            UsxItem::FootnoteVerse(item) => item.container.add(child),
            UsxItem::CrossReferenceChar(item) => item.container.add(child),
            UsxItem::Chapter(item) => item.container.add(child),
            UsxItem::Table(item) => item.container.add(child),
            UsxItem::Row(item) => item.container.add(child),
            UsxItem::Cell(item) => item.container.add(child),
            UsxItem::BookTitle(item) => item.container.add(child),
            UsxItem::BookIntroduction(item) => item.container.add(child),
            UsxItem::BookIntroductionEndTitle(item) => item.container.add(child),
            UsxItem::BookChapterLabel(item) => item.container.add(child),
            UsxItem::BookIdentification(item) => item.container.add(child),
            UsxItem::BookHeader(item) => item.container.add(child),
            UsxItem::Paragraph(item) => item.container.add(child),
            UsxItem::Figure(item) => item.container.add(child),
            UsxItem::IntroChar(item) => item.container.add(child),
            UsxItem::ListChar(item) => item.container.add(child),
            UsxItem::Reference(item) => item.container.add(child),
            UsxItem::List(item) => item.container.add(child),
            UsxItem::VerseItems(item) => item.add_item(&child),
            _ => panic!("cannot have children"),
        }
    }

    fn html_class(&self) -> Option<String> {
        match self {
            UsxItem::Chapter(item) => Some(format!("chapter c{}", item.number)),
            UsxItem::Footnote(item) => Some(format!("{}", item.style)),
            UsxItem::FootnoteChar(item) => Some(format!("{}", item.style)),
            UsxItem::Char(item) => Some(format!("{}", item.style)),
            UsxItem::CrossReference(item) => Some(format!("{}", item.style)),
            UsxItem::FootnoteVerse(item) => Some(format!("{}", item.style)),
            UsxItem::CrossReferenceChar(item) => Some(format!("{}", item.style)),
            UsxItem::BookTitle(item) => Some(format!("{}", item.style)),
            UsxItem::BookIntroduction(item) => Some(format!("book-introduction {}", item.style)),
            UsxItem::BookIntroductionEndTitle(item) => Some(format!("{}", item.style)),
            UsxItem::BookChapterLabel(item) => Some(format!("{}", item.style)),
            UsxItem::BookIdentification(_) => Some("book-identification".to_string()),
            UsxItem::BookHeader(item) => Some(format!("{}", item.style)),
            UsxItem::Paragraph(item) => Some(format!("{}", item.style)),
            UsxItem::Figure(item) => Some(item.style.to_string()),
            UsxItem::IntroChar(item) => Some(format!("{}", item.style)),
            UsxItem::ListChar(item) => Some(format!("{}", item.style)),
            UsxItem::List(item) => Some(format!("{}", item.style)),
            UsxItem::VerseStart(item) => Some(format!("verse number v{}", item.number)),
            UsxItem::Text(item) => {
                if let Some(verse) = &item.verse {
                    Some(format!("verse v{}", verse))
                } else {
                    Some("verse".to_string())
                }
            }
            _ => None,
        }
    }

    fn html_attributes(&self) -> Option<HashMap<String, String>> {
        match self {
            UsxItem::VerseStart(item) => Some(item.html_attributes()),
            UsxItem::Text(item) => Some(item.html_attributes()),
            UsxItem::CrossReference(item) => Some(item.html_attributes()),
            UsxItem::Chapter(item) => Some(item.html_attributes()),
            UsxItem::BookIdentification(item) => Some(item.html_attributes()),
            _ => None,
        }
    }

    fn to_html(&self, context: &mut HtmlContext) -> String {
        match self {
            UsxItem::VerseStart(start) => {
                if !context.include_verse_numbers {
                    "".to_string()
                } else {
                    context.render("span", self, &Some(start.number.clone()))
                }
            }
            UsxItem::CrossReference(_) => {
                if !context.include_cross_references {
                    "".to_string()
                } else {
                    context.render("div", self, &None)
                }
            }
            UsxItem::CrossReferenceChar(_) => {
                if !context.include_cross_references {
                    "".to_string()
                } else {
                    context.render("div", self, &None)
                }
            }
            UsxItem::Footnote(_) => {
                if !context.include_footnotes {
                    "".to_string()
                } else {
                    context.render("div", self, &None)
                }
            }
            UsxItem::FootnoteChar(_) => {
                if !context.include_footnotes {
                    "".to_string()
                } else {
                    context.render("div", self, &None)
                }
            }
            UsxItem::FootnoteVerse(_) => {
                if !context.include_footnotes {
                    "".to_string()
                } else {
                    context.render("div", self, &None)
                }
            }
            UsxItem::Text(item) => context.render("span", self, &item.text),
            UsxItem::Char(_) => context.render("span", self, &None),
            UsxItem::Break(_) => context.render("br", self, &None),
            UsxItem::Milestone(_) => context.render("milestone", self, &None),
            UsxItem::Paragraph(_) => context.render("p", self, &None),
            UsxItem::Table(_) => context.render("table", self, &None),
            UsxItem::Row(_) => context.render("tr", self, &None),
            UsxItem::Cell(_) => context.render("td", self, &None),
            _ => {
                if self.children().is_some() {
                    context.render("div", self, &None)
                } else {
                    "".to_string()
                }
            }
        }
    }

    fn to_string(&self, context: &Option<StringContext>) -> String {
        let ctx = if context.is_none() {
            &DEFAULT_CONTEXT
        } else {
            context.as_ref().unwrap()
        };
        match self {
            UsxItem::VerseStart(start) => {
                if !ctx.include_verse_numbers {
                    " ".to_string()
                } else {
                    format!(" {}. ", start.number)
                }
            }
            UsxItem::CrossReference(_) => {
                if !ctx.include_cross_references {
                    "".to_string()
                } else {
                    self.to_string_internal(context)
                }
            }
            UsxItem::CrossReferenceChar(_) => {
                if !ctx.include_cross_references {
                    "".to_string()
                } else {
                    self.to_string_internal(context)
                }
            }
            UsxItem::Footnote(_) => {
                if !ctx.include_footnotes {
                    "".to_string()
                } else {
                    self.to_string_internal(context)
                }
            }
            UsxItem::FootnoteChar(_) => {
                if !ctx.include_footnotes {
                    "".to_string()
                } else {
                    self.to_string_internal(context)
                }
            }
            UsxItem::FootnoteVerse(_) => {
                if !ctx.include_footnotes {
                    "".to_string()
                } else {
                    self.to_string_internal(context)
                }
            }
            _ => self.to_string_internal(context),
        }
    }
}

pub struct UsxItemContainer {
    pub items: Vec<Arc<Mutex<UsxItem>>>,
    pub position: Option<Arc<Mutex<Position>>>,
    pub verse: Option<String>,
}

impl UsxItemContainer {
    pub fn new(context: &UsxContext) -> Self {
        Self {
            items: Vec::new(),
            position: context.position(),
            verse: None,
        }
    }

    pub fn add(&mut self, item: Arc<Mutex<UsxItem>>) {
        self.items.push(item)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ItemFactoryFilter {
    Style(StyleFactoryFilter),
    EndId(EndIdFactoryFilter),
    Negate(NegateFactoryFilter),
    Code(CodeFactoryFilter),
}

impl ItemFactoryFilter {
    pub fn supports(
        &self,
        tag_name: &str,
        attributes: &HashMap<String, String>,
        progression: &Option<usize>,
    ) -> bool {
        match self {
            ItemFactoryFilter::Style(filter) => filter.supports(tag_name, attributes, progression),
            ItemFactoryFilter::Code(filter) => filter.supports(tag_name, attributes, progression),
            ItemFactoryFilter::Negate(filter) => filter.supports(tag_name, attributes, progression),
            ItemFactoryFilter::EndId(filter) => filter.supports(tag_name, attributes, progression),
        }
    }
}

pub trait IItemFactoryFilter {
    fn supports(
        &self,
        tag_name: &str,
        attributes: &HashMap<String, String>,
        progression: &Option<usize>,
    ) -> bool;
}

#[derive(Debug, Eq, PartialEq)]
pub struct StyleFactoryFilter {
    styles: Vec<String>,
}

impl StyleFactoryFilter {
    pub fn new(styles: Vec<String>) -> Self {
        Self { styles }
    }
}

impl IItemFactoryFilter for StyleFactoryFilter {
    fn supports(
        &self,
        _: &str,
        attributes: &HashMap<String, String>,
        progression: &Option<usize>,
    ) -> bool {
        let style = attributes.get("style");
        if style.is_none() {
            return false;
        }
        if progression.is_some() {
            let check = self.styles.get(progression.unwrap()).unwrap();
            let eq = check.eq(style.unwrap());
            return eq;
        }
        self.styles.contains(style.unwrap())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CodeFactoryFilter {
    codes: Vec<String>,
}

impl CodeFactoryFilter {
    pub fn new(codes: Vec<String>) -> Self {
        Self { codes }
    }
}

impl IItemFactoryFilter for CodeFactoryFilter {
    fn supports(&self, _: &str, attributes: &HashMap<String, String>, _: &Option<usize>) -> bool {
        let code = attributes.get("code");
        if code.is_none() {
            return false;
        }
        let code = code.unwrap();
        let mut chap = "Chap".to_string();
        let _ = chap.write_str(code.as_str());
        self.codes.contains(&chap)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct EndIdFactoryFilter {}

impl IItemFactoryFilter for EndIdFactoryFilter {
    fn supports(&self, _: &str, attributes: &HashMap<String, String>, _: &Option<usize>) -> bool {
        let key = "eid".to_string();
        attributes.contains_key(&key)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct NegateFactoryFilter {
    filter: Arc<ItemFactoryFilter>,
}

impl NegateFactoryFilter {
    pub fn new(filter: Arc<ItemFactoryFilter>) -> Self {
        Self { filter }
    }
}

impl IItemFactoryFilter for NegateFactoryFilter {
    fn supports(
        &self,
        tag_name: &str,
        attributes: &HashMap<String, String>,
        progression: &Option<usize>,
    ) -> bool {
        !self.filter.supports(tag_name, attributes, progression)
    }
}

pub struct UsxVerseItems {
    pub usfm: String,
    pub verse: Option<String>,
    pub position: Arc<Mutex<Position>>,
    items: Vec<Arc<Mutex<UsxItem>>>,
}

impl UsxVerseItems {
    pub fn new(
        usfm: String,
        verse_start: &VerseStart,
        verse_start_item: &Arc<Mutex<UsxItem>>,
        position: &Arc<Mutex<Position>>,
    ) -> Self {
        Self {
            usfm,
            verse: Some(verse_start.number.clone()),
            position: Arc::clone(position),
            items: vec![Arc::clone(verse_start_item)],
        }
    }

    pub fn html_class() -> String {
        "verses".to_string()
    }

    pub fn add_item(&mut self, item: &Arc<Mutex<UsxItem>>) {
        self.items.push(Arc::clone(item))
    }

    pub fn to_string(&self, context: &Option<StringContext>) -> String {
        let mut str = String::new();
        for item in &self.items {
            let s = item.lock().unwrap().to_string(context);
            str.push_str(s.as_str());
        }
        str
    }
}
