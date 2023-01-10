use crate::generated::schema::{todos, todos_relations};
use utoipa::ToSchema;

#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: i16,
    pub tags: Option<Vec<Option<String>>>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = todos)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: i16,
    pub tags: Option<Vec<Option<String>>>,
}

#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = todos_relations)]
pub struct TodoRelation {
    pub id: i32,
    pub parent_todo_id: Option<i32>,
    pub child_todo_id: Option<i32>,
    pub relationship_type: i16,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = todos_relations)]
pub struct NewTodoRelation {
    pub parent_todo_id: Option<i32>,
    pub child_todo_id: Option<i32>,
    pub relationship_type: i16,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LinkTodosRequest {
    pub todo_ids: Vec<i32>,
    pub relationship_type: i16,
}
