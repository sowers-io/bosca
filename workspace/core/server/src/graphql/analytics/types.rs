use async_graphql::{Enum, InputObject, SimpleObject, Union, ID};
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticQueryGraphType {
    Line,
    Bar,
    StackedBar,
    Area,
    Donut,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticQueryGraphCurveType {
    Basis,
}

#[derive(SimpleObject)]
pub struct AnalyticQuery {
    pub id: ID,
    pub name: String,
    pub description: String,
}

#[derive(InputObject)]
pub struct AnalyticQueryRequest {
    pub id: ID,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticDataAxisPosition {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticDataAxisType {
    X,
    Y,
}

#[derive(SimpleObject)]
pub struct AnalyticDataAxisLabel {
    pub label: String,
    #[graphql(name = "type")]
    pub axis_type: AnalyticDataAxisType,
    pub color: String,
    pub margin: Option<i32>,
    pub domain_line: bool,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticDataAxisTickTextAlign {
    Left,
    Center,
    Right,
}

#[derive(SimpleObject)]
pub struct AnalyticDataAxisTick {
    pub enabled: bool,
    pub label_color: Option<String>,
    pub text_align: Option<AnalyticDataAxisTickTextAlign>,
    pub text_angle: Option<f64>,
    pub text_width: Option<f64>,
    pub number: Option<i32>,
    pub format: Option<String>,
}

#[derive(SimpleObject)]
pub struct AnalyticDataAxis {
    #[graphql(name = "type")]
    pub axis_type: AnalyticDataAxisType,
    pub label: AnalyticDataAxisLabel,
    pub tick: AnalyticDataAxisTick,
}

#[derive(Union, Clone)]
pub enum AnalyticDataRecord {
    Single(AnalyticSingleDataRecord),
    Multi(AnalyticMultiDataRecord),
}

#[derive(SimpleObject, Clone)]
pub struct AnalyticSingleDataRecord {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub attributes: Option<JsonValue>,
}

#[derive(SimpleObject, Clone)]
pub struct AnalyticMultiDataRecord {
    pub x: Option<f64>,
    pub y: Option<Vec<f64>>,
    pub attributes: Option<JsonValue>,
}

#[derive(SimpleObject)]
pub struct AnalyticDataAnnotationContent {
    pub text: String,
    pub font_size: Option<i32>,
}

#[derive(SimpleObject)]
pub struct AnalyticDataAnnotation {
    pub content: AnalyticDataAnnotationContent,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticDataContainerType {
    Single,
    Xy,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticDataContainerScale {
    Time,
}

#[derive(SimpleObject)]
pub struct AnalyticDataContainerGraph {
    #[graphql(name = "type")]
    pub graph_type: AnalyticQueryGraphType,
}

#[derive(SimpleObject)]
pub struct AnalyticDataContainer {
    #[graphql(name = "type")]
    pub container_type: AnalyticDataContainerType,
    pub scale: Option<AnalyticDataContainerScale>,
    pub graphs: Vec<AnalyticDataContainerGraph>,
    pub axis: Vec<AnalyticDataAxis>,
    pub data: Vec<AnalyticDataRecord>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticDataLegendType {
    Bullet,
}

#[derive(SimpleObject)]
pub struct AnalyticDataLegend {
    #[graphql(name = "type")]
    pub legend_type: Option<AnalyticDataLegendType>,
    pub items: Vec<AnalyticDataLegendItem>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticDataLegendShape {
    Circle,
    Square,
    Triangle,
    Star,
}

#[derive(SimpleObject)]
pub struct AnalyticDataLegendItem {
    pub name: Option<String>,
    pub color: Option<String>,
    pub shape: Option<AnalyticDataLegendShape>,
    pub inactive: Option<bool>,
    pub hidden: Option<bool>,
    pub pointer: Option<bool>,
}

#[derive(SimpleObject)]
pub struct AnalyticQueryResponse {
    pub legend: Option<AnalyticDataLegend>,
    pub container: AnalyticDataContainer,
    pub annotations: Vec<AnalyticDataAnnotation>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticEventType {
    Session,
    Interaction,
    Impression,
    Completion,
}

#[derive(SimpleObject)]
pub struct AnalyticContent {
    pub id: String,
    #[graphql(name = "type")]
    pub content_type: String,
    pub index: Option<i32>,
    pub percent: Option<f64>,
}

#[derive(SimpleObject)]
pub struct AnalyticElement {
    pub id: String,
    #[graphql(name = "type")]
    pub element_type: String,
    pub content: Option<Vec<AnalyticContent>>,
    pub extras: Option<JsonValue>,
}

#[derive(SimpleObject)]
pub struct AnalyticContext {
    pub app_id: Option<String>,
    pub app_version: Option<String>,
    pub browser: Option<AnalyticBrowser>,
    pub client_id: Option<String>,
    pub session_id: Option<String>,
    pub device: Option<AnalyticDevice>,
    pub geo: Option<AnalyticGeo>,
    pub user_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AnalyticDevice {
    pub installation_id: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub platform: Option<String>,
    pub primary_locale: Option<String>,
    pub system_name: Option<String>,
    pub timezone: Option<String>,
    pub device_type: Option<String>,
    pub version: Option<String>,
}

#[derive(SimpleObject)]
pub struct AnalyticBrowser {
    pub agent: Option<String>,
}

#[derive(SimpleObject)]
pub struct AnalyticGeo {
    pub city: Option<String>,
    pub country: Option<String>,
    pub continent: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub region: Option<String>,
    pub region_code: Option<String>,
    pub postal_code: Option<String>,
    pub timezone: Option<String>,
}

#[derive(SimpleObject)]
pub struct AnalyticEvent {
    #[graphql(name = "type")]
    pub event_type: AnalyticEventType,
    pub name: String,
    pub client_id: String,
    pub created: DateTime<Utc>,
    pub element: Option<AnalyticElement>,
    pub context: Option<AnalyticContext>,
    pub sent: Option<DateTime<Utc>>,
    pub received: Option<DateTime<Utc>>,
}

#[derive(InputObject, Clone)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(SimpleObject)]
pub struct DailyActiveUserRecord {
    pub date: DateTime<Utc>,
    pub count: i32,
}

#[derive(SimpleObject)]
pub struct WeeklyActiveUserRecord {
    pub week: DateTime<Utc>,
    pub count: i32,
}

#[derive(SimpleObject)]
pub struct NewUserRecord {
    pub date: DateTime<Utc>,
    pub count: i32,
}

#[derive(SimpleObject)]
pub struct ReturningUserRecord {
    pub date: DateTime<Utc>,
    pub percentage: f64,
}

#[derive(SimpleObject)]
pub struct RetentionCohort {
    pub cohort_date: DateTime<Utc>,
    pub users_count: i32,
    pub retention_periods: Vec<f64>,
}

#[derive(SimpleObject)]
pub struct RetentionFlowResponse {
    pub cohorts: Vec<RetentionCohort>,
}

#[derive(SimpleObject)]
pub struct SessionTotalRecord {
    pub date: DateTime<Utc>,
    pub session_count: i32,
}

#[derive(SimpleObject)]
pub struct SessionDurationRecord {
    pub date: DateTime<Utc>,
    pub average_duration: f64,
}

#[derive(SimpleObject)]
pub struct SessionDeviceRecord {
    pub device_type: String,
    pub session_count: i32,
    pub average_duration: f64,
}

#[derive(SimpleObject)]
pub struct SessionMetrics {
    pub total_sessions: i32,
    pub average_duration: f64,
    pub device_breakdown: Vec<SessionDeviceRecord>,
}

#[derive(SimpleObject)]
pub struct ContentViewRecord {
    pub content_id: String,
    pub date: DateTime<Utc>,
    pub view_count: i32,
}

#[derive(SimpleObject)]
pub struct TopContentRecord {
    pub content_id: String,
    pub title: String,
    pub view_count: i32,
    pub engagement_score: f64,
}

#[derive(SimpleObject)]
pub struct EngagementRecord {
    pub content_id: String,
    pub views: i32,
    pub likes: i32,
    pub shares: i32,
    pub comments: i32,
    pub time_spent: f64,
}

#[derive(SimpleObject)]
pub struct ContentMetrics {
    pub total_views: i32,
    pub unique_viewers: i32,
    pub average_engagement_time: f64,
    pub top_content: Vec<TopContentRecord>,
}