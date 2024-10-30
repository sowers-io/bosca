use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::security::principal::Principal;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Evaluator {
    permissions: HashMap<PermissionAction, Vec<Uuid>>,
}

impl Evaluator {
    fn add_permissions(
        p: &mut HashMap<PermissionAction, Vec<Uuid>>,
        action: &PermissionAction,
        group_id: &Uuid,
    ) {
        if p.contains_key(action) {
            let v = p.get_mut(action).unwrap();
            v.push(*group_id)
        } else {
            let v = vec![*group_id];
            p.insert(*action, v);
        }
    }

    pub fn new(permissions: Vec<Permission>) -> Self {
        let mut p = HashMap::<PermissionAction, Vec<Uuid>>::new();
        for permission in permissions {
            Self::add_permissions(&mut p, &permission.action, &permission.group_id);
            if permission.action == PermissionAction::Manage {
                Self::add_permissions(&mut p, &PermissionAction::View, &permission.group_id);
                Self::add_permissions(&mut p, &PermissionAction::Edit, &permission.group_id);
                Self::add_permissions(&mut p, &PermissionAction::List, &permission.group_id);
                Self::add_permissions(&mut p, &PermissionAction::Delete, &permission.group_id);
            } else if permission.action == PermissionAction::Edit {
                Self::add_permissions(&mut p, &PermissionAction::View, &permission.group_id);
                Self::add_permissions(&mut p, &PermissionAction::List, &permission.group_id);
            }
        }
        Self { permissions: p }
    }

    pub fn evaluate(&self, p: &Principal, action: &PermissionAction) -> bool {
        match self.permissions.get(action) {
            Some(groups) => {
                for group in groups {
                    if p.has_group(group) {
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }
}
