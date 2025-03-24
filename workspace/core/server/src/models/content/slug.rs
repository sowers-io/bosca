use uuid::Uuid;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SlugType {
    Metadata,
    Collection,
    Profile,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Slug {
    pub id: Uuid,
    pub slug_type: SlugType
}