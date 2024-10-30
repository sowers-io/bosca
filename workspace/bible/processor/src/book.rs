use crate::metadata::{ManifestName, MetadataPublicationContent};
use crate::usx::item::UsxItem;
use crate::usx::position::Position;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Book {
    pub name: ManifestName,
    pub chapters: Vec<Arc<Mutex<UsxItem>>>,
    pub chapters_by_usfm: HashMap<String, Arc<Mutex<UsxItem>>>,
    content: MetadataPublicationContent,
    pub raw: String,
}

impl Book {
    pub fn new(name: ManifestName, content: MetadataPublicationContent) -> Self {
        Self {
            name,
            chapters: vec![],
            chapters_by_usfm: Default::default(),
            content,
            raw: String::new(),
        }
    }

    pub fn usfm(&self) -> &String {
        self.content.usfm()
    }

    pub fn get_raw_content(&self, position: &Position) -> String {
        self.raw[(position.start as usize)..(position.end as usize)].to_string()
    }
}
