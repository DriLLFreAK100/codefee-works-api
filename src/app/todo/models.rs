use diesel::prelude::*;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
}
