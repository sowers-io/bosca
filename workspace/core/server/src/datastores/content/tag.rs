use async_graphql::Error;
use deadpool_postgres::Transaction;
use uuid::Uuid;

#[tracing::instrument(skip(txn, id))]
pub async fn update_metadata_etag(txn: &Transaction<'_>, id: &Uuid) -> async_graphql::Result<(), Error> {
    let stmt = txn
        .prepare_cached("select calculate_metadata_etag($1)")
        .await?;
    txn.execute(&stmt, &[id]).await?;
    Ok(())
}

#[tracing::instrument(skip(txn, id))]
pub async fn update_collection_etag(txn: &Transaction<'_>, id: &Uuid) -> async_graphql::Result<(), Error> {
    let stmt = txn
        .prepare_cached("select calculate_collection_etag($1)")
        .await?;
    txn.execute(&stmt, &[id]).await?;
    Ok(())
}