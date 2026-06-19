#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub sort: i32
}