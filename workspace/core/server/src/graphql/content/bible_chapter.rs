use crate::graphql::content::bible_reference::BibleReferenceObject;
use crate::models::bible::book::Book;
use crate::models::bible::chapter::Chapter;
use crate::models::bible::reference::Reference;
use async_graphql::Object;
use serde_json::{json, Value};

pub struct BibleChapterObject {
    book: Book,
    chapter: Chapter,
    reference: Option<Reference>,
}

impl BibleChapterObject {
    pub fn new(book: Book, chapter: Chapter, reference: Option<Reference>) -> Self {
        Self {
            book,
            chapter,
            reference,
        }
    }
}

#[Object(name = "BibleChapter")]
impl BibleChapterObject {
    async fn usfm(&self) -> &String {
        self.chapter.reference.usfm()
    }

    async fn human(&self) -> String {
        format!("{} {}", self.book.name_short, self.chapter.reference.chapter().unwrap_or_default())
    }

    async fn reference(&self) -> BibleReferenceObject {
        if let Some(ref r) = self.reference {
            BibleReferenceObject::new(r.clone(), self.book.clone())
        } else {
            BibleReferenceObject::new(self.chapter.reference.clone(), self.book.clone())
        }
    }

    async fn component(&self) -> Value {
        if let Some(ref r) = self.reference {
            let refs = r.references();
            if refs.is_empty() || refs.iter().all(|r| r.verse_usfm().is_none()) {
                json!(self.chapter.component)
            } else {
                json!(self.chapter.component.filter(r))
            }
        } else {
            json!(self.chapter.component)
        }
    }

    async fn verses(&self) -> Vec<BibleReferenceObject> {
        if let Some(ref r) = self.reference {
            let refs = r.references();
            if refs.is_empty() || refs.iter().all(|r| r.verse_usfm().is_none()) {
                self.chapter.component.find_verses(None)
            } else {
                self.chapter.component.find_verses(Some(refs))
            }
        } else {
            self.chapter.component.find_verses(None)
        }
        .into_iter()
        .map(|r| BibleReferenceObject::new(r, self.book.clone()))
        .collect()
    }
}
