//! Data enums for content, presence and profile metadata.
//!
//! # A note on Data
//!
//! The quality of all data provided to Instrumentality will determine it's usefulness.
//! For example, if one data provider posts full profile metadata and another posts the
//! minimum subset in quick succession, Instrumentality must discard the full profile
//! in favour of the minimum subset as it has no ability to determine that all the
//! previous data was not removed by the user between posts. This also applies to
//! content.
//!
//! In the future, profiles with lots of coverage could be used to sniff out lazy data
//! providers automatically or through a reputation system. However, at this stage,
//! data providers posting all available data is key to the utility of the platform.
//!
//! # Content
//! Content exists to represent any event occurring at a discrete point in time.
//!
//! Examples of content include:
//! - a blog entry.
//! - a video.
//! - an item coming back into stock on an online store.
//!
//! The only requirements of content are that it must have a subject, a content type
//! and a time retrieved. For example,
//! ```json
//! {
//!     "id": "123456789",
//!     "platform": "twitter",
//!     "content_type": "tweet",
//!     "created_at": "2038-01-19T03:14:07Z",
//!     "body": "I love my epoch.",
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
//! change, you must fill the id field with a unique user ID. Instrumentality will not
//! stop you submitting content with a username as the subject but this is suboptimal.
//!
//! ## Content types
//! Platforms cannot said to be solely made up of one type of content. For example,
//! 'stories' are a common temporary post feature that exist on top a platforms 'bread
//! and butter' content. In order to differentiate between content types on the same
//! platform we tag them with a type. For example,
//! ```json
//! {
//!     "id": "123456789",
//!     "platform": "instagram",
//!     "content_type": "post",
//!     "created_at": "2022-01-01T00:00:05Z",
//!     "body": "Happy new year!",
//!     "media": ["https://..."]
//! };
//! ```
//! and
//! ```json
//! {
//!     "id": "123456789",
//!     "platform": "instagram",
//!     "content_type": "story",
//!     "created_at": "2022-01-01T00:00:05Z",
//!     "body": "Happy new year!",
//!     "media": ["https://..."]
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
//! ```json
//! {
//!     "id": "123456789",
//!     "platform": "twitch",
//!     "presence_type": "livestream",
//!     "retrieved_at": "2022-01-01T00:00:00Z",
//! };
//! ```
//!
//! # Meta
//! Profile metadata changes regularly and sometimes silently. Without data providers
//! keeping a local copy of the data, it's difficult to determine what has changed from fetch
//! to fetch. Given that each request of the profile will generally contain a full copy of
//! that profile, it's easier to post the entire profile to Instrumentality to determine
//! changes.

use crate::database::DBHandle;
use crate::routes::queue;
use crate::routes::queue::InternalQueueItem;

use chrono::{DateTime, Utc};
use mongodb::{bson::doc, Collection};
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
        retrieved_at: DateTime<Utc>,
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
    pub fn verify(
        &self,
        content_types: &HashMap<String, Vec<String>>,
        presence_types: &HashMap<String, Vec<String>>,
    ) -> bool {
        match self {
            Self::Presence {
                platform,
                presence_type,
                ..
            } => presence_types
                .get(platform)
                .unwrap()
                .contains(presence_type),
            Self::Content {
                platform,
                content_type,
                ..
            } => content_types.get(platform).unwrap().contains(content_type),
            Self::Meta { platform, .. } => {
                presence_types.contains_key(platform) || content_types.contains_key(platform)
            }
        }
    }

    // I'm sure this can be cleaned up but I don't know how.
    // This is the debt to be paid for using an enum.
    pub fn tag(self, uuid: String) -> Self {
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
                retrieved_at,
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
                retrieved_at,
                added_by: Some(uuid),
                added_at: Some(Utc::now()),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Datas {
    pub data: Vec<Data>,
    pub queue_id: Option<String>,
}

impl Datas {
    pub fn verify(
        self,
        content_types: &HashMap<String, Vec<String>>,
        presence_types: &HashMap<String, Vec<String>>,
    ) -> Self {
        let mut verified_data = Vec::new();
        for d in self.data {
            if d.verify(content_types, presence_types) {
                verified_data.push(d);
            }
        }
        Self {
            data: verified_data,
            queue_id: self.queue_id,
        }
    }

    pub fn tag(self, uuid: String) -> Self {
        let mut tagged_data = Vec::new();
        for d in self.data {
            tagged_data.push(d.tag(uuid.clone()))
        }
        Self {
            data: tagged_data,
            queue_id: self.queue_id,
        }
    }

    pub fn get_meta(datas: &Vec<Data>) -> Option<&Data> {
        for d in datas {
            if let Data::Meta { .. } = d {
                return Some(d);
            }
        }
        None
    }

    pub fn get_info(datas: &Vec<Data>) -> (&String, &String, &Option<String>, Option<&String>) {
        let meta = Datas::get_meta(datas);
        if let Some(meta) = meta {
            let (platform_id, platform, added_by, username) = match meta {
                Data::Meta {
                    id,
                    platform,
                    added_by,
                    username,
                    ..
                } => (id, platform, added_by, Some(username)),
                _ => panic!("Expected Data::Meta."),
            };
            (platform_id, platform, added_by, username)
        } else {
            let data = &datas[0];
            let (platform_id, platform, added_by) = match data {
                Data::Presence {
                    id,
                    platform,
                    added_by,
                    ..
                } => (id, platform, added_by),
                Data::Content {
                    id,
                    platform,
                    added_by,
                    ..
                } => (id, platform, added_by),
                _ => panic!("Expected Presence or Content."),
            };
            (platform_id, platform, added_by, None)
        }
    }

    // The logic for this function needs to be simplified significantly.
    // There are several sources of uncertainty that this function resolves:
    // - Is there data to be processed and is there an attached queue_id?
    // - Does the given queue_id reference an actual job?
    // - Does the queue item have a username attached or a platform id?
    // - Does all the data in self.data pertain to the queue job? If not filter it out.
    // Then get relevant data and pass it to the queue for processing.
    pub async fn process_queue(self, db: &DBHandle) -> Self {
        if !self.data.is_empty() && self.queue_id.is_some() {
            let mut verified_data = Vec::new();
            let queue_id = self.queue_id.clone().unwrap();
            let q_coll: Collection<InternalQueueItem> = db.collection("queue");
            let q_item = q_coll
                .find_one(doc! {"queue_id": &queue_id }, None)
                .await
                .unwrap();
            if q_item.is_some() {
                let q_item = q_item.unwrap();
                // We can't guarantee the queue item has the correct platform id,
                // as it might be a new queue item. So we grab it early from any
                // Data::Meta in the array.
                let meta = Datas::get_meta(&self.data);
                let mut platform_id: Option<String> = None;
                if let Some(meta) = meta {
                    platform_id = match meta {
                        Data::Meta { id, .. } => Some(id.to_string()),
                        _ => panic!("Expected Data::Meta."),
                    };
                }

                // We verify that all data in the array is pertinent to this job.
                for d in &self.data {
                    let verified: bool = match &d {
                        Data::Meta { platform, id, .. } => {
                            if let Some(platform_id) = platform_id.clone() {
                                &platform_id == id && &q_item.platform == platform
                            } else {
                                &q_item.platform == platform
                            }
                        }
                        Data::Content { platform, id, .. } => {
                            if let Some(platform_id) = platform_id.clone() {
                                &platform_id == id && &q_item.platform == platform
                            } else {
                                &q_item.platform == platform
                            }
                        }
                        Data::Presence { platform, id, .. } => {
                            if let Some(platform_id) = platform_id.clone() {
                                &platform_id == id && &q_item.platform == platform
                            } else {
                                &q_item.platform == platform
                            }
                        }
                    };
                    if verified {
                        verified_data.push(d.clone());
                    }
                }

                if !verified_data.is_empty() {
                    let (platform_id, platform, added_by, username) =
                        Datas::get_info(&verified_data);

                    queue::process(&queue_id, platform_id, platform, added_by, username, db).await;
                }

                return Self {
                    data: verified_data,
                    queue_id: self.queue_id,
                };
            }
        }
        self
    }
}
