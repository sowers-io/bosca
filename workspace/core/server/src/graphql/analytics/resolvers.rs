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
            "page-views-7d" => Ok(self.generate_page_views_7d(&mut generator)),
            "user-sessions-30d" => Ok(self.generate_user_sessions_30d(&mut generator)),
            "events-by-type" => Ok(self.generate_events_by_type(&mut generator)),
            "device-types" => Ok(self.generate_device_types(&mut generator)),
            "content-created-monthly" => Ok(self.generate_content_created_monthly(&mut generator)),
            _ => Err(Error::new(format!("Unknown query ID: {}", query_id))),
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
}