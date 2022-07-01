//! Groups for organisitions of subjects.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub uuid: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String, // UUID
    pub name: String,
    pub subjects: Vec<String>,
    pub description: Option<String>,
}
