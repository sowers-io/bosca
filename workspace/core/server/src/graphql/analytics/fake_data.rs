use crate::graphql::analytics::types::*;
use chrono::{DateTime, Duration, Utc};
use rand::prelude::*;
use rand::SeedableRng;
use serde_json::json;

pub struct FakeDataGenerator {
    rng: StdRng,
}

impl FakeDataGenerator {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_rng(&mut rand::rng()),
        }
    }

    #[allow(dead_code)]
    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn generate_time_series_data(
        &mut self,
        points: usize,
        start_time: DateTime<Utc>,
        interval: Duration,
        min_value: f64,
        max_value: f64,
    ) -> Vec<AnalyticDataRecord> {
        let mut data = Vec::with_capacity(points);
        let mut current_time = start_time;
        let mut last_value = (min_value + max_value) / 2.0;

        for _ in 0..points {
            let variation = self.rng.random_range(-0.2..=0.2);
            let new_value = (last_value * (1.0 + variation))
                .max(min_value)
                .min(max_value);
            
            data.push(AnalyticDataRecord::Single(AnalyticSingleDataRecord {
                x: Some(current_time.timestamp_millis() as f64),
                y: Some(new_value),
                attributes: Some(json!({
                    "timestamp": current_time.to_rfc3339(),
                })),
            }));

            last_value = new_value;
            current_time += interval;
        }

        data
    }

    pub fn generate_categorical_data(
        &mut self,
        categories: Vec<String>,
        min_value: f64,
        max_value: f64,
    ) -> Vec<AnalyticDataRecord> {
        categories
            .into_iter()
            .enumerate()
            .map(|(i, category)| {
                AnalyticDataRecord::Single(AnalyticSingleDataRecord {
                    x: Some(i as f64),
                    y: Some(self.rng.random_range(min_value..=max_value)),
                    attributes: Some(json!({
                        "category": category,
                    })),
                })
            })
            .collect()
    }

    pub fn generate_percentage_data(&mut self, segments: Vec<&str>) -> Vec<AnalyticDataRecord> {
        let mut remaining = 100.0;
        let segment_count = segments.len();
        
        segments
            .into_iter()
            .enumerate()
            .map(|(i, segment)| {
                let value = if i == segment_count - 1 {
                    remaining
                } else {
                    let max_value = remaining / (segment_count - i) as f64 * 1.5;
                    let value = self.rng.random_range(0.0..=max_value.min(remaining));
                    remaining -= value;
                    value
                };

                AnalyticDataRecord::Single(AnalyticSingleDataRecord {
                    x: Some(i as f64),
                    y: Some(value),
                    attributes: Some(json!({
                        "label": segment,
                        "percentage": value,
                    })),
                })
            })
            .collect()
    }

    pub fn generate_events(&mut self, count: usize, hours_back: i64) -> Vec<AnalyticEvent> {
        let mut events = Vec::with_capacity(count);
        let now = Utc::now();
        let start_time = now - Duration::hours(hours_back);

        let event_types = [
            AnalyticEventType::Session,
            AnalyticEventType::Interaction,
            AnalyticEventType::Impression,
            AnalyticEventType::Completion,
        ];

        let event_names = [
            "page_view",
            "button_click",
            "form_submit",
            "video_play",
            "download_start",
            "search_performed",
        ];

        let platforms = ["iOS", "Android", "Web", "Desktop"];
        let device_types = ["mobile", "tablet", "desktop"];
        let countries = ["US", "GB", "CA", "AU", "DE", "FR", "JP"];
        let cities = ["New York", "London", "Toronto", "Sydney", "Berlin", "Paris", "Tokyo"];

        for i in 0..count {
            let event_time = start_time + Duration::seconds(
                self.rng.random_range(0..=(hours_back * 3600))
            );

            let event_type = *event_types.choose(&mut self.rng).unwrap();
            let event_name = event_names.choose(&mut self.rng).unwrap();
            let platform = platforms.choose(&mut self.rng).unwrap();
            let device_type = device_types.choose(&mut self.rng).unwrap();
            let country = countries.choose(&mut self.rng).unwrap();
            let city = cities.choose(&mut self.rng).unwrap();

            events.push(AnalyticEvent {
                event_type,
                name: event_name.to_string(),
                client_id: format!("client_{}", self.rng.random_range(1000..9999)),
                created: event_time,
                element: Some(AnalyticElement {
                    id: format!("element_{}", i),
                    element_type: "button".to_string(),
                    content: None,
                    extras: Some(json!({
                        "color": "blue",
                        "size": "large",
                    })),
                }),
                context: Some(AnalyticContext {
                    app_id: Some("bosca_admin".to_string()),
                    app_version: Some("1.0.0".to_string()),
                    browser: Some(AnalyticBrowser {
                        agent: Some("Mozilla/5.0".to_string()),
                    }),
                    client_id: Some(format!("client_{}", self.rng.random_range(1000..9999))),
                    session_id: Some(format!("session_{}", self.rng.random_range(10000..99999))),
                    device: Some(AnalyticDevice {
                        installation_id: Some(format!("install_{}", self.rng.random_range(10000..99999))),
                        manufacturer: Some("Apple".to_string()),
                        model: Some("iPhone 14".to_string()),
                        platform: Some(platform.to_string()),
                        primary_locale: Some("en_US".to_string()),
                        system_name: Some("iOS".to_string()),
                        timezone: Some("America/New_York".to_string()),
                        device_type: Some(device_type.to_string()),
                        version: Some("16.0".to_string()),
                    }),
                    geo: Some(AnalyticGeo {
                        city: Some(city.to_string()),
                        country: Some(country.to_string()),
                        continent: Some("North America".to_string()),
                        longitude: Some(self.rng.random_range(-180.0..180.0)),
                        latitude: Some(self.rng.random_range(-90.0..90.0)),
                        region: Some("State".to_string()),
                        region_code: Some("ST".to_string()),
                        postal_code: Some("12345".to_string()),
                        timezone: Some("America/New_York".to_string()),
                    }),
                    user_id: Some(format!("user_{}", self.rng.random_range(100..999))),
                }),
                sent: Some(event_time + Duration::milliseconds(self.rng.random_range(10..100))),
                received: Some(event_time + Duration::milliseconds(self.rng.random_range(100..500))),
            });
        }

        events.sort_by_key(|e| e.created);
        events.reverse();
        events
    }
}