use uuid::Uuid;

pub enum SlugType {
    Metadata,
    Collection,
}
pub struct Slug {
    pub id: Uuid,
    pub slug_type: SlugType
}