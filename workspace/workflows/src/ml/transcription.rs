use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Segment {
    pub avg_logprob: f64,
    pub compression_ratio: f64,
    pub end: f64,
    pub id: i64,
    pub no_speech_prob: f64,
    pub seek: f64,
    pub start: f64,
    pub temperature: f64,
    pub text: String,
    pub tokens: Vec<i64>,
    pub words: Vec<Word>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Word {
    pub start: f64,
    pub end: f64,
    pub probability: f64,
    pub word: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transcription {
    pub language: String,
    pub segments: Vec<Segment>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TranscriptionResult {
    pub transcription: Transcription
}

impl Transcription {
    pub fn get_segment(&self, str: &str) -> Option<Segment> {
        let mut new_segment = Segment {
            avg_logprob: 0.0,
            start: f64::NEG_INFINITY,
            temperature: 0.0,
            text: str.trim_ascii().to_owned(),
            tokens: vec![],
            end: f64::NEG_INFINITY,
            id: 0,
            no_speech_prob: 0.0,
            compression_ratio: 0.0,
            seek: 0.0,
            words: vec![],
        };

        let mut search = str.trim_ascii().to_lowercase();

        for segment in &self.segments {
            for word in &segment.words {
                let w = word.word.trim_ascii().to_lowercase();
                if search.starts_with(&w) {
                    new_segment.words.push(word.clone());
                    new_segment.start = if new_segment.start.is_sign_negative() || word.start < new_segment.start {
                        word.start
                    } else {
                        new_segment.start
                    };
                    new_segment.end = if new_segment.end.is_sign_negative() || word.end > new_segment.end {
                        word.end
                    } else {
                        new_segment.end
                    };
                    search = search[w.len()..].trim_ascii().parse().unwrap();
                    if search.is_empty() {
                        break;
                    }
                } else if new_segment.start.is_nan() {
                    search = str.trim_ascii().parse().unwrap();
                    new_segment.start = f64::NEG_INFINITY;
                    new_segment.end = f64::NEG_INFINITY;
                    new_segment.words.clear();
                }
            }
            if search.is_empty() {
                break;
            }
        }
        if !search.is_empty() {
            None
        } else {
            Some(new_segment)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ml::transcription::{Segment, Transcription, Word};

    #[test]
    pub fn test() {
        let transcript = Transcription {
            language: "en".to_owned(),
            text: "Hello World, how are you? I think you're awesome!".to_owned(),
            segments: vec![
                Segment {
                    text: "Hello World, ".to_owned(),
                    tokens: vec![],
                    start: 0.0,
                    end: 3.0,
                    id: 0,
                    no_speech_prob: 0.0,
                    avg_logprob: 0.0,
                    compression_ratio: 0.0,
                    seek: 0.0,
                    temperature: 0.0,
                    words: vec![
                        Word {
                            start: 0.0,
                            end: 1.0,
                            probability: 0.0,
                            word: " Hello ".to_string(),
                        },
                        Word {
                            start: 2.0,
                            end: 3.0,
                            probability: 0.0,
                            word: " World, ".to_string(),
                        }
                    ],
                },
                Segment {
                    text: "how are you?".to_owned(),
                    tokens: vec![],
                    start: 10.0,
                    end: 17.0,
                    id: 0,
                    no_speech_prob: 0.0,
                    avg_logprob: 0.0,
                    compression_ratio: 0.0,
                    seek: 0.0,
                    temperature: 0.0,
                    words: vec![
                        Word {
                            start: 10.0,
                            end: 11.0,
                            probability: 0.0,
                            word: " how ".to_string(),
                        },
                        Word {
                            start: 12.0,
                            end: 13.0,
                            probability: 0.0,
                            word: " are ".to_string(),
                        },
                        Word {
                            start: 14.0,
                            end: 15.0,
                            probability: 0.0,
                            word: " you? ".to_string(),
                        }
                    ],
                },
                Segment {
                    text: " I think you're awesome! ".to_owned(),
                    tokens: vec![],
                    start: 17.0,
                    end: 28.0,
                    id: 0,
                    no_speech_prob: 0.0,
                    avg_logprob: 0.0,
                    compression_ratio: 0.0,
                    seek: 0.0,
                    temperature: 0.0,
                    words: vec![
                        Word {
                            start: 17.0,
                            end: 18.0,
                            probability: 0.0,
                            word: " I ".to_string(),
                        },
                        Word {
                            start: 18.0,
                            end: 19.0,
                            probability: 0.0,
                            word: " think ".to_string(),
                        },
                        Word {
                            start: 22.0,
                            end: 23.0,
                            probability: 0.0,
                            word: " you're ".to_string(),
                        },
                        Word {
                            start: 24.0,
                            end: 26.0,
                            probability: 0.0,
                            word: " awesome! ".to_string(),
                        }
                    ],
                }
            ],
        };

        let timing = transcript.get_segment("world, how are you?").unwrap();

        assert_eq!(timing.start, 2.0);
        assert_eq!(timing.end, 15.0);
    }
}