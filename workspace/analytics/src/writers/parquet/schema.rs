use arrow::datatypes::{DataType, Field, Schema};

pub(crate) fn create_schema() -> Schema {
    // Content schema
    let content_schema = vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("content_type", DataType::Utf8, false),
        Field::new("index", DataType::Int32, true),
        Field::new("percent", DataType::Float32, true),
    ];

    // Element schema
    let element_schema = vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("element_type", DataType::Utf8, false),
        Field::new("content", DataType::List(Field::new("item", DataType::Struct(content_schema.into()), false).into()), false),
        Field::new("extras", DataType::Utf8, false), // JSON serialized
    ];

    // Browser schema
    let browser_schema = vec![
        Field::new("agent", DataType::Utf8, false),
    ];

    // Device schema
    let device_schema = vec![
        Field::new("installation_id", DataType::Utf8, false),
        Field::new("manufacturer", DataType::Utf8, false),
        Field::new("model", DataType::Utf8, false),
        Field::new("platform", DataType::Utf8, false),
        Field::new("primary_locale", DataType::Utf8, false),
        Field::new("system_name", DataType::Utf8, false),
        Field::new("timezone", DataType::Utf8, false),
        Field::new("device_type", DataType::Utf8, false),
        Field::new("version", DataType::Utf8, false),
    ];

    // Geo schema
    let geo_schema = vec![
        Field::new("city", DataType::Utf8, true),
        Field::new("country", DataType::Utf8, true),
        Field::new("region", DataType::Utf8, true),
    ];

    // Context schema
    let context_schema = vec![
        Field::new("app_id", DataType::Utf8, false),
        Field::new("app_version", DataType::Utf8, false),
        Field::new("browser", DataType::Struct(browser_schema.into()), true),
        Field::new("client_id", DataType::Utf8, false),
        Field::new("device", DataType::Struct(device_schema.into()), false),
        Field::new("geo", DataType::Struct(geo_schema.into()), false),
        Field::new("session_id", DataType::Utf8, false),
        Field::new("user_id", DataType::Utf8, true),
    ];

    // Top level schema (flattened)
    Schema::new(vec![
        // Context fields
        Field::new("context", DataType::Struct(context_schema.into()), false),
        // Event fields
        Field::new("created", DataType::Int64, false),
        Field::new("created_micros", DataType::Int64, true),
        Field::new("event_type", DataType::Utf8, false),
        Field::new("element", DataType::Struct(element_schema.into()), false),
        // Timing fields
        Field::new("sent", DataType::Int64, false),
        Field::new("sent_micros", DataType::Int64, false),
    ])
}