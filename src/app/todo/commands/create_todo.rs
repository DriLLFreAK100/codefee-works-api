use crate::app::{
    db::PostgresPool,
    schema::todos,
    todo::models::{CreateTodoRequest, Todo},
};
use crate::diesel::RunQueryDsl;
use actix_web::{post, web, HttpResponse, Responder};

#[post("")]
pub async fn execute(
    db_pool: web::Data<PostgresPool>,
    req_body: web::Json<CreateTodoRequest>,
) -> impl Responder {
    match db_pool.get() {
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        con => {
            let res: Todo = diesel::insert_into(todos::table)
                .values(req_body.into_inner())
                .get_result(&mut con.unwrap())
                .expect("Error saving new todo");

            // Return inserted data
            HttpResponse::Ok().json(res)
        }
    }
}
