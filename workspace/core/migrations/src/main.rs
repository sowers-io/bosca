use std::ops::DerefMut;

use bosca_database::build_pool;
use rustls::crypto::ring;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    ring::default_provider().install_default().unwrap();
    let bosca_pool = build_pool("DATABASE_URL").unwrap();
    let mut conn = bosca_pool.get().await.unwrap();
    let client = conn.deref_mut().deref_mut();
    embedded::migrations::runner().run_async(client).await.unwrap();
}
