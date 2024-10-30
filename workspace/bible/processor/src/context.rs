use crate::book::Book;
use crate::usx::book_chapter_label::BookChapterLabelFactory;
use crate::usx::book_header::BookHeaderFactory;
use crate::usx::book_identification::BookIdentificationFactory;
use crate::usx::book_introduction::BookIntroductionFactory;
use crate::usx::book_introduction_end_titles::BookIntroductionEndTitleFactory;
use crate::usx::book_title::BookTitleFactory;
use crate::usx::chapter::Chapter;
use crate::usx::factory::{find_child_factory, UsxItemFactory};
use crate::usx::item::{IUsxItem, UsxItem, UsxVerseItems};
use crate::usx::position::Position;
use crate::usx::styles::{
    BookChapterLabelStyle, BookHeaderStyle, BookIntroductionEndTitleStyle, BookIntroductionStyle,
    BookTitleStyle,
};
use crate::usx::usx::UsxFactory;
use crate::usx::verse_start::VerseStart;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[derive(Clone, Eq, PartialEq, Hash)]
enum CompletedBookTag {
    Identification,
    Headers,
    Titles,
    Introduction,
    EndIntroductionTitles,
    Label,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum BookTagResult {
    Supported,
    Unsupported,
    Unknown,
}

pub struct UsxNode {
    factory: Arc<UsxItemFactory>,
    item: Arc<Mutex<UsxItem>>,
}

#[derive(Clone)]
struct ContextTag {
    factory: Arc<UsxItemFactory>,
    tag: CompletedBookTag,
    max_styles: usize,
}

pub struct UsxContext {
    book: Arc<Mutex<Book>>,
    chapters: Vec<Arc<Mutex<UsxItem>>>,
    items: Vec<Arc<Mutex<UsxNode>>>,
    completed: Vec<CompletedBookTag>,
    positions: Vec<Arc<Mutex<Position>>>,
    last_factory: Arc<UsxItemFactory>,
    progression: bool,
    verses: Vec<Arc<Mutex<UsxVerseItems>>>,
    verse_items: Vec<Arc<Mutex<UsxVerseItems>>>,
    tags: Vec<ContextTag>,
}

impl UsxContext {
    pub fn new(book: Arc<Mutex<Book>>) -> Self {
        Self {
            book,
            chapters: vec![],
            items: vec![],
            completed: vec![],
            positions: vec![],
            last_factory: BookIdentificationFactory::get().get(),
            progression: true,
            verses: vec![],
            verse_items: vec![],
            tags: vec![
                ContextTag {
                    factory: BookIdentificationFactory::get().get(),
                    tag: CompletedBookTag::Identification,
                    max_styles: 1,
                },
                ContextTag {
                    factory: BookHeaderFactory::get().get(),
                    tag: CompletedBookTag::Headers,
                    max_styles: BookHeaderStyle::to_str_name().len(),
                },
                ContextTag {
                    factory: BookTitleFactory::get().get(),
                    tag: CompletedBookTag::Titles,
                    max_styles: BookTitleStyle::to_str_name().len(),
                },
                ContextTag {
                    factory: BookIntroductionFactory::get().get(),
                    tag: CompletedBookTag::Introduction,
                    max_styles: BookIntroductionStyle::to_str_name().len(),
                },
                ContextTag {
                    factory: BookIntroductionEndTitleFactory::get().get(),
                    tag: CompletedBookTag::EndIntroductionTitles,
                    max_styles: BookIntroductionEndTitleStyle::to_str_name().len(),
                },
                ContextTag {
                    factory: BookChapterLabelFactory::get().get(),
                    tag: CompletedBookTag::Label,
                    max_styles: BookChapterLabelStyle::to_str_name().len(),
                },
            ],
        }
    }

    pub fn position(&self) -> Option<Arc<Mutex<Position>>> {
        let last = self.positions.last();
        last?;
        let pos = Arc::clone(last.unwrap());
        Some(pos)
    }

    pub fn push_verse(
        &mut self,
        book_chapter_usfm: &String,
        verse_start: &VerseStart,
        verse_start_item: &Arc<Mutex<UsxItem>>,
        position: &Arc<Mutex<Position>>,
    ) {
        let items = Arc::new(Mutex::new(UsxVerseItems::new(
            format!("{}.{}", book_chapter_usfm, verse_start.number.clone()),
            verse_start,
            verse_start_item,
            position,
        )));
        self.verses.push(items)
    }

    pub fn pop_verse(&mut self) -> Option<Arc<Mutex<UsxVerseItems>>> {
        self.verses.pop()
    }

    pub fn add_verse_item(
        &mut self,
        parent: &Option<Arc<Mutex<UsxItem>>>,
        item: Arc<Mutex<UsxItem>>,
    ) {
        if self.verses.is_empty() {
            return;
        }
        let verses = self.verses.last_mut();
        if verses.is_none() {
            return;
        }
        let verses = verses.unwrap();
        if parent.is_none()
            || !parent
                .as_ref()
                .unwrap()
                .try_lock()
                .unwrap()
                .verse()
                .eq(&verses.try_lock().unwrap().verse)
        {
            let mut v = verses.try_lock().unwrap();
            v.add_item(&item);
        }
    }

    pub fn verse(&self) -> Option<String> {
        self.verses
            .last()
            .map(|i| i.try_lock().unwrap().verse.clone())
            .unwrap_or(None)
    }

    pub fn supports(
        &mut self,
        factory: &Arc<UsxItemFactory>,
        parent: &Arc<Mutex<UsxNode>>,
        tag_name: &String,
        attributes: &HashMap<String, String>,
        progression: &Option<usize>,
    ) -> bool {
        if tag_name.to_lowercase() == "chapter" {
            self.progression = false
        }
        if !self.progression
            || tag_name.to_lowercase() == "#text"
            || parent.try_lock().unwrap().factory.tag_name().as_str()
                != UsxFactory::get().tag_name().as_str()
        {
            if !self.progression {
                for tag in self.tags.iter() {
                    if tag.factory.name() == factory.name() {
                        return false;
                    }
                }
            }
            return factory.supports(tag_name, attributes, progression);
        }
        match self.supports_internal(factory, tag_name, attributes) {
            BookTagResult::Supported => {
                self.last_factory = Arc::clone(factory);
                true
            }
            BookTagResult::Unsupported => false,
            BookTagResult::Unknown => {
                !self.progression && self.supports(factory, parent, tag_name, attributes, &None)
            }
        }
    }

    fn supports_internal(
        &mut self,
        factory: &Arc<UsxItemFactory>,
        tag_name: &str,
        attributes: &HashMap<String, String>,
    ) -> BookTagResult {
        for tag_index in 0..self.tags.len() {
            let ctx_tag = self.tags.get(tag_index).unwrap();
            if self.completed.contains(&ctx_tag.tag) {
                if ctx_tag.factory.name() == factory.name() {
                    return BookTagResult::Unsupported;
                }
                continue;
            }
            if ctx_tag.factory.name() == factory.name() {
                if tag_index > 0 {
                    let tag = self.tags.get(tag_index - 1).unwrap();
                    if !self.completed.contains(&tag.tag) {
                        return BookTagResult::Unsupported;
                    }
                }
                for i in 0..ctx_tag.max_styles {
                    if ctx_tag.factory.supports(tag_name, attributes, &Some(i)) {
                        if i + 1 == ctx_tag.max_styles {
                            self.completed.push(ctx_tag.tag.clone());
                        }
                        return BookTagResult::Supported;
                    }
                }
                if factory.name() == self.last_factory.name() {
                    self.completed.push(ctx_tag.tag.clone());
                }
                return BookTagResult::Unsupported;
            }
        }
        BookTagResult::Unknown
    }

    pub fn add_text(&mut self, text: &str, position: i64) {
        self.positions
            .push(Arc::new(Mutex::new(Position::new(position))));
        if self.items.is_empty() {
            return;
        }
        {
            let name = "#text".to_string();
            let attrs = HashMap::new();
            let item = self.push(&name, &attrs, position);
            let mut mitem = item.try_lock().unwrap();
            if let UsxItem::Text(txt) = mitem.deref_mut() {
                txt.text = Some(text.to_owned())
            }
        }
        self.pop(position + text.len() as i64);
    }

    pub fn push(
        &mut self,
        tag_name: &String,
        attributes: &HashMap<String, String>,
        position: i64,
    ) -> Arc<Mutex<UsxItem>> {
        self.positions
            .push(Arc::new(Mutex::new(Position::new(position))));
        if tag_name.to_lowercase() == "usx" {
            let factory = UsxFactory::get().get();
            let item = factory.create(self, &None, attributes);
            self.items.push(Arc::new(Mutex::new(UsxNode {
                factory: Arc::clone(&factory),
                item: Arc::clone(&item),
            })));
            return item;
        }
        if self.items.is_empty() {
            panic!("empty stack, invalid state");
        }
        let node = Arc::clone(self.items.last().unwrap());
        let node_factory = Arc::clone(&node.try_lock().unwrap().factory);
        let node_parent = Arc::clone(&node);
        let factory =
            find_child_factory(&node_factory, self, &node_parent, tag_name, attributes).unwrap();
        let parent = Arc::clone(&node.try_lock().unwrap().item);
        let item = factory.create(self, &Some(parent), attributes);
        match item.try_lock().unwrap().deref_mut() {
            UsxItem::ChapterStart(start) => {
                self.progression = false;
                self.positions
                    .push(Arc::new(Mutex::new(Position::new(position))));
                let usfm = self.book.as_ref().try_lock().unwrap().usfm().clone();
                let chapter = Chapter::create(self, None, usfm, start);
                self.positions.pop();
                self.items.push(Arc::new(Mutex::new(UsxNode {
                    factory: Arc::clone(&node.try_lock().unwrap().factory),
                    item: Arc::clone(&chapter),
                    // position,
                })));
                self.chapters.push(chapter);
            }
            UsxItem::VerseStart(start) => {
                let usfm = self
                    .chapters
                    .last()
                    .unwrap()
                    .try_lock()
                    .unwrap()
                    .usfm()
                    .unwrap();
                let position = Arc::new(Mutex::new(Position::new(
                    start.position.try_lock().unwrap().start,
                )));
                self.push_verse(&usfm, start, &item, &position);
            }
            _ => {}
        }
        let container_node = node.lock().unwrap();
        let mut container = container_node.item.lock().unwrap();
        container.add_child(Arc::clone(&item));
        self.items.push(Arc::new(Mutex::new(UsxNode {
            factory: Arc::clone(&factory),
            item: Arc::clone(&item),
            // position,
        })));
        item
    }

    pub fn pop(&mut self, position: i64) {
        let last_position = self.positions.pop();
        if last_position.is_some() {
            let p = last_position.unwrap();
            let mut p = p.lock().unwrap();
            p.end = position;
        }
        let node = self.items.pop();
        if node.is_some() {
            match node
                .unwrap()
                .lock()
                .unwrap()
                .item
                .lock()
                .unwrap()
                .deref_mut()
            {
                UsxItem::VerseEnd(_) => {
                    let verse = self.pop_verse().unwrap();
                    let v = verse.lock().unwrap();
                    let mut p = v.position.lock().unwrap();
                    p.end = position;
                    self.verse_items.push(Arc::clone(&verse));
                }
                UsxItem::ChapterEnd(end) => {
                    let chapter_node = self.items.pop();
                    if chapter_node.is_none() {
                        panic!("missing chapter node");
                    }
                    let uchapter_node = chapter_node.unwrap();
                    let node = uchapter_node.lock().unwrap();
                    let mut lnode = node.item.lock().unwrap();
                    let chapter = match lnode.deref_mut() {
                        UsxItem::Chapter(chapter) => chapter,
                        _ => panic!("missing chapter node, invalid item"),
                    };
                    chapter.set_end(Arc::clone(&node.item));
                    let cpos = chapter.container.position.as_ref().unwrap();
                    let ccpos = Arc::clone(cpos);
                    let mut position = ccpos.lock().unwrap();
                    position.start = chapter.start_position.lock().unwrap().start;
                    position.end = end.position.lock().unwrap().end;
                    let items = self.verse_items.clone();
                    chapter.add_verse_items(items);
                    self.verse_items = Vec::new();

                    let mut book = self.book.lock().unwrap();
                    book.chapters.push(Arc::clone(&node.item));
                    book.chapters_by_usfm
                        .insert(chapter.usfm.clone(), Arc::clone(&node.item));
                }
                _ => {}
            }
        }
    }
}
