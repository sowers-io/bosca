use crate::book::Book;
use crate::metadata::Metadata;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Bible {
    pub metadata: Metadata,
    pub books: Vec<Arc<Mutex<Book>>>,
    pub books_by_usfm: HashMap<String, Arc<Mutex<Book>>>,
}

impl Bible {
    pub fn new(metadata: Metadata, books: Vec<Arc<Mutex<Book>>>) -> Self {
        let mut bible = Self {
            metadata,
            books,
            books_by_usfm: HashMap::<String, Arc<Mutex<Book>>>::new(),
        };
        for book in bible.books.iter() {
            bible
                .books_by_usfm
                .insert(book.try_lock().unwrap().usfm().clone(), Arc::clone(book));
        }
        bible
    }
}