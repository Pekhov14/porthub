use serde::{Deserialize, Serialize};
use crate::models::tag::Tag;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookmark {
    pub id: String, // ULID
    pub title: String,
    pub url: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub icon: String,
    pub sort: i32,
    #[serde(default)]
    pub is_favorite: bool,
    pub tags: Vec<Tag>,
}
