use async_graphql::Object;

pub struct CacheObject {
    name: String,
}

impl CacheObject {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[Object(name = "CacheObject")]
impl CacheObject {
    async fn name(&self) -> &String {
        &self.name
    }
}
