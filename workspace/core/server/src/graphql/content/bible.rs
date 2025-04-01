use crate::context::BoscaContext;
use crate::graphql::content::bible_book::BibleBookObject;
use crate::graphql::content::bible_language::BibleLanguageObject;
use crate::models::bible::bible::Bible;
use async_graphql::{Context, Error, Object};

pub struct BibleObject {
    bible: Bible,
}

impl BibleObject {
    pub fn new(bible: Bible) -> Self {
        Self { bible }
    }
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

    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<BibleBookObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let books = ctx
            .content
            .bibles
            .get_books(&self.bible.metadata_id, self.bible.version)
            .await?;
        Ok(books.into_iter().map(BibleBookObject::new).collect())
    }
}
