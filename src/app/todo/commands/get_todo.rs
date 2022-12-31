use crate::app::{db::PostgresPool, schema::todos, todo::models::Todo};
use crate::diesel::RunQueryDsl;
use actix_web::{get, web, HttpResponse, Responder};

#[get("")]
pub async fn execute(db_pool: web::Data<PostgresPool>) -> impl Responder {
    match db_pool.get() {
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        con => {
            let res: Vec<Todo> = todos::table
                .load(&mut con.unwrap())
                .expect("Error getting todos");

            HttpResponse::Ok().json(res)
        }
    }
}
