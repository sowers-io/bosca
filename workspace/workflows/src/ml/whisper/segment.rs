use serde::{Deserialize, Serialize};
use crate::ml::whisper::decoding_result::DecodingResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Segment {
    pub start_offset: f64,
    pub start: f64,
    pub duration: f64,
    pub duration_offset: f64,
    pub dr: DecodingResult,
}