use crate::datastores::notifier::Notifier;
use crate::models::bible::bible::{Bible, BibleInput};
use crate::models::bible::bible_language::BibleLanguage;
use crate::models::bible::book::Book;
use crate::models::bible::chapter::Chapter;
use crate::models::bible::components::component::Component;
use async_graphql::Error;
use deadpool_postgres::{GenericClient, Pool};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct BiblesDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl BiblesDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

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
        let stmt = txn.prepare("insert into bibles (metadata_id, version, system_id, name, name_local, description, abbreviation, abbreviation_local) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
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
        let stmt_usx = txn.prepare("insert into bible_book_usx (metadata_id, version, usfm, usx) values ($1, $2, $3, $4)").await?;
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
            txn.execute(
                &stmt_usx,
                &[metadata_id, &version, &book.reference.usfm, &book.usx],
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
        Ok(())
    }

    pub async fn get_bible(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Option<Bible>, Error> {
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("select * from bibles where metadata_id = $1 and version = $2")
            .await?;
        let row = conn.query_one(&stmt, &[metadata_id, &version]).await?;
        if row.is_empty() {
            return Ok(None);
        }
        Ok(Some((&row).into()))
    }

    pub async fn get_bible_languages(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<BibleLanguage>, Error> {
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("select * from bible_languages where metadata_id = $1 and version = $2")
            .await?;
        let row = conn.query(&stmt, &[metadata_id, &version]).await?;
        Ok(row.iter().map(|r| r.into()).collect())
    }

    pub async fn get_books(&self, metadata_id: &Uuid, version: i32) -> Result<Vec<Book>, Error> {
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("select * from bible_books where metadata_id = $1 and version = $2")
            .await?;
        let row = conn.query(&stmt, &[metadata_id, &version]).await?;
        Ok(row.iter().map(|r| r.into()).collect())
    }

    pub async fn get_chapters(
        &self,
        metadata_id: &Uuid,
        version: i32,
        usfm: &str,
    ) -> Result<Vec<Chapter>, Error> {
        let usfm = usfm.to_string();
        let conn = self.pool.get().await?;
        let stmt = conn.prepare("select * from bible_chapters where metadata_id = $1 and version = $2 and book_usfm = $3").await?;
        let row = conn.query(&stmt, &[metadata_id, &version, &usfm]).await?;
        Ok(row.iter().map(|r| r.into()).collect())
    }
}
