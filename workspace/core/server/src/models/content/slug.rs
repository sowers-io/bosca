use uuid::Uuid;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SlugType {
    Metadata,
    Collection,
    Profile,
}

pub struct Slug {
    pub id: Uuid,
    pub slug_type: SlugType
}