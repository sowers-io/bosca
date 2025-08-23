use crate::graphql::analytics::types::*;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

pub struct AnalyticEventData {
    pub event_type: AnalyticEventType,
    pub name: String,
    pub client_id: String,
    pub created: DateTime<Utc>,
    pub element: Option<AnalyticElementData>,
    pub context: Option<AnalyticContextData>,
    pub sent: Option<DateTime<Utc>>,
    pub received: Option<DateTime<Utc>>,
}

pub struct AnalyticElementData {
    pub id: String,
    pub element_type: String,
    pub content: Option<Vec<AnalyticContentData>>,
    pub extras: Option<JsonValue>,
}

pub struct AnalyticContentData {
    pub id: String,
    pub content_type: String,
    pub index: Option<i32>,
    pub percent: Option<f64>,
}

pub struct AnalyticContextData {
    pub app_id: Option<String>,
    pub app_version: Option<String>,
    pub browser: Option<AnalyticBrowserData>,
    pub client_id: Option<String>,
    pub session_id: Option<String>,
    pub device: Option<AnalyticDeviceData>,
    pub geo: Option<AnalyticGeoData>,
    pub user_id: Option<String>,
}

pub struct AnalyticDeviceData {
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

pub struct AnalyticBrowserData {
    pub agent: Option<String>,
}

pub struct AnalyticGeoData {
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

impl From<AnalyticEventData> for AnalyticEvent {
    fn from(data: AnalyticEventData) -> Self {
        AnalyticEvent {
            event_type: data.event_type,
            name: data.name,
            client_id: data.client_id,
            created: data.created,
            element: data.element.map(Into::into),
            context: data.context.map(Into::into),
            sent: data.sent,
            received: data.received,
        }
    }
}

impl From<AnalyticElementData> for AnalyticElement {
    fn from(data: AnalyticElementData) -> Self {
        AnalyticElement {
            id: data.id,
            element_type: data.element_type,
            content: data.content.map(|c| c.into_iter().map(Into::into).collect()),
            extras: data.extras,
        }
    }
}

impl From<AnalyticContentData> for AnalyticContent {
    fn from(data: AnalyticContentData) -> Self {
        AnalyticContent {
            id: data.id,
            content_type: data.content_type,
            index: data.index,
            percent: data.percent,
        }
    }
}

impl From<AnalyticContextData> for AnalyticContext {
    fn from(data: AnalyticContextData) -> Self {
        AnalyticContext {
            app_id: data.app_id,
            app_version: data.app_version,
            browser: data.browser.map(Into::into),
            client_id: data.client_id,
            session_id: data.session_id,
            device: data.device.map(Into::into),
            geo: data.geo.map(Into::into),
            user_id: data.user_id,
        }
    }
}

impl From<AnalyticDeviceData> for AnalyticDevice {
    fn from(data: AnalyticDeviceData) -> Self {
        AnalyticDevice {
            installation_id: data.installation_id,
            manufacturer: data.manufacturer,
            model: data.model,
            platform: data.platform,
            primary_locale: data.primary_locale,
            system_name: data.system_name,
            timezone: data.timezone,
            device_type: data.device_type,
            version: data.version,
        }
    }
}

impl From<AnalyticBrowserData> for AnalyticBrowser {
    fn from(data: AnalyticBrowserData) -> Self {
        AnalyticBrowser {
            agent: data.agent,
        }
    }
}

impl From<AnalyticGeoData> for AnalyticGeo {
    fn from(data: AnalyticGeoData) -> Self {
        AnalyticGeo {
            city: data.city,
            country: data.country,
            continent: data.continent,
            longitude: data.longitude,
            latitude: data.latitude,
            region: data.region,
            region_code: data.region_code,
            postal_code: data.postal_code,
            timezone: data.timezone,
        }
    }
}