use std::ops::Deref;
use std::sync::{Arc, Mutex};
use serde_json::Value;
use bosca_bible_processor::bible::Bible;
use bosca_bible_processor::book::Book;
use bosca_bible_processor::usx::chapter::ChapterVerse;
use bosca_bible_processor::usx::item::UsxItem;
use bosca_client::client::add_collection::{CollectionInput, CollectionType, MetadataInput, MetadataSourceInput, MetadataType, CollectionChildInput, MetadataChildInput};
use bosca_client::client::Source;
use crate::attrs::{new_bible_attrs, new_book_attrs, new_chapter_attrs, new_chapter_verses_attrs, new_verse_attrs};

pub fn new_bible_collection(bible: &Bible, raw_bible_id: &str, bible_collection_id: &str, source: &Source) -> CollectionInput {
    let attrs = new_bible_attrs(bible, raw_bible_id);
    let mut books = Vec::new();
    let all_books = bible.books.iter().clone();
    for (i, book) in all_books.enumerate() {
        books.push(new_book_collection(bible, book, i, source));
    }
    CollectionInput {
        parent_collection_id: Some(bible_collection_id.to_owned()),
        collection_type: Some(CollectionType::STANDARD),
        name: bible.metadata.identification.name_local.clone(),
        description: None,
        labels: None,
        attributes: Some(Value::Object(attrs)),
        state: None,
        index: None,
        ordering: Some(serde_json::from_str("[{\"path\": [\"bible.book.order\"], \"order\": \"asc\"}]").unwrap()),
        collections: Some(books),
        metadata: None
    }
}

pub fn new_book_collection(bible: &Bible, book: &Arc<Mutex<Book>>, book_order: usize, source: &Source) -> CollectionChildInput {
    let attrs = new_book_attrs(bible, book, book_order);
    let mut chapters = Vec::new();
    let all_chapters = book.lock().unwrap().chapters.clone();
    for (i, chapter) in all_chapters.iter().enumerate() {
        chapters.push(new_chapter(bible, book, book_order, chapter, i, source));
    }
    let mut chapters_verses = Vec::new();
    let all_chapters = book.lock().unwrap().chapters.clone();
    for (i, chapter) in all_chapters.iter().enumerate() {
        chapters_verses.push(new_chapter_verse_collection(bible, book, book_order, chapter, i, source));
    }
    CollectionChildInput {
        collection: CollectionInput {
            parent_collection_id: None,
            collection_type: Some(CollectionType::STANDARD),
            name: book.lock().unwrap().name.short().clone(),
            description: None,
            labels: None,
            attributes: Some(Value::Object(attrs)),
            state: None,
            index: None,
            ordering: Some(serde_json::from_str("[{\"path\": [\"bible.chapter.order\"], \"order\": \"asc\"}]").unwrap()),
            collections: Some(chapters_verses),
            metadata: Some(chapters)
        },
        attributes: Some(serde_json::from_str(format!("{{\"bible.book.order\": {}}}", book_order).to_string().as_str()).unwrap()),
    }
}

pub fn new_chapter(bible: &Bible, book: &Arc<Mutex<Book>>, book_order: usize, chapter: &Arc<Mutex<UsxItem>>, chapter_order: usize, source: &Source) -> MetadataChildInput {
    let attrs = new_chapter_attrs(bible, book, book_order, chapter, chapter_order);
    let chapter_number = match chapter.lock().unwrap().deref() {
        UsxItem::Chapter(chapter) => chapter.number.clone(),
        _ => "".to_string(),
    };
    MetadataChildInput {
        metadata: MetadataInput {
            parent_collection_id: None,
            parent_id: None,
            version: None,
            metadata_type: Some(MetadataType::STANDARD),
            name: format!("{} {}", book.lock().unwrap().name.short().clone(), chapter_number),
            content_type: "bible/usx-chapter".to_owned(),
            content_length: None,
            language_tag: bible.metadata.language.iso.clone(),
            labels: None,
            trait_ids: Some(vec!["bible.usx.chapter".to_string()]),
            category_ids: None,
            attributes: Some(Value::Object(attrs)),
            state: None,
            index: None,
            source: Some(MetadataSourceInput {
                id: source.id.clone(),
                identifier: "".to_string(),
            })
        },
        attributes: Some(serde_json::from_str(format!("{{\"bible.chapter.order\": {}}}", chapter_order).to_string().as_str()).unwrap()),
    }
}

fn new_chapter_verse_collection(bible: &Bible, book: &Arc<Mutex<Book>>, book_order: usize, chapter: &Arc<Mutex<UsxItem>>, chapter_order: usize, source: &Source) -> CollectionChildInput {
    let attrs = new_chapter_verses_attrs(bible, book, book_order, chapter, chapter_order);
    let mut verses_metadata = Vec::new();
    let verses = match chapter.lock().unwrap().deref() {
        UsxItem::Chapter(c) => c.get_verses(Arc::clone(book)),
        _ => Vec::new(),
    };
    for (order, verse) in verses.into_iter().enumerate() {
        verses_metadata.push(new_verse(bible, book, book_order, chapter, chapter_order, &verse, order, source));
    }
    CollectionChildInput {
        collection: CollectionInput {
            parent_collection_id: None,
            collection_type: Some(CollectionType::STANDARD),
            name: format!(
                "{} {} Verses",
                book.lock().unwrap().name.short(),
                match chapter.lock().unwrap().deref() {
                        UsxItem::Chapter(c) => c.number.clone(),
                    _ => "".to_string(),
                }
            ),
            description: None,
            labels: None,
            attributes: Some(Value::Object(attrs)),
            state: None,
            index: None,
            ordering: Some(serde_json::from_str("[{\"path\": [\"bible.verse.order\"], \"order\": \"asc\"}]").unwrap()),
            collections: None,
            metadata: Some(verses_metadata)
        },
        attributes: Some(serde_json::from_str(format!("{{\"bible.chapter.order\": {}}}", chapter_order).to_string().as_str()).unwrap()),
    }
}

#[allow(clippy::too_many_arguments)]
fn new_verse(bible: &Bible, book: &Arc<Mutex<Book>>, book_order: usize, chapter: &Arc<Mutex<UsxItem>>, chapter_order: usize, verse: &ChapterVerse, verse_order: usize, source: &Source) -> MetadataChildInput {
    let attrs = new_verse_attrs(bible, book, book_order, chapter, chapter_order, verse, verse_order);
    MetadataChildInput {
        metadata: MetadataInput {
            parent_collection_id: None,
            parent_id: None,
            version: None,
            metadata_type: Some(MetadataType::STANDARD),
            name: format!(
                "{} {}:{}",
                book.lock().unwrap().name.short(),
                match chapter.lock().unwrap().deref() {
                        UsxItem::Chapter(c) => c.number.clone(),
                    _ => "".to_owned(),
                },
                verse.verse
            ),
            content_type: "bible/usx-verse".to_owned(),
            content_length: None,
            language_tag: bible.metadata.language.iso.to_owned(),
            labels: None,
            trait_ids: Some(vec!["bible.usx.verse".to_owned()]),
            category_ids: None,
            attributes: Some(Value::Object(attrs)),
            state: None,
            index: None,
            source: Some(MetadataSourceInput {
                id: source.id.clone(),
                identifier: "".to_owned(),
            })
        },
        attributes: Some(serde_json::from_str(format!("{{\"bible.verse.order\": {}}}", verse_order).to_string().as_str()).unwrap()),
    }
}