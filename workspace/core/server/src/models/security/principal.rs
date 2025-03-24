use crate::models::security::group::Group;
use serde_json::Value;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Principal {
    pub id: Uuid,
    pub verified: bool,
    pub anonymous: bool,
    pub attributes: Value,
    groups: Option<Vec<Group>>,
    group_ids: HashSet<Uuid>,
}

impl Principal {
    pub fn new(
        id: Uuid,
        verified: bool,
        anonymous: bool,
        attributes: Value,
        groups: Vec<Group>,
    ) -> Self {
        let mut group_ids = HashSet::<Uuid>::new();
        for group in groups.iter() {
            group_ids.insert(group.id);
        }
        Self {
            id,
            verified,
            anonymous,
            attributes,
            groups: Some(groups),
            group_ids,
        }
    }

    pub fn get_groups(&self) -> &Option<Vec<Group>> {
        &self.groups
    }

    pub fn has_group(&self, group_id: &Uuid) -> bool {
        self.group_ids.contains(group_id)
    }

    pub fn set_groups(&mut self, groups: &Option<Vec<Group>>) {
        let g = &mut self.group_ids;
        if groups.is_none() {
            self.groups = None;
            g.clear();
        } else {
            let gr = groups.as_ref().unwrap();
            self.groups = Some(gr.clone());
            for group in gr.iter() {
                g.insert(group.id);
            }
        }
    }
}

impl From<&Row> for Principal {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            verified: row.get("verified"),
            anonymous: row.get("anonymous"),
            attributes: row.get("attributes"),
            groups: None,
            group_ids: HashSet::new(),
        }
    }
}
