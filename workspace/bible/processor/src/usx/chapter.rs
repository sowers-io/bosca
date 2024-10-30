use crate::book::Book;
use crate::context::UsxContext;
use crate::string_context::StringContext;
use crate::usx::chapter_start::ChapterStart;
use crate::usx::item::{UsxItem, UsxItemContainer, UsxVerseItems};
use crate::usx::position::Position;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ChapterVerse {
    pub usfm: String,
    pub chapter: String,
    pub verse: String,
    pub items: Vec<Arc<Mutex<UsxVerseItems>>>,
    pub raw: String,
}

impl ChapterVerse {
    pub fn to_string(&self, context: &Option<StringContext>) -> String {
        let mut buf = String::new();
        for item in &self.items {
            let str = &item.lock().unwrap().to_string(context);
            buf.push_str(str.as_str());
        }
        buf
    }
}

pub struct Chapter {
    pub number: String,
    pub usfm: String,
    pub verse_items: HashMap<String, Vec<Arc<Mutex<UsxVerseItems>>>>,
    pub container: UsxItemContainer,
    pub start_position: Arc<Mutex<Position>>,
    end: Option<Arc<Mutex<UsxItem>>>,
}

impl Chapter {
    pub fn create(
        context: &mut UsxContext,
        _: Option<Arc<Mutex<UsxItem>>>,
        book_usfm: String,
        start: &ChapterStart,
    ) -> Arc<Mutex<UsxItem>> {
        Arc::new(Mutex::new(UsxItem::Chapter(Self {
            number: start.number.clone(),
            usfm: format!("{}.{}", book_usfm, start.number).to_string(),
            verse_items: Default::default(),
            start_position: Arc::clone(&start.position),
            container: UsxItemContainer::new(context),
            end: None,
        })))
    }

    pub fn get_verse(&self, book: &Book, usfm: &str) -> Option<ChapterVerse> {
        if let Some(items) = self.verse_items.get(usfm) {
            let mut raw = String::new();
            for item in items.iter() {
                let r = book.get_raw_content(item.lock().unwrap().position.lock().as_ref().unwrap());
                raw.push_str(r.trim());
            }
            Some(ChapterVerse {
                usfm: usfm.to_owned(),
                chapter: self.number.clone(),
                verse: usfm.split('.').last().unwrap().to_string(),
                items: items.clone(),
                raw: raw.clone(),
            })
        } else {
            None
        }
    }

    pub fn get_verses(&self, book: Arc<Mutex<Book>>) -> Vec<ChapterVerse> {
        let mut verses: Vec<ChapterVerse> = Vec::new();
        for item in self.verse_items.iter() {
            let usfm_parts = item.0.split('.');
            let mut raw = String::new();
            for item in item.1.iter() {
                let r = book
                    .lock()
                    .unwrap()
                    .get_raw_content(item.lock().unwrap().position.lock().as_ref().unwrap());
                raw.push_str(r.trim());
            }
            let items = item.1.clone();
            verses.push(ChapterVerse {
                usfm: item.0.clone(),
                chapter: self.number.clone(),
                verse: usfm_parts.last().unwrap().to_string(),
                items,
                raw: raw.clone(),
            })
        }
        verses.sort_by(|a, b| {
            let a_chapter: i32 = a.chapter.parse().unwrap();
            let b_chapter: i32 = b.chapter.parse().unwrap();
            if a_chapter > b_chapter {
                return Ordering::Greater;
            }
            if a_chapter < b_chapter {
                return Ordering::Less;
            }
            let a_verse: i32 = a.verse.parse().unwrap();
            let b_verse: i32 = b.verse.parse().unwrap();
            if a_verse > b_verse {
                return Ordering::Greater;
            }
            if a_verse < b_verse {
                return Ordering::Less;
            }
            Ordering::Equal
        });
        verses
    }

    pub fn add_verse_items(&mut self, items: Vec<Arc<Mutex<UsxVerseItems>>>) {
        for item in items {
            let mut current = self.verse_items.get_mut(&item.lock().unwrap().usfm);
            if current.is_none() {
                let usfm = item.lock().unwrap().usfm.clone();
                self.verse_items.insert(usfm, Vec::new());
                current = self.verse_items.get_mut(&item.lock().unwrap().usfm);
            }
            current.unwrap().push(item);
        }
    }

    pub fn set_end(&mut self, chapter_end: Arc<Mutex<UsxItem>>) {
        if self.end.is_some() {
            panic!("end already set")
        }
        self.end = Some(chapter_end)
    }

    pub fn html_attributes(&self) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        attrs.insert("data-usfm".to_string(), self.usfm.to_string());
        attrs.insert("data-number".to_string(), self.number.to_string());
        attrs
    }
}
