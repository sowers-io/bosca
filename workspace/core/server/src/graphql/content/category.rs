use crate::models::content::category::Category;
use async_graphql::Object;

pub struct CategoryObject {
    pub category: Category,
}

impl CategoryObject {
    pub fn new(category: Category) -> Self {
        Self { category }
    }
}

#[Object(name = "Category")]
impl CategoryObject {
    pub async fn id(&self) -> String {
        self.category.id.to_string()
    }

    pub async fn name(&self) -> &String {
        &self.category.name
    }
}
