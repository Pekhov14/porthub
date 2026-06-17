#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Link {
    pub id: String, // UUID or ULID
    pub title: String,
    pub url: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub icon: String, // favicon
    pub sort: i32,
    #[serde(default)]
    pub is_favorite: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String, // hex-code like #FF5733
}
