use crate::datastores::bible_cache::BibleCache;
use crate::datastores::notifier::Notifier;
use crate::models::bible::bible::{Bible, BibleInput};
use crate::models::bible::bible_language::BibleLanguage;
use crate::models::bible::book::Book;
use crate::models::bible::chapter::Chapter;
use crate::models::bible::components::component::Component;
use crate::models::bible::components::style::Style;
use async_graphql::Error;
use bosca_database::TracingPool;
use deadpool_postgres::GenericClient;
use log::error;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct BiblesDataStore {
    pool: TracingPool,
    notifier: Arc<Notifier>,
    cache: BibleCache,
}

impl BiblesDataStore {
    pub fn new(pool: TracingPool, notifier: Arc<Notifier>, cache: BibleCache) -> Self {
        Self {
            pool,
            notifier,
            cache,
        }
    }

    async fn on_metadata_changed(&self, id: &Uuid) -> async_graphql::Result<(), Error> {
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {e:?}");
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, bible))]
    pub async fn set_bible(
        &self,
        metadata_id: &Uuid,
        version: i32,
        bible: &BibleInput,
    ) -> Result<(), Error> {
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        let stmt = txn
            .prepare("delete from bibles where metadata_id = $1 and version = $2 and variant = $3")
            .await?;
        txn.execute(&stmt, &[metadata_id, &version, &bible.variant])
            .await?;
        let stmt = txn.prepare("insert into bibles (metadata_id, version, variant, default_variant, system_id, name, name_local, description, abbreviation, abbreviation_local, styles) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)").await?;
        let styles: Vec<Style> = bible.styles.iter().map(|s| s.into()).collect();
        let styles_json = json!(styles);
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &bible.variant,
                &bible.default_variant,
                &bible.system_id,
                &bible.name,
                &bible.name_local,
                &bible.description,
                &bible.abbreviation,
                &bible.abbreviation_local,
                &styles_json,
            ],
        )
        .await?;
        let stmt = txn.prepare("insert into bible_languages (metadata_id, version, variant, iso, name, name_local, script, script_code, script_direction, sort) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)").await?;
        let sort = 0;
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &bible.variant,
                &bible.language.iso,
                &bible.language.name,
                &bible.language.name_local,
                &bible.language.script,
                &bible.language.script_code,
                &bible.language.script_direction,
                &sort,
            ],
        )
        .await?;
        let stmt = txn.prepare("insert into bible_books (metadata_id, version, variant, usfm, name_short, name_long, abbreviation, sort) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        let stmt_chapter = txn.prepare("insert into bible_chapters (metadata_id, version, variant, book_usfm, usfm, components, sort) values ($1, $2, $3, $4, $5, $6, $7)").await?;
        for (sort, book) in bible.books.iter().enumerate() {
            let sort = sort as i32;
            txn.execute(
                &stmt,
                &[
                    metadata_id,
                    &version,
                    &bible.variant,
                    &book.reference.usfm,
                    &book.name_short,
                    &book.name_long,
                    &book.abbreviation,
                    &sort,
                ],
            )
            .await?;
            for (sort, chapter) in book.chapters.iter().enumerate() {
                let sort = sort as i32;
                let component: Component = (&chapter.component).into();
                let component_json = json!(component);
                txn.execute(
                    &stmt_chapter,
                    &[
                        metadata_id,
                        &version,
                        &bible.variant,
                        &book.reference.usfm,
                        &chapter.reference.usfm,
                        &component_json,
                        &sort,
                    ],
                )
                .await?;
            }
        }
        txn.commit().await?;
        self.cache
            .evict_bible(metadata_id, version, &bible.variant)
            .await;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant))]
    pub async fn get_bible(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: Option<String>,
    ) -> Result<Option<Bible>, Error> {
        if let Some(bible) = self.cache.get_bible(metadata_id, version, &variant).await {
            return Ok(Some(bible));
        }
        let conn = self.pool.get().await?;
        let rows = if let Some(ref variant) = variant {
            let stmt = conn
                .prepare(
                    "select * from bibles where metadata_id = $1 and version = $2 and variant = $3",
                )
                .await?;
            conn.query(&stmt, &[metadata_id, &version, &variant])
                .await?
        } else {
            let stmt = conn
                .prepare(
                    "select * from bibles where metadata_id = $1 and version = $2 and default_variant = true",
                )
                .await?;
            conn.query(&stmt, &[metadata_id, &version]).await?
        };
        if rows.is_empty() {
            return Ok(None);
        }
        if let Some(row) = rows.first() {
            let bible = row.into();
            self.cache
                .set_bible(metadata_id, version, &variant, &bible)
                .await;
            Ok(Some(bible))
        } else {
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant))]
    pub async fn get_bible_languages(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &str,
    ) -> Result<Vec<BibleLanguage>, Error> {
        if let Some(languages) = self
            .cache
            .get_languages(metadata_id, version, variant)
            .await
        {
            return Ok(languages);
        }
        let variant = variant.to_string();
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("select * from bible_languages where metadata_id = $1 and version = $2 and variant = $3 order by sort asc")
            .await?;
        let row = conn
            .query(&stmt, &[metadata_id, &version, &variant])
            .await?;
        let languages = row.iter().map(|r| r.into()).collect();
        self.cache
            .set_languages(metadata_id, version, &variant, &languages)
            .await;
        Ok(languages)
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant, usfm))]
    pub async fn get_book(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &str,
        usfm: &String,
    ) -> Result<Option<Book>, Error> {
        if let Some(books) = self.get_books(metadata_id, version, variant).await? {
            if let Some(book) = books.iter().find(|b| b.reference.is_usfm(usfm)) {
                return Ok(Some(book.clone()));
            }
        }
        Ok(None)
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant))]
    pub async fn get_books(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &str,
    ) -> Result<Vec<Book>, Error> {
        if let Some(books) = self.cache.get_books(metadata_id, version, variant).await {
            return Ok(books);
        }
        let variant = variant.to_string();
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("select * from bible_books where metadata_id = $1 and version = $2 and variant = $3 order by sort asc")
            .await?;
        let row = conn
            .query(&stmt, &[metadata_id, &version, &variant])
            .await?;
        let books = row.iter().map(|r| r.into()).collect();
        self.cache
            .set_books(metadata_id, version, &variant, &books)
            .await;
        Ok(books)
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant, usfm))]
    pub async fn get_chapter(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &str,
        usfm: &str,
    ) -> Result<Option<Chapter>, Error> {
        if let Some(chapter) = self
            .cache
            .get_chapter(metadata_id, version, variant, usfm)
            .await
        {
            return Ok(Some(chapter));
        }
        let variant = variant.to_string();
        let usfm = usfm.to_string();
        let conn = self.pool.get().await?;
        let stmt = conn.prepare("select * from bible_chapters where metadata_id = $1 and version = $2 and variant = $3 and usfm = $4 order by sort asc").await?;
        let row = conn
            .query(&stmt, &[metadata_id, &version, &variant, &usfm])
            .await?;
        let chapter = row.first().map(|r| r.into());
        if let Some(ref chapter) = chapter {
            self.cache
                .set_chapter(metadata_id, version, &variant, &usfm, &chapter)
                .await;
        }
        Ok(chapter)
    }

    #[tracing::instrument(skip(self, metadata_id, version, variant, usfm))]
    pub async fn get_chapters(
        &self,
        metadata_id: &Uuid,
        version: i32,
        variant: &str,
        usfm: &str,
    ) -> Result<Vec<Chapter>, Error> {
        let usfms: Vec<String> = {
            let variant = variant.to_string();
            let usfm = usfm.to_string();
            let conn = self.pool.get().await?;
            let stmt = conn.prepare("select usfm from bible_chapters where metadata_id = $1 and version = $2 and variant = $3 and book_usfm = $4 order by sort asc").await?;
            let row = conn
                .query(&stmt, &[metadata_id, &version, &variant, &usfm])
                .await?;
            row.iter().map(|r| r.get("usfm")).collect()
        };
        let mut chapters = Vec::new();
        for usfm in usfms {
            if let Some(chapter) = self
                .get_chapter(metadata_id, version, &variant, &usfm)
                .await?
            {
                chapters.push(chapter);
            }
        }
        Ok(chapters)
    }
}
