use crate::models::security::group::Group;
use async_graphql::Object;

pub struct GroupObject {
    group: Group,
}

impl GroupObject {
    pub fn new(group: Group) -> Self {
        Self { group }
    }
}

#[Object(name = "Group")]
impl GroupObject {
    async fn id(&self) -> String {
        self.group.id.to_string()
    }

    async fn name(&self) -> &String {
        &self.group.name
    }
}

impl From<Group> for GroupObject {
    fn from(group: Group) -> Self {
        Self::new(group)
    }
}
