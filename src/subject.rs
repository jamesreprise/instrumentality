//! Subjects for organisation of profiles.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subject {
    pub uuid: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub name: String,
    pub profiles: HashMap<String, Vec<String>>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subjects {
    pub data: Vec<Subject>,
}
