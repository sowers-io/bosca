use std::sync::{Arc, Mutex};
use serde_json::{Map, Value};
use bosca_bible_processor::bible::Bible;
use bosca_bible_processor::book::Book;
use bosca_bible_processor::usx::chapter::ChapterVerse;
use bosca_bible_processor::usx::item::{IUsxItem, UsxItem};

pub fn new_bible_attrs(bible: &Bible, raw_bible_id: &str) -> Map<String, Value> {
    let mut attrs = Map::<String, Value>::new();
    attrs.insert("bible.raw.id".to_owned(), Value::String(raw_bible_id.to_owned()));
    attrs.insert("bible.type".to_owned(), Value::String("bible".to_string()));
    attrs.insert("bible.system.id".to_owned(), Value::String(bible.metadata.identification.system_id.first().unwrap().id.to_owned()));
    attrs.insert("bible.language".to_owned(), Value::String(bible.metadata.language.iso.to_owned()));
    attrs.insert("bible.abbreviation".to_owned(), Value::String(bible.metadata.identification.abbreviation_local.to_owned()));
    attrs
}

pub fn new_book_attrs(bible: &Bible, book: &Arc<Mutex<Book>>, book_order: usize) -> Map<String, Value> {
    let mut attrs = Map::<String, Value>::new();
    attrs.insert("bible.type".to_owned(), Value::String("book".to_owned()));
    attrs.insert("bible.system.id".to_owned(), Value::String(bible.metadata.identification.system_id.first().unwrap().id.to_owned()));
    attrs.insert("bible.language".to_owned(), Value::String(bible.metadata.language.iso.to_owned()));
    attrs.insert("bible.abbreviation".to_owned(), Value::String(bible.metadata.identification.abbreviation_local.to_owned()));
    attrs.insert("bible.book.usfm".to_owned(), Value::String(book.lock().unwrap().usfm().clone()));
    attrs.insert("bible.book.order".to_owned(), Value::Number(book_order.into()));
    attrs
}

pub fn new_chapter_attrs(bible: &Bible, book: &Arc<Mutex<Book>>, book_order: usize, chapter: &Arc<Mutex<UsxItem>>, chapter_order: usize) -> Map<String, Value> {
    let mut attrs = Map::<String, Value>::new();
    attrs.insert("bible.type".to_owned(), Value::String("chapter".to_owned()));
    attrs.insert("bible.system.id".to_owned(), Value::String(bible.metadata.identification.system_id.first().unwrap().id.to_owned()));
    attrs.insert("bible.language".to_owned(), Value::String(bible.metadata.language.iso.to_owned()));
    attrs.insert("bible.abbreviation".to_owned(), Value::String(bible.metadata.identification.abbreviation_local.to_owned()));
    attrs.insert("bible.book.usfm".to_owned(), Value::String(book.lock().unwrap().usfm().clone()));
    attrs.insert("bible.chapter.usfm".to_owned(), Value::String(chapter.lock().unwrap().usfm().unwrap().clone()));
    attrs.insert("bible.book.order".to_owned(), Value::Number(book_order.into()));
    attrs.insert("bible.chapter.order".to_owned(), Value::Number(chapter_order.into()));
    attrs
}

pub fn new_chapter_verses_attrs(bible: &Bible, book: &Arc<Mutex<Book>>, book_order: usize, chapter: &Arc<Mutex<UsxItem>>, chapter_order: usize) -> Map<String, Value> {
    let mut attrs = Map::<String, Value>::new();
    attrs.insert("bible.type".to_owned(), Value::String("verses".to_owned()));
    attrs.insert("bible.system.id".to_owned(), Value::String(bible.metadata.identification.system_id.first().unwrap().id.to_owned()));
    attrs.insert("bible.language".to_owned(), Value::String(bible.metadata.language.iso.to_owned()));
    attrs.insert("bible.abbreviation".to_owned(), Value::String(bible.metadata.identification.abbreviation_local.to_owned()));
    attrs.insert("bible.book.usfm".to_owned(), Value::String(book.lock().unwrap().usfm().clone()));
    attrs.insert("bible.chapter.usfm".to_owned(), Value::String(chapter.lock().unwrap().usfm().unwrap().clone()));
    attrs.insert("bible.book.order".to_owned(), Value::Number(book_order.into()));
    attrs.insert("bible.chapter.order".to_owned(), Value::Number(chapter_order.into()));
    attrs
}

pub fn new_verse_attrs(bible: &Bible, book: &Arc<Mutex<Book>>, book_order: usize, chapter: &Arc<Mutex<UsxItem>>, chapter_order: usize, verse: &ChapterVerse, verse_order: usize) -> Map<String, Value> {
    let mut attrs = Map::<String, Value>::new();
    attrs.insert("bible.type".to_owned(), Value::String("verse".to_owned()));
    attrs.insert("bible.system.id".to_owned(), Value::String(bible.metadata.identification.system_id.first().unwrap().id.to_owned()));
    attrs.insert("bible.language".to_owned(), Value::String(bible.metadata.language.iso.to_owned()));
    attrs.insert("bible.abbreviation".to_owned(), Value::String(bible.metadata.identification.abbreviation_local.to_owned()));
    attrs.insert("bible.book.usfm".to_owned(), Value::String(book.lock().unwrap().usfm().clone()));
    attrs.insert("bible.chapter.usfm".to_owned(), Value::String(chapter.lock().unwrap().usfm().unwrap().clone()));
    attrs.insert("bible.verse.usfm".to_owned(), Value::String(verse.usfm.clone()));
    attrs.insert("bible.book.order".to_owned(), Value::Number(book_order.into()));
    attrs.insert("bible.chapter.order".to_owned(), Value::Number(chapter_order.into()));
    attrs.insert("bible.verse.order".to_owned(), Value::Number(verse_order.into()));
    attrs
}