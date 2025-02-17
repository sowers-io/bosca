use uuid::Uuid;

pub enum SlugType {
    Metadata,
    Collection,
    Profile,
}
pub struct Slug {
    pub id: Uuid,
    pub slug_type: SlugType
}