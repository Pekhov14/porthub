#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub sort: i32
}