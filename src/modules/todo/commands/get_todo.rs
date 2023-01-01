use crate::{
    generated::schema::todos,
    modules::todo::models::Todo,
    utils::{db::*, http::*},
};
use actix_web::{get, web, Responder};
use diesel::prelude::*;

#[get("")]
pub async fn execute(db_pool: web::Data<PostgresPool>) -> impl Responder {
    db_pool.run(|con| {
        todos::table
            .load::<Todo>(con)
            .into_res("Error reading todos")
    })
}
