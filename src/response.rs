//! Error responses.
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Error {
    pub response: String,
    pub text: String,
}

impl Error {
    pub fn new(text: &str) -> Self {
        Self {
            response: "ERROR".to_string(),
            text: text.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Ok {
    response: String,
}

impl Ok {
    pub fn new() -> Self {
        Self {
            response: "OK".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct InviteResponse {
    response: String,
    key: String,
}

impl InviteResponse {
    pub fn new(key: String) -> Self {
        Self {
            response: "OK".to_string(),
            key,
        }
    }
}

#[derive(Serialize)]
pub struct QueueResponse {
    response: String,
    queue_id: String,
    username: String,
    platform: String,
}

impl QueueResponse {
    pub fn new(queue_id: String, username: String, platform: String) -> Self {
        Self {
            response: "OK".to_string(),
            queue_id,
            username,
            platform,
        }
    }
}

#[derive(Serialize)]
pub struct GroupResponse {
    response: String,
    group: crate::group::Group,
}

impl GroupResponse {
    pub fn new(group: crate::group::Group) -> Self {
        Self {
            response: "OK".to_string(),
            group,
        }
    }
}

#[derive(Serialize)]
pub struct RegisterResponse {
    response: String,
    user: crate::user::User,
}

impl RegisterResponse {
    pub fn new(user: crate::user::User) -> Self {
        Self {
            response: "OK".to_string(),
            user,
        }
    }
}

#[derive(Serialize)]
pub struct ViewResponse {
    response: String,
    view_data: crate::routes::view::ViewData,
}

impl ViewResponse {
    pub fn new(view_data: crate::routes::view::ViewData) -> Self {
        Self {
            response: "OK".to_string(),
            view_data,
        }
    }
}

#[derive(Serialize)]
pub struct SubjectResponse {
    response: String,
    subject: crate::subject::Subject,
}

impl SubjectResponse {
    pub fn new(subject: crate::subject::Subject) -> Self {
        Self {
            response: "OK".to_string(),
            subject,
        }
    }
}

#[derive(Serialize)]
pub struct TypesResponse {
    response: String,
    content_types: std::collections::HashMap<String, Vec<String>>,
    presence_types: std::collections::HashMap<String, Vec<String>>,
}

impl TypesResponse {
    pub fn new(
        content_types: std::collections::HashMap<String, Vec<String>>,
        presence_types: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        Self {
            response: "OK".to_string(),
            content_types,
            presence_types,
        }
    }
}
