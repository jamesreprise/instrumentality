//! Data enums for content, presence and profile metadata.
//!
//! # Content
//! Content exists to represent any event occurring at a certain point in time.
//!
//! Examples of content include:
//! - a blog entry.
//! - a video.
//! - an item coming back into stock on an online store.
//!
//! The only requirements of content are that it must have a subject, a content type
//! and a time retrieved. For example,
//! ```rust
//! let tweet: Content = {
//!     id: "123456789"
//!     platform: "twitter",
//!     content_type: "tweet",
//!     retrieved_at: "2038-01-19T03:14:07Z", // All times must be in UTC+00:00/UTC±0: Z
//!     body: "I love my epoch.",
//! };
//! ```
//! When handling URLs, we store the original URL of the content and have a separate media
//! table in MongoDB to retrieve the content at the point at which we want to reconstruct
//! the post. This is true for image, audio, video and any further content that cannot be
//! reasonably represented in UTF-8.
//!
//! The URLs in media should be direct links to the files themselves, not a page
//! with the media present on it. If need be, this may involve the extractor manually
//! grabbing the media and hosting it until Instrumentality has confirmed it as received.
//!
//! ## IDs
//! In order to continue attributing new content to the correct user after a username
//! change, you must fill the id field with a unique ID. Instrumentality will not
//! stop you submitting content with a username as the subject but this is suboptimal.
//!
//! ## Content types
//! Platforms cannot said to be solely made up of one type of content. For example,
//! 'stories' are a common temporary post feature that exist on top a platforms 'bread
//! and butter' content. In order to differentiate between content types on the same
//! platform we tag them with a type. For example,
//! ```rust
//! let ig_post: Content = {
//!     id: "123456789",
//!     platform: "instagram",
//!     content_type: "post",
//!     retrieved_at: "2022-01-01T00:00:05Z", // All times must be in UTC±00:00/UTC Z
//!     body: "Happy new year!",
//!     media: ["https://..."]
//! };
//! ```
//! and
//! ```rust
//! let ig_story: Content = {
//!     id: "123456789",
//!     platform: "instagram",
//!     content_type: "story",
//!     retrieved_at: "2022-01-01T00:00:05Z",
//!     body: "Happy new year!",
//!     media: ["https://..."]
//! };
//! ```
//! are distinct types of content that are still tied to the id '123456789' on 'instagram'.
//! Content can only be of types specified within Instrumentality.
//!
//! ## Activity
//! Updates to the user are always tagged as 'activity'. These are distinct
//! from other types of content in that they are not content in and of themselves but
//! do represent some action that the user has taken.
//!
//! ## Limitations
//! The Content struct is not intended to perfectly mirror all types of content on every
//! platform, it is merely a first approximation. Certain information is inevitably lost
//! during the process of mirroring content, such as the positions of tags on group
//! photos.
//!
//! # Presence
//! Presence exists to represent a user being 'active' or present in a continuous manner.
//! Obviously, these are discrete observations of continuous behaviour but labeling them
//! accordingly makes this apparent to the system.
//!
//! One must be wary of attempting to interpret discrete observations to continuous data.
//!
//! An example of this is a Twitch livestream being live. This isn't content because it can't
//! be said to have 'happened' at a discrete point in time until it is finished, at which point
//! you would post it as content. A Twitch livestream going live could be considered to be
//! content as it happens at a discrete time.
//!
//! These are expected to make up the bulk of traffic as presence changes occur far more often
//! than content posts.
//!
//! For example,
//! ```rust
//! let twitch_live: Presence = {
//!     id: "123456789",
//!     platform: "twitch",
//!     presence_type: "livestream",
//!     retrieved_at: "2022-01-01T00:00:05Z",
//! };
//! ```

use crate::config::IConfig;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Data {
    Presence {
        id: String,
        platform: String,
        presence_type: String,
        retrieved_at: DateTime<Utc>,
        added_by: Option<String>,
        added_at: Option<DateTime<Utc>>,
    },
    Content {
        id: String,
        platform: String,
        content_type: String,
        retrieved_at: DateTime<Utc>,
        content_id: String,
        deleted: Option<bool>,
        retrieved_from: Option<String>,
        created_at: Option<DateTime<Utc>>,
        body: Option<String>,
        media: Option<Vec<String>>,
        references: Option<HashMap<String, String>>,
        added_by: Option<String>,
        added_at: Option<DateTime<Utc>>,
    },
    Meta {
        id: String,
        platform: String,
        username: String,
        private: bool,
        suspended_or_banned: bool,
        display_name: Option<String>,
        profile_picture: Option<String>,
        bio: Option<String>,
        verified: Option<bool>,
        references: Option<HashMap<String, String>>,
        link: Option<String>,
        added_by: Option<String>,
        added_at: Option<DateTime<Utc>>,
    },
}

impl Data {
    pub fn verify(self: &Self, config: &IConfig) -> bool {
        match self {
            Self::Presence {
                platform,
                presence_type,
                ..
            } => config
                .presence_types
                .get(platform)
                .unwrap()
                .contains(presence_type),
            Self::Content {
                platform,
                content_type,
                ..
            } => config
                .content_types
                .get(platform)
                .unwrap()
                .contains(content_type),
            _ => true,
        }
    }

    pub fn tag(self: Self, uuid: String) -> Self {
        match self {
            Self::Presence {
                id,
                platform,
                presence_type,
                retrieved_at,
                ..
            } => Self::Presence {
                id,
                platform,
                presence_type,
                retrieved_at,
                added_by: Some(uuid),
                added_at: Some(Utc::now()),
            },
            Self::Content {
                id,
                platform,
                content_type,
                retrieved_at,
                content_id,
                deleted,
                retrieved_from,
                created_at,
                body,
                media,
                references,
                ..
            } => Self::Content {
                id,
                platform,
                content_type,
                retrieved_at,
                content_id,
                deleted,
                retrieved_from,
                created_at,
                body,
                media,
                references,
                added_by: Some(uuid),
                added_at: Some(Utc::now()),
            },
            Self::Meta {
                id,
                platform,
                username,
                private,
                suspended_or_banned,
                display_name,
                profile_picture,
                bio,
                verified,
                references,
                link,
                ..
            } => Self::Meta {
                id,
                platform,
                username,
                private,
                suspended_or_banned,
                display_name,
                profile_picture,
                bio,
                verified,
                references,
                link,
                added_by: Some(uuid),
                added_at: Some(Utc::now()),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Datas {
    pub data: Vec<Data>,
}

impl Datas {
    pub fn verify(self: Self, config: &IConfig) -> Self {
        let mut verified_datas = Vec::new();
        for data in self.data {
            if data.verify(config) {
                verified_datas.push(data);
            }
        }
        Self {
            data: verified_datas,
        }
    }

    pub fn tag(self: Self, uuid: String) -> Self {
        let mut tagged_data = Vec::new();
        for d in self.data {
            tagged_data.push(d.tag(uuid.clone()))
        }
        Self { data: tagged_data }
    }
}
