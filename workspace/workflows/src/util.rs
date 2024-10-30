use serde_json::{Map, Value};

pub fn add_to(union: &mut Map<String, Value>, a: &Value) {
    if let Value::Object(map) = a {
        for e in map {
            union.insert(e.0.clone(), e.1.clone());
        }
    }
}