use serde_json::{json, Value};
use yaml_rust2::Yaml;

pub fn into(yaml: &Yaml) -> Value {
    match yaml {
        Yaml::Real(r) => json!(r),
        Yaml::Integer(r) => json!(r),
        Yaml::String(r) => json!(r),
        Yaml::Boolean(r) => json!(r),
        Yaml::Array(r) => Value::Array(r.iter().map(into).collect()),
        Yaml::Hash(r) => Value::Object(
            r.into_iter()
                .map(|(k, v)| (k.as_str().unwrap().to_string(), into(v)))
                .collect(),
        ),
        Yaml::Alias(r) => json!(r),
        Yaml::Null => Value::Null,
        Yaml::BadValue => Value::Null,
    }
}
