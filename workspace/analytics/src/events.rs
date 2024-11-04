use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub enum EventType {
    Interaction,
    Impression,
    Completion
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Content {
    pub id: String,
    #[serde(alias="type")]
    pub content_type: String,
    pub index: Option<i32>,
    pub percent: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Element {
    pub id: String,
    #[serde(alias="type")]
    pub element_type: String,
    pub content: Vec<Content>,
    pub extras: Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
    pub created: i64,
    pub created_micros: Option<u32>,
    #[serde(alias="type")]
    pub event_type: EventType,
    pub element: Element,
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Geo {
    pub city: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Device {
    pub installation_id: String,
    pub manufacturer: String,
    pub model: String,
    pub platform: String,
    #[serde(alias="primaryLocale")]
    pub primary_locale: String,
    #[serde(alias="systemName")]
    pub system_name: String,
    pub timezone: String,
    #[serde(alias="type")]
    pub device_type: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Browser {
    pub agent: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EventContext {
    pub app_id: String,
    pub app_version: String,
    pub browser: Option<Browser>,
    pub client_id: String,
    pub device: Device,
    pub geo: Geo,
    pub session_id: String,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Events {
    pub context: EventContext,
    pub events: Vec<Event>,
    pub sent: i64,
    pub sent_micros: u32,
    pub received: Option<i64>,
    pub received_micros: Option<u32>,
}