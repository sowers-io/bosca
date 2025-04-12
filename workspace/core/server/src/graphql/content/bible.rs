use crate::context::BoscaContext;
use crate::graphql::content::bible_book::BibleBookObject;
use crate::graphql::content::bible_chapter::BibleChapterObject;
use crate::graphql::content::bible_language::BibleLanguageObject;
use crate::models::bible::bible::Bible;
use crate::models::bible::reference_parse::parse;
use async_graphql::{Context, Error, Object, SimpleObject};
use serde_json::{json, Value};
use crate::models::bible::reference::Reference;

pub struct BibleObject {
    bible: Bible,
}

impl BibleObject {
    pub fn new(bible: Bible) -> Self {
        Self { bible }
    }
}

#[derive(SimpleObject)]
pub struct FindBibleResult {
    pub usfm: String,
    pub human: String,
    pub book: BibleBookObject,
    pub chapter: Option<BibleChapterObject>,
    pub component: Option<FilteredComponent>
}

#[derive(SimpleObject)]
pub struct FilteredComponent {
    pub content: Value
}

#[derive(SimpleObject)]
pub struct BibleChapterComponent {
    pub chapter: BibleChapterObject,
    pub component: FilteredComponent
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

    async fn find(&self, ctx: &Context<'_>, human: String) -> Result<Vec<FindBibleResult>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let mut results = parse(ctx, &self.bible, &human).await?;
        let mut items = Vec::new();
        for result in results.iter_mut() {
            if let Some(book_usfm) = result.book_usfm() {
                if let Some(book) = ctx.content.bibles.get_book(&self.bible.metadata_id, self.bible.version, &book_usfm).await? {
                    let chapter = if let Some(chapter_usfm) = result.chapter_usfm() {
                        ctx.content.bibles.get_chapter(&self.bible.metadata_id, self.bible.version, &chapter_usfm).await?
                    } else {
                        None
                    };
                    let component = if let Some(chapter) = &chapter {
                        chapter.component.filter(result)
                    } else {
                        None
                    };
                    items.push(FindBibleResult {
                        human: result.format(&book),
                        usfm: result.usfm().clone(),
                        book: BibleBookObject::new(book.clone()),
                        chapter: chapter.map(|c| BibleChapterObject::new(book.clone(), c, Some(result.clone()))),
                        component: component.map(|c| FilteredComponent { content: json!(c) })
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
        let reference = Reference::new(usfm);
        let Some(book_usfm) = reference.book_usfm() else {
            return Ok(None);
        };
        let Some(chapter_usfm) = reference.chapter_usfm() else {
            return Ok(None);
        };
        let Some(book) = ctx
            .content
            .bibles
            .get_book(&self.bible.metadata_id, self.bible.version, &book_usfm)
            .await? else {
            return Ok(None);
        };
        let Some(chapter) = ctx
            .content
            .bibles
            .get_chapter(&self.bible.metadata_id, self.bible.version, &chapter_usfm)
            .await? else {
            return Ok(None)
        };
        if reference.verse().is_some() {
            Ok(Some(BibleChapterObject::new(book, chapter, Some(reference))))
        } else {
            Ok(Some(BibleChapterObject::new(book, chapter, None)))
        }
    }

    async fn styles(&self) -> Value {
        json!(self.bible.styles)
    }
}
