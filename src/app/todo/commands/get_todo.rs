use crate::app::{db::PostgresPool, schema::todos, todo::models::Todo};
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;

#[get("")]
pub async fn execute(db_pool: web::Data<PostgresPool>) -> impl Responder {
    let con_result = db_pool.get();
    if let Err(e) = con_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    let mut con = con_result.unwrap();

    let query_result = web::block(move || todos::table.load::<Todo>(&mut con).unwrap()).await;

    HttpResponse::Ok().json(query_result.unwrap())
}
