use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    id: String,
    platform: String,
    username: String, // No platform specific conventions i.e. "x" not "@x"
    private: bool,
    suspended_or_banned: bool,
    first_linked_at: DateTime<Utc>,
    display_name: Option<String>,
    profile_picture: Option<String>,
    bio: Option<String>,
    verified: Option<bool>,
    references: Option<HashMap<String, String>>,
    link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profiles {
    pub data: Vec<Profile>,
}
