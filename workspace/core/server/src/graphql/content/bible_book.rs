use crate::context::BoscaContext;
use crate::graphql::content::bible_chapter::BibleChapterObject;
use crate::models::bible::book::Book;
use async_graphql::{Context, Error, Object};

pub struct BibleBookObject {
    book: Book,
}

impl BibleBookObject {
    pub fn new(book: Book) -> Self {
        Self { book }
    }
}

#[Object(name = "BibleBook")]
impl BibleBookObject {
    async fn usfm(&self) -> &String {
        self.book.reference.usfm()
    }

    async fn name_short(&self) -> &String {
        &self.book.name_short
    }

    async fn name_long(&self) -> &String {
        &self.book.name_long
    }

    async fn human(&self) -> &String {
        &self.book.name_long
    }

    async fn abbreviation(&self) -> &String {
        &self.book.abbreviation
    }

    async fn chapters(&self, ctx: &Context<'_>) -> Result<Vec<BibleChapterObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let chapters = ctx
            .content
            .bibles
            .get_chapters(
                &self.book.metadata_id,
                self.book.version,
                &self.book.variant,
                self.book.reference.usfm(),
            )
            .await?;
        Ok(chapters
            .into_iter()
            .map(|c| BibleChapterObject::new(self.book.clone(), c, None))
            .collect())
    }
}
