//! Basic user concepts for Instrumentality.

use serde::{Deserialize, Serialize};
use std::fmt::Write;
use uuid::Uuid;

#[derive(Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub key: String,
    pub banned: bool,
}

impl User {
    pub fn new(name: &str) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name: name.to_string(),
            key: Self::new_key(),
            banned: false,
        }
    }

    pub fn new_key() -> String {
        let key_bytes: &mut [u8] = &mut [0; 32];
        getrandom::getrandom(key_bytes).unwrap();
        let mut key = String::new();
        for b in key_bytes {
            write!(&mut key, "{:0>2X}", b).unwrap();
        }
        key
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new_user() {
        let user = User::new("test");

        assert!(!user.banned);
        assert_eq!(user.name, "test");
    }

    #[test]
    fn test_key() {
        let user = User::new("test");
        let re = regex::Regex::new(r"^([A-F0-9])*$").unwrap();

        assert_eq!(user.key.len(), 64);
        assert!(re.is_match(&user.key));
    }
}
