use std::sync::Arc;
use arrow::array::{ArrayRef, Float32Builder, Int32Builder, Int64Builder, ListBuilder, RecordBatch, StringBuilder, StructBuilder, TimestampMillisecondBuilder};
use crate::events::Events;
use crate::writers::arrow::schema::SchemaDefinition;

pub(crate) struct BatchAccumulator {
    schema: Arc<SchemaDefinition>,
    data: Vec<Events>,
    events: usize,
    capacity: usize,
}

impl BatchAccumulator {
    pub fn new(schema: Arc<SchemaDefinition>, capacity: usize) -> Self {
        Self {
            schema,
            data: Vec::with_capacity(capacity),
            events: 0,
            capacity,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.events == 0
    }

    pub fn add_batch(&mut self, events: Events) {
        self.events += events.events.len();
        self.data.push(events);
    }

    pub fn is_full(&self) -> bool {
        self.data.len() >= self.capacity
    }

    pub fn build(&mut self) -> Result<RecordBatch, Box<dyn std::error::Error>> {
        // Context struct builders
        let context_app_id_builder = StringBuilder::new();
        let context_app_version_builder = StringBuilder::new();
        let context_client_id_builder = StringBuilder::new();
        let context_session_id_builder = StringBuilder::new();
        let context_user_id_builder = StringBuilder::new();

        // Browser struct builders
        let browser_agent_builder = StringBuilder::new();
        let browser_struct_builder = StructBuilder::new(
            self.schema.browser_struct.clone(),
            vec![Box::new(browser_agent_builder)],
        );

        // Device struct builders
        let device_installation_id_builder = StringBuilder::new();
        let device_manufacturer_builder = StringBuilder::new();
        let device_model_builder = StringBuilder::new();
        let device_platform_builder = StringBuilder::new();
        let device_locale_builder = StringBuilder::new();
        let device_system_builder = StringBuilder::new();
        let device_timezone_builder = StringBuilder::new();
        let device_type_builder = StringBuilder::new();
        let device_version_builder = StringBuilder::new();

        let device_struct_builder = StructBuilder::new(
            self.schema.device_struct.clone(),
            vec![
                Box::new(device_installation_id_builder),
                Box::new(device_manufacturer_builder),
                Box::new(device_model_builder),
                Box::new(device_platform_builder),
                Box::new(device_locale_builder),
                Box::new(device_system_builder),
                Box::new(device_timezone_builder),
                Box::new(device_type_builder),
                Box::new(device_version_builder),
            ],
        );

        // Geo struct builders
        let geo_city_builder = StringBuilder::new();
        let geo_country_builder = StringBuilder::new();
        let geo_region_builder = StringBuilder::new();

        let geo_struct_builder = StructBuilder::new(
            self.schema.geo_struct.clone(),
            vec![
                Box::new(geo_city_builder),
                Box::new(geo_country_builder),
                Box::new(geo_region_builder),
            ],
        );

        let mut context_struct_builder = StructBuilder::new(
            self.schema.context_struct.clone(),
            vec![
                Box::new(context_app_id_builder),
                Box::new(context_app_version_builder),
                Box::new(browser_struct_builder),
                Box::new(context_client_id_builder),
                Box::new(device_struct_builder),
                Box::new(geo_struct_builder),
                Box::new(context_session_id_builder),
                Box::new(context_user_id_builder),
            ],
        );

        // Element struct builders
        let content_id_builder = StringBuilder::new();
        let content_type_builder = StringBuilder::new();
        let content_index_builder = Int32Builder::new();
        let content_percent_builder = Float32Builder::new();

        let content_struct_builder = StructBuilder::new(
            self.schema.content_struct.clone(),
            vec![
                Box::new(content_id_builder),
                Box::new(content_type_builder),
                Box::new(content_index_builder),
                Box::new(content_percent_builder),
            ],
        );

        let element_id_builder = StringBuilder::new();
        let element_type_builder = StringBuilder::new();
        let element_content_builder = ListBuilder::new(content_struct_builder);
        let element_extras_builder = StringBuilder::new();

        let mut element_struct_builder = StructBuilder::new(
            self.schema.element_struct.clone(),
            vec![
                Box::new(element_id_builder),
                Box::new(element_type_builder),
                Box::new(element_content_builder),
                Box::new(element_extras_builder),
            ],
        );

        // Top-level event builders
        let mut created_builder = TimestampMillisecondBuilder::new().with_timezone("UTC");
        let mut created_micros_builder = Int64Builder::new();
        let mut type_builder = StringBuilder::new();
        let mut sent_builder = TimestampMillisecondBuilder::new().with_timezone("UTC");
        let mut sent_micros_builder = Int64Builder::new();

        // Populate data for each event
        for events in &self.data {
            for event in &events.events {
                let context = &events.context;

                // Browser data
                let browser_struct_builder = context_struct_builder.field_builder::<StructBuilder>(2).unwrap();
                if let Some(browser) = &context.browser {
                    browser_struct_builder.field_builder::<StringBuilder>(0).unwrap()
                        .append_value(&browser.agent);
                    browser_struct_builder.append(true);
                } else {
                    browser_struct_builder.field_builder::<StringBuilder>(0).unwrap()
                        .append_value("");
                    browser_struct_builder.append(false);
                }

                // Device data
                let device = &context.device;
                let device_struct_builder = context_struct_builder.field_builder::<StructBuilder>(4).unwrap();
                device_struct_builder.field_builder::<StringBuilder>(0).unwrap().append_value(&device.installation_id);
                device_struct_builder.field_builder::<StringBuilder>(1).unwrap().append_value(&device.manufacturer);
                device_struct_builder.field_builder::<StringBuilder>(2).unwrap().append_value(&device.model);
                device_struct_builder.field_builder::<StringBuilder>(3).unwrap().append_value(&device.platform);
                device_struct_builder.field_builder::<StringBuilder>(4).unwrap().append_value(&device.primary_locale);
                device_struct_builder.field_builder::<StringBuilder>(5).unwrap().append_value(&device.system_name);
                device_struct_builder.field_builder::<StringBuilder>(6).unwrap().append_value(&device.timezone);
                device_struct_builder.field_builder::<StringBuilder>(7).unwrap().append_value(&device.device_type);
                device_struct_builder.field_builder::<StringBuilder>(8).unwrap().append_value(&device.version);
                device_struct_builder.append(true);

                // Geo data
                let geo = &context.geo;
                let geo_struct_builder = context_struct_builder.field_builder::<StructBuilder>(5).unwrap();
                if let Some(city) = &geo.city {
                    geo_struct_builder.field_builder::<StringBuilder>(0).unwrap().append_value(city);
                } else {
                    geo_struct_builder.field_builder::<StringBuilder>(0).unwrap().append_null();
                }
                if let Some(country) = &geo.country {
                    geo_struct_builder.field_builder::<StringBuilder>(1).unwrap().append_value(country);
                } else {
                    geo_struct_builder.field_builder::<StringBuilder>(1).unwrap().append_null();
                }
                if let Some(region) = &geo.region {
                    geo_struct_builder.field_builder::<StringBuilder>(2).unwrap().append_value(region);
                } else {
                    geo_struct_builder.field_builder::<StringBuilder>(2).unwrap().append_null();
                }
                geo_struct_builder.append(true);

                // Context data
                context_struct_builder.field_builder::<StringBuilder>(0).unwrap().append_value(&context.app_id);
                context_struct_builder.field_builder::<StringBuilder>(1).unwrap().append_value(&context.app_version);
                context_struct_builder.field_builder::<StringBuilder>(3).unwrap().append_value(&context.client_id);
                context_struct_builder.field_builder::<StringBuilder>(6).unwrap().append_value(&context.session_id);
                if let Some(user_id) = &context.user_id {
                    context_struct_builder.field_builder::<StringBuilder>(7).unwrap().append_value(user_id);
                } else {
                    context_struct_builder.field_builder::<StringBuilder>(7).unwrap().append_null();
                }
                context_struct_builder.append(true);

                // Element data
                let element = &event.element;
                element_struct_builder.field_builder::<StringBuilder>(0).unwrap().append_value(&element.id);
                element_struct_builder.field_builder::<StringBuilder>(1).unwrap().append_value(&element.element_type);

                // Content list
                let content_builder = element_struct_builder.field_builder::<ListBuilder<StructBuilder>>(2).unwrap();
                for content in &element.content {
                    let struct_builder = content_builder.values();
                    struct_builder.field_builder::<StringBuilder>(0).unwrap().append_value(&content.id);
                    struct_builder.field_builder::<StringBuilder>(1).unwrap().append_value(&content.content_type);

                    if let Some(index) = content.index {
                        struct_builder.field_builder::<Int32Builder>(2).unwrap().append_value(index);
                    } else {
                        struct_builder.field_builder::<Int32Builder>(2).unwrap().append_null();
                    }

                    if let Some(percent) = content.percent {
                        struct_builder.field_builder::<Float32Builder>(3).unwrap().append_value(percent);
                    } else {
                        struct_builder.field_builder::<Float32Builder>(3).unwrap().append_null();
                    }

                    struct_builder.append(true);
                }
                content_builder.append(!element.content.is_empty());

                let extras_json = serde_json::to_string(&element.extras).unwrap_or_default();
                element_struct_builder.field_builder::<StringBuilder>(3).unwrap().append_value(&extras_json);
                element_struct_builder.append(true);

                // Event data
                created_builder.append_value(event.created);
                if let Some(created_micros) = event.created_micros {
                    created_micros_builder.append_value(created_micros);
                } else {
                    created_micros_builder.append_null();
                }
                type_builder.append_value(format!("{:?}", event.event_type));

                // Sent timestamps
                sent_builder.append_value(events.sent);
                sent_micros_builder.append_value(events.sent_micros);
            }
        }

        // Build the final RecordBatch
        let arrays: Vec<ArrayRef> = vec![
            Arc::new(context_struct_builder.finish()),
            Arc::new(created_builder.finish()),
            Arc::new(created_micros_builder.finish()),
            Arc::new(type_builder.finish()),
            Arc::new(element_struct_builder.finish()),
            Arc::new(sent_builder.finish()),
            Arc::new(sent_micros_builder.finish()),
        ];

        let batch = RecordBatch::try_new(self.schema.schema.clone(), arrays)?;

        self.data.clear();
        self.events = 0;

        Ok(batch)
    }
}