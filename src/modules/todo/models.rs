use crate::generated::schema::{todos, todos_relations};
use utoipa::ToSchema;

// #[derive(ToSchema)]
// pub enum TodoStatus {
//     InProgress,
//     Done,
//     Cancel,
// }

#[derive(ToSchema, Copy, Clone, Serialize, Deserialize)]
pub enum TodoRelationship {
    SubTask,
    Dependency,
}

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

#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize, ToSchema, Associations)]
#[diesel(table_name = todos_relations)]
#[diesel(belongs_to(Todo, foreign_key = parent_todo_id))]
pub struct TodoRelation {
    pub id: i32,
    pub parent_todo_id: Option<i32>,
    pub child_todo_id: Option<i32>,
    pub relationship_type: i16,
}
