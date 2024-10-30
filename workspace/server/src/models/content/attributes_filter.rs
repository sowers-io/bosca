use std::collections::HashSet;
use std::sync::Arc;
use async_graphql::InputObject;
use serde_json::{Map, Value};

#[derive(InputObject)]
pub struct AttributesFilterInput {
    pub attributes: HashSet<String>,
    pub child_attributes: Option<Arc<AttributesFilterInput>>
}

impl AttributesFilterInput {

    pub fn filter(&self, attributes: &Value) -> Value {
        match attributes {
            Value::Object(map) => {
                let mut filtered = Map::new();
                for (key, value) in map {
                    if self.attributes.contains(key) {
                        if let Some(child_filter) = &self.child_attributes {
                            let filtered_value = child_filter.filter(value);
                            filtered.insert(key.to_owned(), filtered_value);
                        } else {
                            filtered.insert(key.to_owned(), value.to_owned());
                        }
                    }
                }
                Value::Object(filtered)
            },
            Value::Array(array) => {
                let mut filtered = Vec::new();
                for value in array {
                    if let Some(child_filter) = &self.child_attributes {
                        let filtered_value = child_filter.filter(value);
                        filtered.push(filtered_value);
                    } else {
                        filtered.push(value.to_owned());
                    }
                }
                Value::Array(filtered)
            },
            _ => attributes.clone()
        }
    }
}