use std::error::Error;
use std::str::FromStr;
use crate::events::Events;
use crate::events_sink::EventPipelineContext;
use crate::events_transform::EventTransform;

pub struct CloudflareGeoTransform {

}

#[async_trait::async_trait]
impl EventTransform for CloudflareGeoTransform {
    async fn transform(&self, context: &mut EventPipelineContext, event: &mut Events) -> Result<(), Box<dyn Error>> {
        event.context.geo.city = context.get_header_value("cf-ipcity").map(|s| s.to_owned());
        event.context.geo.country = context.get_header_value("cf-ipcountry").map(|s| s.to_owned());
        event.context.geo.continent = context.get_header_value("cf-ipcontinent").map(|s| s.to_owned());
        event.context.geo.region = context.get_header_value("cf-region").map(|s| s.to_owned());
        event.context.geo.region_code = context.get_header_value("cf-region-code").map(|s| s.to_owned());
        event.context.geo.postal_code = context.get_header_value("cf-postal-code").map(|s| s.to_owned());
        event.context.geo.timezone = context.get_header_value("cf-timezone").map(|s| s.to_owned());
        event.context.geo.longitude = context.get_header_value("cf-iplongitude").map(|s| f64::from_str(s).unwrap_or(0f64));
        event.context.geo.latitude = context.get_header_value("cf-iplatitude").map(|s| f64::from_str(s).unwrap_or(0f64));
        Ok(())
    }
}