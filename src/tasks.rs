use chrono::{serde::ts_seconds,DateTime, Local, UTC};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<UTC>,
    
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<UTC> = Utc::now();
        Task { text, created_at }
    }
}