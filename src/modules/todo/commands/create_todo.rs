use crate::{
    generated::schema::todos,
    modules::todo::models::{Todo, UpdateTodoRequest},
    utils::{db::*, http::*},
};
use actix_web::{post, web, Responder};
use diesel::prelude::*;

#[post("")]
pub async fn execute(
    db_pool: web::Data<PostgresPool>,
    req_body: web::Json<UpdateTodoRequest>,
) -> impl Responder {
    let req_body = &req_body.into_inner();

    db_pool.get().run(|con| {
        diesel::insert_into(todos::table)
            .values(req_body)
            .get_result::<Todo>(con)
            .into_res("Error creating todo")
    })
}
