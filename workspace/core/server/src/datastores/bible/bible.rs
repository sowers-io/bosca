use crate::datastores::notifier::Notifier;
use crate::models::bible::bible::{Bible, BibleInput};
use crate::models::bible::bible_language::BibleLanguage;
use crate::models::bible::book::Book;
use crate::models::bible::chapter::Chapter;
use crate::models::bible::components::component::Component;
use async_graphql::Error;
use deadpool_postgres::GenericClient;
use serde_json::json;
use std::sync::Arc;
use log::error;
use uuid::Uuid;
use bosca_database::TracingPool;
use crate::models::bible::components::style::Style;

#[derive(Clone)]
pub struct BiblesDataStore {
    pool: TracingPool,
    notifier: Arc<Notifier>,
}

impl BiblesDataStore {
    pub fn new(pool: TracingPool, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_metadata_changed(&self, id: &Uuid) -> async_graphql::Result<(), Error> {
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
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
            .prepare("delete from bibles where metadata_id = $1 and version = $2")
            .await?;
        txn.execute(&stmt, &[metadata_id, &version]).await?;
        let stmt = txn.prepare("insert into bibles (metadata_id, version, system_id, name, name_local, description, abbreviation, abbreviation_local, styles) values ($1, $2, $3, $4, $5, $6, $7, $8, $9)").await?;
        let styles: Vec<Style> = bible.styles.iter().map(|s| s.into()).collect();
        let styles_json = json!(styles);
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &bible.system_id,
                &bible.name,
                &bible.name_local,
                &bible.description,
                &bible.abbreviation,
                &bible.abbreviation_local,
                &styles_json
            ],
        )
        .await?;
        let stmt = txn.prepare("insert into bible_languages (metadata_id, version, iso, name, name_local, script, script_code, script_direction, sort) values ($1, $2, $3, $4, $5, $6, $7, $8, $9)").await?;
        let sort = 0;
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
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
        let stmt = txn.prepare("insert into bible_books (metadata_id, version, usfm, name_short, name_long, abbreviation, sort) values ($1, $2, $3, $4, $5, $6, $7)").await?;
        let stmt_chapter = txn.prepare("insert into bible_chapters (metadata_id, version, book_usfm, usfm, components, sort) values ($1, $2, $3, $4, $5, $6)").await?;
        for (sort, book) in bible.books.iter().enumerate() {
            let sort = sort as i32;
            txn.execute(
                &stmt,
                &[
                    metadata_id,
                    &version,
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
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_bible(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Option<Bible>, Error> {
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("select * from bibles where metadata_id = $1 and version = $2")
            .await?;
        let rows = conn.query(&stmt, &[metadata_id, &version]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        if let Some(row) = rows.first() {
            Ok(Some(row.into()))
        } else {
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_bible_languages(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<BibleLanguage>, Error> {
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("select * from bible_languages where metadata_id = $1 and version = $2 order by sort asc")
            .await?;
        let row = conn.query(&stmt, &[metadata_id, &version]).await?;
        Ok(row.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, metadata_id, version, usfm))]
    pub async fn get_book(&self, metadata_id: &Uuid, version: i32, usfm: &String) -> Result<Option<Book>, Error> {
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("select * from bible_books where metadata_id = $1 and version = $2 and usfm = $3")
            .await?;
        let row = conn.query(&stmt, &[metadata_id, &version, usfm]).await?;
        Ok(row.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_books(&self, metadata_id: &Uuid, version: i32) -> Result<Vec<Book>, Error> {
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("select * from bible_books where metadata_id = $1 and version = $2 order by sort asc")
            .await?;
        let row = conn.query(&stmt, &[metadata_id, &version]).await?;
        Ok(row.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, metadata_id, version, usfm))]
    pub async fn get_chapter(
        &self,
        metadata_id: &Uuid,
        version: i32,
        usfm: &str,
    ) -> Result<Option<Chapter>, Error> {
        let usfm = usfm.to_string();
        let conn = self.pool.get().await?;
        let stmt = conn.prepare("select * from bible_chapters where metadata_id = $1 and version = $2 and usfm = $3 order by sort asc").await?;
        let row = conn.query(&stmt, &[metadata_id, &version, &usfm]).await?;
        Ok(row.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, metadata_id, version, usfm))]
    pub async fn get_chapters(
        &self,
        metadata_id: &Uuid,
        version: i32,
        usfm: &str,
    ) -> Result<Vec<Chapter>, Error> {
        let usfm = usfm.to_string();
        let conn = self.pool.get().await?;
        let stmt = conn.prepare("select * from bible_chapters where metadata_id = $1 and version = $2 and book_usfm = $3 order by sort asc").await?;
        let row = conn.query(&stmt, &[metadata_id, &version, &usfm]).await?;
        Ok(row.iter().map(|r| r.into()).collect())
    }
}
