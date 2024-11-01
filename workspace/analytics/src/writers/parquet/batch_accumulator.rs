use std::sync::Arc;
use arrow::array::{Array, ArrayRef, Float32Array, Float32Builder, Int32Array, Int32Builder, Int64Array, ListArray, ListBuilder, RecordBatch, StringArray, StringBuilder, StructArray, StructBuilder};
use arrow::buffer::Buffer;
use arrow::datatypes::{DataType, Field, Schema};
use crate::events::Events;

pub(crate) struct BatchAccumulator {
    schema: Arc<Schema>,
    data: Vec<Events>,
    events: usize,
    capacity: usize,
}

impl BatchAccumulator {
    pub fn new(schema: Arc<Schema>, capacity: usize) -> Self {
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
        // Pre-allocate vectors for all fields
        let mut app_ids = Vec::with_capacity(self.events);
        let mut app_versions = Vec::with_capacity(self.events);
        let mut client_ids = Vec::with_capacity(self.events);
        let mut session_ids = Vec::with_capacity(self.events);
        let mut user_ids = Vec::with_capacity(self.events);
        let mut browser_agents = Vec::with_capacity(self.events);

        let mut installation_ids = Vec::with_capacity(self.events);
        let mut manufacturers = Vec::with_capacity(self.events);
        let mut models = Vec::with_capacity(self.events);
        let mut platforms = Vec::with_capacity(self.events);
        let mut primary_locales = Vec::with_capacity(self.events);
        let mut system_names = Vec::with_capacity(self.events);
        let mut timezones = Vec::with_capacity(self.events);
        let mut device_types = Vec::with_capacity(self.events);
        let mut versions = Vec::with_capacity(self.events);

        let mut cities = Vec::with_capacity(self.events);
        let mut countries = Vec::with_capacity(self.events);
        let mut regions = Vec::with_capacity(self.events);

        let mut createds = Vec::with_capacity(self.events);
        let mut created_micros = Vec::with_capacity(self.events);
        let mut event_types = Vec::with_capacity(self.events);
        let mut element_ids = Vec::with_capacity(self.events);
        let mut element_types = Vec::with_capacity(self.events);
        let mut element_extras = Vec::with_capacity(self.events);

        let mut sents = Vec::with_capacity(self.events);
        let mut sent_micros = Vec::with_capacity(self.events);

        // Collect all content data
        let mut all_content_ids = Vec::new();
        let mut all_content_types = Vec::new();
        let mut all_content_indexes = Vec::new();
        let mut all_content_percents = Vec::new();
        let mut content_offsets = vec![0];
        let mut running_offset = 0;

        // Process all records
        for events in std::mem::take(&mut self.data) {
            for event in events.events {
                // Context fields
                app_ids.push(events.context.app_id.to_owned());
                app_versions.push(events.context.app_version.to_owned());
                client_ids.push(events.context.client_id.to_owned());
                session_ids.push(events.context.session_id.to_owned());
                user_ids.push(events.context.user_id.to_owned());
                browser_agents.push(events.context.browser.as_ref().map(|b| b.agent.to_owned()));

                // Device fields
                installation_ids.push(events.context.device.installation_id.to_owned());
                manufacturers.push(events.context.device.manufacturer.to_owned());
                models.push(events.context.device.model.to_owned());
                platforms.push(events.context.device.platform.to_owned());
                primary_locales.push(events.context.device.primary_locale.to_owned());
                system_names.push(events.context.device.system_name.to_owned());
                timezones.push(events.context.device.timezone.to_owned());
                device_types.push(events.context.device.device_type.to_owned());
                versions.push(events.context.device.version.to_owned());

                // Geo fields
                cities.push(events.context.geo.city.to_owned());
                countries.push(events.context.geo.country.to_owned());
                regions.push(events.context.geo.region.to_owned());

                // Event fields
                createds.push(event.created);
                created_micros.push(event.created_micros);
                event_types.push(format!("{:?}", event.event_type));
                element_ids.push(event.element.id);
                element_types.push(event.element.element_type);
                element_extras.push(serde_json::to_string(&event.element.extras)?);

                // Content fields with offset tracking
                let contents = event.element.content;
                running_offset += contents.len();
                content_offsets.push(running_offset);

                for content in contents {
                    all_content_ids.push(content.id);
                    all_content_types.push(content.content_type);
                    all_content_indexes.push(content.index);
                    all_content_percents.push(content.percent);
                }

                // Timing fields
                sents.push(events.sent);
                sent_micros.push(events.sent_micros);
            }
        }

        // Build browser array (optional)
        let browser_array = StructArray::try_from(vec![
            ("agent", Arc::new(StringArray::from_iter(browser_agents)) as ArrayRef),
        ])?;

        // Build device array
        let device_array = StructArray::try_from(vec![
            ("installation_id", Arc::new(StringArray::from_iter_values(installation_ids)) as ArrayRef),
            ("manufacturer", Arc::new(StringArray::from_iter_values(manufacturers)) as ArrayRef),
            ("model", Arc::new(StringArray::from_iter_values(models)) as ArrayRef),
            ("platform", Arc::new(StringArray::from_iter_values(platforms)) as ArrayRef),
            ("primary_locale", Arc::new(StringArray::from_iter_values(primary_locales)) as ArrayRef),
            ("system_name", Arc::new(StringArray::from_iter_values(system_names)) as ArrayRef),
            ("timezone", Arc::new(StringArray::from_iter_values(timezones)) as ArrayRef),
            ("device_type", Arc::new(StringArray::from_iter_values(device_types)) as ArrayRef),
            ("version", Arc::new(StringArray::from_iter_values(versions)) as ArrayRef),
        ])?;

        // Build geo array
        let geo_array = StructArray::try_from(vec![
            ("city", Arc::new(StringArray::from_iter(cities)) as ArrayRef),
            ("country", Arc::new(StringArray::from_iter(countries)) as ArrayRef),
            ("region", Arc::new(StringArray::from_iter(regions)) as ArrayRef),
        ])?;

        // Build context array
        let context_array = StructArray::try_from(vec![
            ("app_id", Arc::new(StringArray::from_iter_values(app_ids)) as ArrayRef),
            ("app_version", Arc::new(StringArray::from_iter_values(app_versions)) as ArrayRef),
            ("browser", Arc::new(browser_array) as ArrayRef),
            ("client_id", Arc::new(StringArray::from_iter_values(client_ids)) as ArrayRef),
            ("device", Arc::new(device_array) as ArrayRef),
            ("geo", Arc::new(geo_array) as ArrayRef),
            ("session_id", Arc::new(StringArray::from_iter_values(session_ids)) as ArrayRef),
            ("user_id", Arc::new(StringArray::from_iter(user_ids)) as ArrayRef),
        ])?;

        // Build content struct array
        let content_struct = StructArray::try_from(vec![
            ("id", Arc::new(StringArray::from_iter_values(all_content_ids)) as ArrayRef),
            ("content_type", Arc::new(StringArray::from_iter_values(all_content_types)) as ArrayRef),
            ("index", Arc::new(Int32Array::from_iter(all_content_indexes)) as ArrayRef),
            ("percent", Arc::new(Float32Array::from_iter(all_content_percents)) as ArrayRef),
        ])?;

        // Build element array
        let element_array = StructArray::try_from(vec![
            ("id", Arc::new(StringArray::from_iter_values(element_ids)) as ArrayRef),
            ("element_type", Arc::new(StringArray::from_iter_values(element_types)) as ArrayRef),
            ("content", Arc::new(content_struct) as ArrayRef),
            ("extras", Arc::new(StringArray::from_iter_values(element_extras)) as ArrayRef),
        ])?;

        let columns = vec![
            Arc::new(context_array) as ArrayRef,
            Arc::new(Int64Array::from_iter_values(createds)) as ArrayRef,
            Arc::new(Int64Array::from_iter(created_micros)) as ArrayRef,
            Arc::new(StringArray::from_iter_values(event_types)) as ArrayRef,
            Arc::new(element_array) as ArrayRef,
            Arc::new(Int64Array::from_iter_values(sents)) as ArrayRef,
            Arc::new(Int64Array::from_iter_values(sent_micros)) as ArrayRef,
        ];

        let batch = RecordBatch::try_new(self.schema.clone(), columns)?;

        self.data.clear();
        self.events = 0;

        Ok(batch)
    }
}