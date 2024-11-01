use argon2::Error;
use crate::events::Events;

pub trait EventSink {

    async fn add(events: Events) -> Result<(), Error>;
}