use serde::{Deserialize, Serialize};
use crate::ml::whisper::decoding_result::DecodingResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Segment {
    pub start: f64,
    pub duration: f64,
    pub dr: DecodingResult,
    pub timing: Vec<SegmentTiming>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentTiming {
    pub start: f32,
    pub duration: f32,
    pub text: String
}