use crate::context::BoscaContext;
use crate::graphql::content::bible_book::BibleBookObject;
use crate::graphql::content::bible_chapter::BibleChapterObject;
use crate::graphql::content::bible_language::BibleLanguageObject;
use crate::models::bible::bible::Bible;
use crate::models::bible::reference_parse::parse;
use async_graphql::{Context, Error, Object, SimpleObject, Union};
use serde_json::{json, Value};

pub struct BibleObject {
    bible: Bible,
}

impl BibleObject {
    pub fn new(bible: Bible) -> Self {
        Self { bible }
    }
}

#[derive(SimpleObject)]
pub struct FindResult {
    pub usfm: String,
    pub content: FindResultContent,
}

#[derive(SimpleObject)]
pub struct FilteredComponent {
    pub content: Value
}

#[derive(Union)]
pub enum FindResultContent {
    Book(BibleBookObject),
    Chapter(BibleChapterObject),
    Component(FilteredComponent),
}

#[Object(name = "Bible")]
impl BibleObject {
    async fn system_id(&self) -> &String {
        &self.bible.system_id
    }

    async fn name(&self) -> &String {
        &self.bible.name
    }

    async fn name_local(&self) -> &String {
        &self.bible.name_local
    }

    async fn description(&self) -> &String {
        &self.bible.description
    }

    async fn abbreviation(&self) -> &String {
        &self.bible.abbreviation
    }

    async fn abbreviation_local(&self) -> &String {
        &self.bible.abbreviation_local
    }

    async fn languages(&self, ctx: &Context<'_>) -> Result<Vec<BibleLanguageObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let languages = ctx
            .content
            .bibles
            .get_bible_languages(&self.bible.metadata_id, self.bible.version)
            .await?;
        Ok(languages
            .into_iter()
            .map(BibleLanguageObject::new)
            .collect())
    }

    async fn find(&self, ctx: &Context<'_>, human: String) -> Result<Vec<FindResult>, Error> {
        let original_ctx = ctx;
        let ctx = ctx.data::<BoscaContext>()?;
        let mut results = parse(ctx, &self.bible, &human).await?;
        let mut items = Vec::new();
        for result in results.iter_mut() {
            if let Some(_) = result.verse_usfm() {
                if let Some(chapter_usfm) = result.chapter_usfm() {
                    if let Some(chapter) = ctx
                        .content
                        .bibles
                        .get_chapter(&self.bible.metadata_id, self.bible.version, &chapter_usfm)
                        .await?
                    {
                        if let Some(filtered) = chapter.component.filter(result) {
                            items.push(FindResult {
                                usfm: chapter_usfm,
                                content: FindResultContent::Component(FilteredComponent{ content: json!(filtered) }),
                            });
                        }
                    }
                }
            } else if let Some(chapter_usfm) = result.chapter_usfm() {
                if let Some(chapter) = self.chapter(original_ctx, chapter_usfm.clone()).await? {
                    items.push(FindResult {
                        usfm: chapter_usfm,
                        content: FindResultContent::Chapter(chapter),
                    });
                }
            } else if let Some(book_usfm) = result.book_usfm() {
                if let Some(book) = self.book(original_ctx, book_usfm.clone()).await? {
                    items.push(FindResult {
                        usfm: book_usfm,
                        content: FindResultContent::Book(book),
                    });
                }
            }
        }
        Ok(items)
    }

    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<BibleBookObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let books = ctx
            .content
            .bibles
            .get_books(&self.bible.metadata_id, self.bible.version)
            .await?;
        Ok(books.into_iter().map(BibleBookObject::new).collect())
    }

    async fn book(
        &self,
        ctx: &Context<'_>,
        usfm: String,
    ) -> Result<Option<BibleBookObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let books = ctx
            .content
            .bibles
            .get_book(&self.bible.metadata_id, self.bible.version, &usfm)
            .await?;
        Ok(books.map(BibleBookObject::new))
    }

    async fn chapter(
        &self,
        ctx: &Context<'_>,
        usfm: String,
    ) -> Result<Option<BibleChapterObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let books = ctx
            .content
            .bibles
            .get_chapter(&self.bible.metadata_id, self.bible.version, &usfm)
            .await?;
        Ok(books.map(BibleChapterObject::new))
    }

    async fn styles(&self) -> Value {
        json!(self.bible.styles)
    }
}
