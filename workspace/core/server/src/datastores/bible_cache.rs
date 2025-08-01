use crate::datastores::cache::cache::BoscaCache;
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::models::bible::bible::Bible;
use async_graphql::Error;
use uuid::Uuid;
use crate::models::bible::bible_language::BibleLanguage;
use crate::models::bible::book::Book;
use crate::models::bible::chapter::Chapter;

#[derive(Clone)]
pub struct BibleCache {
    bible: BoscaCache<Bible>,
    books: BoscaCache<Vec<Book>>,
    chapters: BoscaCache<Chapter>,
    languages: BoscaCache<Vec<BibleLanguage>>,
}

const __DEFAULT: &str = "__default__";

impl BibleCache {
    pub async fn new(cache: &mut BoscaCacheManager) -> Result<Self, Error> {
        Ok(Self {
            bible: cache.new_id_tiered_cache("bible").await?,
            books: cache.new_id_tiered_cache("bible_books").await?,
            chapters: cache.new_id_tiered_cache("bible_chapters").await?,
            languages: cache.new_id_tiered_cache("bible_languages").await?,
        })
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant))]
    pub async fn get_bible(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &Option<String>,
    ) -> Option<Bible> {
        let variant_ref = variant.as_ref().map(|s| s.as_str());
        let variant = variant_ref.unwrap_or(__DEFAULT);
        let id = format!("{metadata_id}::{version}::{variant}");
        self.bible.get(&id).await
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant))]
    pub async fn get_languages(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &str,
    ) -> Option<Vec<BibleLanguage>> {
        let id = format!("{metadata_id}::{version}::{variant}");
        self.languages.get(&id).await
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant))]
    pub async fn get_books(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &str,
    ) -> Option<Vec<Book>> {
        let id = format!("{metadata_id}::{version}::{variant}");
        self.books.get(&id).await
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant, usfm))]
    pub async fn get_chapter(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &str,
        usfm: &str,
    ) -> Option<Chapter> {
        let id = format!("{metadata_id}::{version}::{variant}::{usfm}");
        self.chapters.get(&id).await
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant, bible))]
    pub async fn set_bible(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &Option<String>,
        bible: &Bible,
    ) {
        let variant_ref = variant.as_ref().map(|s| s.as_str());
        let variant = variant_ref.unwrap_or(__DEFAULT);
        let id = format!("{metadata_id}::{version}::{variant}");
        self.bible.set(&id, bible).await;
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant, bible_languages))]
    pub async fn set_languages(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &String,
        bible_languages: &Vec<BibleLanguage>,
    ) {
        let id = format!("{metadata_id}::{version}::{variant}");
        self.languages.set(&id, bible_languages).await;
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant, books))]
    pub async fn set_books(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &String,
        books: &Vec<Book>,
    ) {
        let id = format!("{metadata_id}::{version}::{variant}");
        self.books.set(&id, books).await;
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant, chapter))]
    pub async fn set_chapter(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &String,
        usfm: &String,
        chapter: &Chapter,
    ) {
        let id = format!("{metadata_id}::{version}::{usfm}::{variant}");
        self.chapters.set(&id, chapter).await;
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant))]
    pub async fn evict_bible(&self, metadata_id: &Uuid, version: i32, variant: &String) {
        let id = format!("{metadata_id}::{version}::{variant}");
        let id_default = format!("{metadata_id}::{version}::__default__");

        self.bible.remove(&id).await;
        self.bible.remove(&id_default).await;

        self.languages.remove(&id).await;
        self.languages.remove(&id_default).await;

        self.books.remove(&id).await;
        self.books.remove(&id_default).await;

        self.chapters.remove(&id).await;
        self.chapters.remove(&id_default).await;
    }
}
