use crate::graphql::analytics::types::*;
use chrono::{DateTime, Datelike, Duration, Utc};
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

    pub fn generate_daily_active_users(&mut self, date_range: Option<DateRange>) -> Vec<DailyActiveUserRecord> {
        let (start, end) = self.get_date_range(date_range, 30);
        let mut records = Vec::new();
        let mut current = start;

        while current <= end {
            let base_count = 500 + (current.ordinal() % 100) as i32;
            let variation = self.rng.random_range(-100..=100);
            let count = (base_count + variation).max(50);

            records.push(DailyActiveUserRecord {
                date: current,
                count,
            });
            current += Duration::days(1);
        }

        records
    }

    pub fn generate_weekly_active_users(&mut self, date_range: Option<DateRange>) -> Vec<WeeklyActiveUserRecord> {
        let (start, end) = self.get_date_range(date_range, 84);
        let mut records = Vec::new();
        let mut current = start;

        while current <= end {
            let base_count = 2500 + (current.ordinal() % 500) as i32;
            let variation = self.rng.random_range(-500..=500);
            let count = (base_count + variation).max(200);

            records.push(WeeklyActiveUserRecord {
                week: current,
                count,
            });
            current += Duration::weeks(1);
        }

        records
    }

    pub fn generate_new_users(&mut self, date_range: Option<DateRange>) -> Vec<NewUserRecord> {
        let (start, end) = self.get_date_range(date_range, 30);
        let mut records = Vec::new();
        let mut current = start;

        while current <= end {
            let base_count = 25 + (current.ordinal() % 20) as i32;
            let variation = self.rng.random_range(-10..=15);
            let count = (base_count + variation).max(0);

            records.push(NewUserRecord {
                date: current,
                count,
            });
            current += Duration::days(1);
        }

        records
    }

    pub fn generate_returning_users(&mut self, date_range: Option<DateRange>) -> Vec<ReturningUserRecord> {
        let (start, end) = self.get_date_range(date_range, 30);
        let mut records = Vec::new();
        let mut current = start;

        while current <= end {
            let base_percentage = 65.0 + (current.ordinal() % 20) as f64;
            let variation = self.rng.random_range(-10.0..=10.0);
            let percentage = (base_percentage + variation).clamp(20.0_f64, 95.0);

            records.push(ReturningUserRecord {
                date: current,
                percentage,
            });
            current += Duration::days(1);
        }

        records
    }

    pub fn generate_retention_flow(&mut self, _date_range: Option<DateRange>) -> RetentionFlowResponse {
        let now = Utc::now();
        let mut cohorts = Vec::new();

        for i in 0..8 {
            let cohort_date = now - Duration::weeks(i);
            let base_users = 1000 + self.rng.random_range(-200..=200);
            
            let mut retention_periods = Vec::new();
            for period in 0..8 {
                let base_retention = 100.0 * (0.8_f64.powf(period as f64));
                let variation = self.rng.random_range(-5.0..=5.0);
                let retention = (base_retention + variation).clamp(5.0, 100.0);
                retention_periods.push(retention);
            }

            cohorts.push(RetentionCohort {
                cohort_date,
                users_count: base_users,
                retention_periods,
            });
        }

        RetentionFlowResponse { cohorts }
    }

    pub fn generate_session_totals(&mut self, date_range: Option<DateRange>) -> Vec<SessionTotalRecord> {
        let (start, end) = self.get_date_range(date_range, 30);
        let mut records = Vec::new();
        let mut current = start;

        while current <= end {
            let base_sessions = 800 + (current.ordinal() % 200) as i32;
            let variation = self.rng.random_range(-150..=150);
            let session_count = (base_sessions + variation).max(100);

            records.push(SessionTotalRecord {
                date: current,
                session_count,
            });
            current += Duration::days(1);
        }

        records
    }

    pub fn generate_session_durations(&mut self, date_range: Option<DateRange>) -> Vec<SessionDurationRecord> {
        let (start, end) = self.get_date_range(date_range, 30);
        let mut records = Vec::new();
        let mut current = start;

        while current <= end {
            let base_duration = 180.0 + (current.ordinal() % 60) as f64;
            let variation = self.rng.random_range(-30.0..=60.0);
            let average_duration = (base_duration + variation).max(30.0);

            records.push(SessionDurationRecord {
                date: current,
                average_duration,
            });
            current += Duration::days(1);
        }

        records
    }

    pub fn generate_sessions_by_device(&mut self, _date_range: Option<DateRange>) -> Vec<SessionDeviceRecord> {
        let devices = vec!["mobile", "desktop", "tablet"];
        let mut records = Vec::new();

        for device in devices {
            let base_sessions = match device {
                "mobile" => 2000,
                "desktop" => 1500,
                "tablet" => 500,
                _ => 100,
            };
            let variation = self.rng.random_range(-200..=200);
            let session_count = (base_sessions + variation).max(50);

            let base_duration: f64 = match device {
                "mobile" => 120.0,
                "desktop" => 300.0,
                "tablet" => 200.0,
                _ => 150.0,
            };
            let duration_variation = self.rng.random_range(-30.0..=60.0);
            let average_duration = (base_duration + duration_variation).max(30.0);

            records.push(SessionDeviceRecord {
                device_type: device.to_string(),
                session_count,
                average_duration,
            });
        }

        records
    }

    pub fn generate_session_metrics(&mut self, date_range: Option<DateRange>) -> SessionMetrics {
        let device_breakdown = self.generate_sessions_by_device(date_range.clone());
        let total_sessions = device_breakdown.iter().map(|d| d.session_count).sum();
        let total_duration: f64 = device_breakdown.iter()
            .map(|d| d.average_duration * d.session_count as f64)
            .sum();
        let average_duration = if total_sessions > 0 {
            total_duration / total_sessions as f64
        } else {
            0.0
        };

        SessionMetrics {
            total_sessions,
            average_duration,
            device_breakdown,
        }
    }

    pub fn generate_content_views(&mut self, content_id: Option<String>, date_range: Option<DateRange>) -> Vec<ContentViewRecord> {
        let (start, end) = self.get_date_range(date_range, 30);
        let mut records = Vec::new();
        let mut current = start;

        let content_ids = if let Some(id) = content_id {
            vec![id]
        } else {
            (1..=10).map(|i| format!("content_{}", i)).collect()
        };

        while current <= end {
            for content_id in &content_ids {
                let base_views = 50 + (current.ordinal() % 30) as i32;
                let variation = self.rng.random_range(-20..=40);
                let view_count = (base_views + variation).max(0);

                records.push(ContentViewRecord {
                    content_id: content_id.clone(),
                    date: current,
                    view_count,
                });
            }
            current += Duration::days(1);
        }

        records
    }

    pub fn generate_top_content(&mut self, limit: Option<i32>, _date_range: Option<DateRange>) -> Vec<TopContentRecord> {
        let limit = limit.unwrap_or(10).min(50) as usize;
        let mut records = Vec::new();

        let titles = [
            "Getting Started Guide", "Advanced Features", "Troubleshooting Tips",
            "Best Practices", "API Documentation", "User Management", "Security Settings",
            "Performance Optimization", "Integration Guide", "Migration Tutorial"
        ];

        for i in 0..limit {
            let title = titles.get(i % titles.len()).unwrap_or(&"Content Item");
            let base_views = 2000 - (i * 150);
            let variation = self.rng.random_range(-200..=300);
            let view_count = (base_views as i32 + variation).max(100);

            let base_engagement = 85.0 - (i as f64 * 5.0);
            let engagement_variation = self.rng.random_range(-10.0..=10.0);
            let engagement_score = (base_engagement + engagement_variation).clamp(20.0, 100.0);

            records.push(TopContentRecord {
                content_id: format!("content_{}", i + 1),
                title: title.to_string(),
                view_count,
                engagement_score,
            });
        }

        records
    }

    pub fn generate_content_engagement(&mut self, content_id: Option<String>, _date_range: Option<DateRange>) -> Vec<EngagementRecord> {
        let content_ids = if let Some(id) = content_id {
            vec![id]
        } else {
            (1..=5).map(|i| format!("content_{}", i)).collect()
        };

        content_ids.into_iter().map(|content_id| {
            let views = self.rng.random_range(500..=5000);
            let likes = (views as f64 * self.rng.random_range(0.02..=0.15)) as i32;
            let shares = (views as f64 * self.rng.random_range(0.005..=0.03)) as i32;
            let comments = (views as f64 * self.rng.random_range(0.01..=0.08)) as i32;
            let time_spent = self.rng.random_range(60.0..=600.0);

            EngagementRecord {
                content_id,
                views,
                likes,
                shares,
                comments,
                time_spent,
            }
        }).collect()
    }

    pub fn generate_content_metrics(&mut self, date_range: Option<DateRange>) -> ContentMetrics {
        let top_content = self.generate_top_content(Some(5), date_range.clone());
        let total_views = top_content.iter().map(|c| c.view_count).sum();
        let unique_viewers = (total_views as f64 * 0.7) as i32;
        let average_engagement_time = self.rng.random_range(120.0..=300.0);

        ContentMetrics {
            total_views,
            unique_viewers,
            average_engagement_time,
            top_content,
        }
    }

    pub fn generate_device_types_enhanced(&mut self, _date_range: Option<DateRange>) -> AnalyticQueryResponse {
        let segments = vec!["Mobile", "Desktop", "Tablet", "Smart TV"];
        let data = self.generate_percentage_data(segments);

        AnalyticQueryResponse {
            legend: Some(AnalyticDataLegend {
                legend_type: Some(AnalyticDataLegendType::Bullet),
                items: vec![
                    AnalyticDataLegendItem {
                        name: Some("Mobile".to_string()),
                        color: Some("#3B82F6".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Desktop".to_string()),
                        color: Some("#EF4444".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Tablet".to_string()),
                        color: Some("#10B981".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Smart TV".to_string()),
                        color: Some("#F59E0B".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                ],
            }),
            container: AnalyticDataContainer {
                container_type: AnalyticDataContainerType::Single,
                scale: None,
                graphs: vec![AnalyticDataContainerGraph {
                    graph_type: AnalyticQueryGraphType::Donut,
                }],
                axis: vec![],
                data,
            },
            annotations: vec![],
        }
    }

    pub fn generate_device_adoption_trends(&mut self, date_range: Option<DateRange>) -> AnalyticQueryResponse {
        let (start, _) = self.get_date_range(date_range, 90);
        let mut data = Vec::new();

        for i in 0..30 {
            let day_offset = start + Duration::days(i);
            let mobile_count = 100.0 + (i as f64 * 2.0) + self.rng.random_range(-10.0..=10.0);
            let desktop_count = 80.0 + (i as f64 * 0.5) + self.rng.random_range(-8.0..=8.0);
            let tablet_count = 20.0 + (i as f64 * 0.2) + self.rng.random_range(-3.0..=3.0);

            data.push(AnalyticDataRecord::Multi(AnalyticMultiDataRecord {
                x: Some(day_offset.timestamp_millis() as f64),
                y: Some(vec![mobile_count, desktop_count, tablet_count]),
                attributes: Some(serde_json::json!({
                    "date": day_offset.format("%Y-%m-%d").to_string(),
                    "types": ["Mobile", "Desktop", "Tablet"]
                })),
            }));
        }

        AnalyticQueryResponse {
            legend: Some(AnalyticDataLegend {
                legend_type: Some(AnalyticDataLegendType::Bullet),
                items: vec![
                    AnalyticDataLegendItem {
                        name: Some("Mobile".to_string()),
                        color: Some("#3B82F6".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Desktop".to_string()),
                        color: Some("#EF4444".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Tablet".to_string()),
                        color: Some("#10B981".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                ],
            }),
            container: AnalyticDataContainer {
                container_type: AnalyticDataContainerType::Xy,
                scale: Some(AnalyticDataContainerScale::Time),
                graphs: vec![AnalyticDataContainerGraph {
                    graph_type: AnalyticQueryGraphType::Line,
                }],
                axis: vec![
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::X,
                        label: AnalyticDataAxisLabel {
                            label: "Date".to_string(),
                            axis_type: AnalyticDataAxisType::X,
                            color: "#666".to_string(),
                            margin: None,
                            domain_line: true,
                        },
                        tick: AnalyticDataAxisTick {
                            enabled: true,
                            label_color: Some("#666".to_string()),
                            text_align: Some(AnalyticDataAxisTickTextAlign::Center),
                            text_angle: None,
                            text_width: None,
                            number: Some(7),
                            format: Some("%m-%d".to_string()),
                        },
                    },
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::Y,
                        label: AnalyticDataAxisLabel {
                            label: "Users".to_string(),
                            axis_type: AnalyticDataAxisType::Y,
                            color: "#666".to_string(),
                            margin: None,
                            domain_line: true,
                        },
                        tick: AnalyticDataAxisTick {
                            enabled: true,
                            label_color: Some("#666".to_string()),
                            text_align: Some(AnalyticDataAxisTickTextAlign::Right),
                            text_angle: None,
                            text_width: None,
                            number: Some(5),
                            format: None,
                        },
                    },
                ],
                data,
            },
            annotations: vec![],
        }
    }

    pub fn generate_device_performance_metrics(&mut self, _date_range: Option<DateRange>) -> AnalyticQueryResponse {
        let devices = vec!["Mobile", "Desktop", "Tablet"]
            .into_iter().map(|s| s.to_string()).collect();
        let data = self.generate_categorical_data(devices, 50.0, 300.0);

        AnalyticQueryResponse {
            legend: None,
            container: AnalyticDataContainer {
                container_type: AnalyticDataContainerType::Single,
                scale: None,
                graphs: vec![AnalyticDataContainerGraph {
                    graph_type: AnalyticQueryGraphType::Bar,
                }],
                axis: vec![
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::X,
                        label: AnalyticDataAxisLabel {
                            label: "Device Type".to_string(),
                            axis_type: AnalyticDataAxisType::X,
                            color: "#666".to_string(),
                            margin: None,
                            domain_line: true,
                        },
                        tick: AnalyticDataAxisTick {
                            enabled: true,
                            label_color: Some("#666".to_string()),
                            text_align: Some(AnalyticDataAxisTickTextAlign::Center),
                            text_angle: None,
                            text_width: None,
                            number: None,
                            format: None,
                        },
                    },
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::Y,
                        label: AnalyticDataAxisLabel {
                            label: "Performance Score".to_string(),
                            axis_type: AnalyticDataAxisType::Y,
                            color: "#666".to_string(),
                            margin: None,
                            domain_line: true,
                        },
                        tick: AnalyticDataAxisTick {
                            enabled: true,
                            label_color: Some("#666".to_string()),
                            text_align: Some(AnalyticDataAxisTickTextAlign::Right),
                            text_angle: None,
                            text_width: None,
                            number: Some(5),
                            format: None,
                        },
                    },
                ],
                data,
            },
            annotations: vec![],
        }
    }

    pub fn generate_device_user_behavior(&mut self, _date_range: Option<DateRange>) -> AnalyticQueryResponse {
        let behaviors = vec!["Browse", "Search", "Purchase", "Share"]
            .into_iter().map(|s| s.to_string()).collect();
        let data = self.generate_categorical_data(behaviors, 20.0, 100.0);

        AnalyticQueryResponse {
            legend: None,
            container: AnalyticDataContainer {
                container_type: AnalyticDataContainerType::Single,
                scale: None,
                graphs: vec![AnalyticDataContainerGraph {
                    graph_type: AnalyticQueryGraphType::Bar,
                }],
                axis: vec![
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::X,
                        label: AnalyticDataAxisLabel {
                            label: "Behavior".to_string(),
                            axis_type: AnalyticDataAxisType::X,
                            color: "#666".to_string(),
                            margin: None,
                            domain_line: true,
                        },
                        tick: AnalyticDataAxisTick {
                            enabled: true,
                            label_color: Some("#666".to_string()),
                            text_align: Some(AnalyticDataAxisTickTextAlign::Center),
                            text_angle: None,
                            text_width: None,
                            number: None,
                            format: None,
                        },
                    },
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::Y,
                        label: AnalyticDataAxisLabel {
                            label: "Frequency".to_string(),
                            axis_type: AnalyticDataAxisType::Y,
                            color: "#666".to_string(),
                            margin: None,
                            domain_line: true,
                        },
                        tick: AnalyticDataAxisTick {
                            enabled: true,
                            label_color: Some("#666".to_string()),
                            text_align: Some(AnalyticDataAxisTickTextAlign::Right),
                            text_angle: None,
                            text_width: None,
                            number: Some(5),
                            format: None,
                        },
                    },
                ],
                data,
            },
            annotations: vec![],
        }
    }

    fn get_date_range(&self, date_range: Option<DateRange>, default_days: i64) -> (DateTime<Utc>, DateTime<Utc>) {
        if let Some(range) = date_range {
            (range.start, range.end)
        } else {
            let end = Utc::now();
            let start = end - Duration::days(default_days);
            (start, end)
        }
    }
}