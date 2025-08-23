use crate::graphql::analytics::fake_data::FakeDataGenerator;
use crate::graphql::analytics::types::*;
use async_graphql::{Context, Error, Object};
use chrono::{Duration, Utc};

pub struct AnalyticQueriesResolvers;

#[Object(name = "AnalyticQueries")]
impl AnalyticQueriesResolvers {
    pub async fn events(
        &self,
        _ctx: &Context<'_>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<AnalyticEvent>, Error> {
        let offset = offset.unwrap_or(0).max(0) as usize;
        let limit = limit.unwrap_or(50).max(1).min(1000) as usize;

        let mut generator = FakeDataGenerator::new();
        let mut events = generator.generate_events(offset + limit + 100, 168); // 7 days back

        if offset >= events.len() {
            return Ok(vec![]);
        }

        let end = (offset + limit).min(events.len());
        events = events.into_iter().skip(offset).take(end - offset).collect();

        Ok(events)
    }

    pub async fn execute(
        &self,
        _ctx: &Context<'_>,
        request: AnalyticQueryRequest,
    ) -> Result<AnalyticQueryResponse, Error> {
        let mut generator = FakeDataGenerator::new();
        let query_id = request.id.to_string();

        match query_id.as_str() {
            // Time Series Queries
            "page-views-7d" => Ok(self.generate_page_views_7d(&mut generator)),
            "user-sessions-30d" => Ok(self.generate_user_sessions_30d(&mut generator)),
            "content-created-monthly" => Ok(self.generate_content_created_monthly(&mut generator)),
            
            // Categorical Queries
            "events-by-type" => Ok(self.generate_events_by_type(&mut generator)),
            "content-by-category" => Ok(self.generate_content_by_category(&mut generator)),
            "events-by-type-stacked" => Ok(self.generate_events_by_type_stacked(&mut generator)),
            
            // Geographic Queries
            "users-by-country" => Ok(self.generate_users_by_country(&mut generator)),
            "sessions-by-region" => Ok(self.generate_sessions_by_region(&mut generator)),
            
            // Percentage/Distribution Queries
            "device-types" => Ok(self.generate_device_types(&mut generator)),
            "browser-distribution" => Ok(self.generate_browser_distribution(&mut generator)),
            
            _ => Err(Error::new(format!("Unknown query ID: {}. Available queries: page-views-7d, user-sessions-30d, content-created-monthly, events-by-type, content-by-category, events-by-type-stacked, users-by-country, sessions-by-region, device-types, browser-distribution", query_id))),
        }
    }
}

impl AnalyticQueriesResolvers {
    fn generate_page_views_7d(&self, generator: &mut FakeDataGenerator) -> AnalyticQueryResponse {
        let now = Utc::now();
        let data = generator.generate_time_series_data(
            7,
            now - Duration::days(6),
            Duration::days(1),
            100.0,
            1000.0,
        );

        AnalyticQueryResponse {
            legend: None,
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
                            format: Some("%Y-%m-%d".to_string()),
                        },
                    },
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::Y,
                        label: AnalyticDataAxisLabel {
                            label: "Page Views".to_string(),
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

    fn generate_user_sessions_30d(&self, generator: &mut FakeDataGenerator) -> AnalyticQueryResponse {
        let now = Utc::now();
        let data = generator.generate_time_series_data(
            30,
            now - Duration::days(29),
            Duration::days(1),
            50.0,
            500.0,
        );

        AnalyticQueryResponse {
            legend: None,
            container: AnalyticDataContainer {
                container_type: AnalyticDataContainerType::Xy,
                scale: Some(AnalyticDataContainerScale::Time),
                graphs: vec![AnalyticDataContainerGraph {
                    graph_type: AnalyticQueryGraphType::Area,
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
                            number: Some(10),
                            format: Some("%m-%d".to_string()),
                        },
                    },
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::Y,
                        label: AnalyticDataAxisLabel {
                            label: "Sessions".to_string(),
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

    fn generate_events_by_type(&self, generator: &mut FakeDataGenerator) -> AnalyticQueryResponse {
        let categories = vec![
            "Session".to_string(),
            "Interaction".to_string(),
            "Impression".to_string(),
            "Completion".to_string(),
        ];

        let data = generator.generate_categorical_data(categories, 10.0, 100.0);

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
                            label: "Event Type".to_string(),
                            axis_type: AnalyticDataAxisType::X,
                            color: "#666".to_string(),
                            margin: None,
                            domain_line: true,
                        },
                        tick: AnalyticDataAxisTick {
                            enabled: true,
                            label_color: Some("#666".to_string()),
                            text_align: Some(AnalyticDataAxisTickTextAlign::Center),
                            text_angle: Some(-45.0),
                            text_width: None,
                            number: None,
                            format: None,
                        },
                    },
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::Y,
                        label: AnalyticDataAxisLabel {
                            label: "Count".to_string(),
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

    fn generate_device_types(&self, generator: &mut FakeDataGenerator) -> AnalyticQueryResponse {
        let segments = vec!["Mobile", "Desktop", "Tablet"];
        let data = generator.generate_percentage_data(segments);

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

    fn generate_content_created_monthly(&self, generator: &mut FakeDataGenerator) -> AnalyticQueryResponse {
        let now = Utc::now();
        let data = generator.generate_time_series_data(
            12,
            now - Duration::days(365),
            Duration::days(30),
            20.0,
            200.0,
        );

        AnalyticQueryResponse {
            legend: None,
            container: AnalyticDataContainer {
                container_type: AnalyticDataContainerType::Xy,
                scale: Some(AnalyticDataContainerScale::Time),
                graphs: vec![AnalyticDataContainerGraph {
                    graph_type: AnalyticQueryGraphType::Bar,
                }],
                axis: vec![
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::X,
                        label: AnalyticDataAxisLabel {
                            label: "Month".to_string(),
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
                            number: Some(12),
                            format: Some("%Y-%m".to_string()),
                        },
                    },
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::Y,
                        label: AnalyticDataAxisLabel {
                            label: "Content Created".to_string(),
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

    fn generate_content_by_category(&self, generator: &mut FakeDataGenerator) -> AnalyticQueryResponse {
        let categories = vec![
            "Articles".to_string(),
            "Videos".to_string(),
            "Images".to_string(),
            "Documents".to_string(),
            "Audio".to_string(),
        ];

        let data = generator.generate_categorical_data(categories, 25.0, 250.0);

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
                            label: "Content Category".to_string(),
                            axis_type: AnalyticDataAxisType::X,
                            color: "#666".to_string(),
                            margin: None,
                            domain_line: true,
                        },
                        tick: AnalyticDataAxisTick {
                            enabled: true,
                            label_color: Some("#666".to_string()),
                            text_align: Some(AnalyticDataAxisTickTextAlign::Center),
                            text_angle: Some(-45.0),
                            text_width: None,
                            number: None,
                            format: None,
                        },
                    },
                    AnalyticDataAxis {
                        axis_type: AnalyticDataAxisType::Y,
                        label: AnalyticDataAxisLabel {
                            label: "Count".to_string(),
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

    fn generate_events_by_type_stacked(&self, generator: &mut FakeDataGenerator) -> AnalyticQueryResponse {
        let now = Utc::now();
        let mut data = Vec::new();

        // Generate stacked bar data for the last 7 days
        for i in 0..7 {
            let day_offset = now - Duration::days(6 - i);
            let session_count = generator.generate_categorical_data(vec!["Session".to_string()], 10.0, 30.0)[0].clone();
            let interaction_count = generator.generate_categorical_data(vec!["Interaction".to_string()], 20.0, 60.0)[0].clone();
            let impression_count = generator.generate_categorical_data(vec!["Impression".to_string()], 30.0, 80.0)[0].clone();
            let completion_count = generator.generate_categorical_data(vec!["Completion".to_string()], 5.0, 25.0)[0].clone();

            if let (
                AnalyticDataRecord::Single(session),
                AnalyticDataRecord::Single(interaction), 
                AnalyticDataRecord::Single(impression),
                AnalyticDataRecord::Single(completion)
            ) = (session_count, interaction_count, impression_count, completion_count) {
                data.push(AnalyticDataRecord::Multi(AnalyticMultiDataRecord {
                    x: Some(day_offset.timestamp_millis() as f64),
                    y: Some(vec![
                        session.y.unwrap_or(0.0),
                        interaction.y.unwrap_or(0.0),
                        impression.y.unwrap_or(0.0),
                        completion.y.unwrap_or(0.0),
                    ]),
                    attributes: Some(serde_json::json!({
                        "date": day_offset.format("%Y-%m-%d").to_string(),
                        "types": ["Session", "Interaction", "Impression", "Completion"]
                    })),
                }));
            }
        }

        AnalyticQueryResponse {
            legend: Some(AnalyticDataLegend {
                legend_type: Some(AnalyticDataLegendType::Bullet),
                items: vec![
                    AnalyticDataLegendItem {
                        name: Some("Session".to_string()),
                        color: Some("#3B82F6".to_string()),
                        shape: Some(AnalyticDataLegendShape::Square),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Interaction".to_string()),
                        color: Some("#EF4444".to_string()),
                        shape: Some(AnalyticDataLegendShape::Square),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Impression".to_string()),
                        color: Some("#10B981".to_string()),
                        shape: Some(AnalyticDataLegendShape::Square),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Completion".to_string()),
                        color: Some("#F59E0B".to_string()),
                        shape: Some(AnalyticDataLegendShape::Square),
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
                    graph_type: AnalyticQueryGraphType::StackedBar,
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
                            label: "Event Count".to_string(),
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

    fn generate_users_by_country(&self, generator: &mut FakeDataGenerator) -> AnalyticQueryResponse {
        let countries = vec![
            "United States".to_string(),
            "United Kingdom".to_string(),
            "Canada".to_string(),
            "Germany".to_string(),
            "France".to_string(),
            "Japan".to_string(),
            "Australia".to_string(),
        ];

        let data = generator.generate_categorical_data(countries, 50.0, 500.0);

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
                            label: "Country".to_string(),
                            axis_type: AnalyticDataAxisType::X,
                            color: "#666".to_string(),
                            margin: None,
                            domain_line: true,
                        },
                        tick: AnalyticDataAxisTick {
                            enabled: true,
                            label_color: Some("#666".to_string()),
                            text_align: Some(AnalyticDataAxisTickTextAlign::Center),
                            text_angle: Some(-45.0),
                            text_width: None,
                            number: None,
                            format: None,
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

    fn generate_sessions_by_region(&self, generator: &mut FakeDataGenerator) -> AnalyticQueryResponse {
        let regions = vec![
            "North America".to_string(),
            "Europe".to_string(),
            "Asia Pacific".to_string(),
            "South America".to_string(),
            "Africa".to_string(),
        ];

        let data = generator.generate_categorical_data(regions, 100.0, 800.0);

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
                            label: "Region".to_string(),
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
                            label: "Sessions".to_string(),
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

    fn generate_browser_distribution(&self, generator: &mut FakeDataGenerator) -> AnalyticQueryResponse {
        let browsers = vec!["Chrome", "Safari", "Firefox", "Edge", "Other"];
        let data = generator.generate_percentage_data(browsers);

        AnalyticQueryResponse {
            legend: Some(AnalyticDataLegend {
                legend_type: Some(AnalyticDataLegendType::Bullet),
                items: vec![
                    AnalyticDataLegendItem {
                        name: Some("Chrome".to_string()),
                        color: Some("#4285F4".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Safari".to_string()),
                        color: Some("#00D4FF".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Firefox".to_string()),
                        color: Some("#FF6611".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Edge".to_string()),
                        color: Some("#0078D4".to_string()),
                        shape: Some(AnalyticDataLegendShape::Circle),
                        inactive: Some(false),
                        hidden: Some(false),
                        pointer: Some(true),
                    },
                    AnalyticDataLegendItem {
                        name: Some("Other".to_string()),
                        color: Some("#6B7280".to_string()),
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
}