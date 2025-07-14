use crate::models::bible::book::Book;
use crate::models::bible::reference::Reference;
use async_graphql::Object;

pub struct BibleReferenceObject {
    reference: Reference,
    book: Book
}

impl BibleReferenceObject {
    pub fn new(reference: Reference, book: Book) -> Self {
        Self { reference, book }
    }
}

#[Object(name = "BibleReference")]
impl BibleReferenceObject {

    async fn usfm(&self) -> &String {
        self.reference.usfm()
    }

    async fn human(&self) -> String {
        self.reference.format(&self.book, true)
    }

    async fn human_short(&self) -> String {
        self.reference.format(&self.book, false)
    }
}
