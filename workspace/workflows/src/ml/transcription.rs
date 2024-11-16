use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Segment {
    pub avg_logprob: f64,
    pub compression_ratio: f64,
    pub end: f64,
    pub id: i64,
    pub no_speech_prob: f64,
    pub seek: i64,
    pub start: f64,
    pub temperature: i64,
    pub text: String,
    pub tokens: Vec<i64>,
    pub words: Vec<Word>,
}

#[derive(Serialize, Deserialize)]
pub struct Word {
    pub start: f64,
    pub end: f64,
    pub probability: f64,
    pub word: String,
}

#[derive(Serialize, Deserialize)]
pub struct Transcription {
    pub language: String,
    pub segments: Vec<Segment>,
    pub text: String,
}