use crate::app::schema::todos;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: i16,
    pub tags: Option<Vec<Option<String>>>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = todos)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: i16,
    pub tags: Option<Vec<Option<String>>>,
}
