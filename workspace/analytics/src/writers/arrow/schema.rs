use std::sync::Arc;
use arrow::datatypes::{DataType, Field, FieldRef, Fields, Schema, TimeUnit};

pub struct SchemaDefinition {
    pub schema: Arc<Schema>,
    pub content_struct: Fields,
    pub element_struct: Fields,
    pub browser_struct: Fields,
    pub device_struct: Fields,
    pub geo_struct: Fields,
    pub context_struct: Fields,
}

impl SchemaDefinition {
    pub fn new() -> SchemaDefinition {
        // Content schema
        let content_struct: Fields = vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("type", DataType::Utf8, false),
            Field::new("index", DataType::Int32, true),
            Field::new("percent", DataType::Float32, true),
        ].into();
        let content_field: FieldRef = Field::new("item", DataType::Struct(content_struct.clone()), true).into();

        // Element schema
        let element_struct: Fields = vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("type", DataType::Utf8, false),
            Field::new("content", DataType::List(Arc::clone(&content_field)), true),
            Field::new("extras", DataType::Utf8, true), // JSON serialized
        ].into();

        // Browser schema
        let browser_struct: Fields = vec![
            Field::new("agent", DataType::Utf8, false),
        ].into();

        // Device schema
        let device_struct: Fields = vec![
            Field::new("installation_id", DataType::Utf8, false),
            Field::new("manufacturer", DataType::Utf8, false),
            Field::new("model", DataType::Utf8, false),
            Field::new("platform", DataType::Utf8, false),
            Field::new("primary_locale", DataType::Utf8, false),
            Field::new("system_name", DataType::Utf8, false),
            Field::new("timezone", DataType::Utf8, false),
            Field::new("type", DataType::Utf8, false),
            Field::new("version", DataType::Utf8, false),
        ].into();

        // Geo schema
        let geo_struct: Fields = vec![
            Field::new("city", DataType::Utf8, true),
            Field::new("country", DataType::Utf8, true),
            Field::new("region", DataType::Utf8, true),
        ].into();

        // Context schema
        let context_struct: Fields = vec![
            Field::new("app_id", DataType::Utf8, false),
            Field::new("app_version", DataType::Utf8, false),
            Field::new("browser", DataType::Struct(browser_struct.clone()), true),
            Field::new("client_id", DataType::Utf8, false),
            Field::new("device", DataType::Struct(device_struct.clone()), false),
            Field::new("geo", DataType::Struct(geo_struct.clone()), false),
            Field::new("session_id", DataType::Utf8, false),
            Field::new("user_id", DataType::Utf8, true),
        ].into();

        // Top level schema (flattened)
        let schema = Schema::new(vec![
            // Context fields
            Field::new("context", DataType::Struct(context_struct.clone()), false),
            // Event fields
            Field::new("created", DataType::Timestamp(TimeUnit::Millisecond, Some("UTC".into())), false),
            Field::new("created_micros", DataType::Int64, true),
            Field::new("type", DataType::Utf8, false),
            Field::new("element", DataType::Struct(element_struct.clone()), false),
            // Timing fields
            Field::new("sent", DataType::Timestamp(TimeUnit::Millisecond, Some("UTC".into())), false),
            Field::new("sent_micros", DataType::Int64, true),
        ]);

        SchemaDefinition {
            schema: Arc::new(schema),
            content_struct,
            element_struct,
            browser_struct,
            device_struct,
            geo_struct,
            context_struct
        }
    }
}