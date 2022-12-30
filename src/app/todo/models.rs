#[derive(Queryable, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: i16,
    pub tags: Option<Vec<Option<String>>>,
}
