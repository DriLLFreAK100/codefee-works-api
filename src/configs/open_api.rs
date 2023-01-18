use crate::{
    modules::todo::{commands, models},
    utils::env::get_host_port,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
      paths(
        // Todo
        commands::create_todo::create_todo,
        commands::delete_todo::delete_todo,
        commands::get_todo::get_todo,
        commands::get_todos::get_todos,
        commands::link_todos::link_todos,
        commands::update_todo::update_todo,
      ),
      components(
            schemas(
                // Shared
                models::Todo, 
                models::UpdateTodoRequest,
                models::TodoRelation,
                // models::TodoStatus,
                models::TodoRelationship,
                
                // Req/Res DTOs
                commands::get_todo::RelatedTodoResponse,
                commands::get_todo::TodoDetailsResponse,
                commands::link_todos::LinkTodosRequest,
            )
      ),
      tags(
          (name = "todo", description = "Todo API endpoints")
      ),
)]
struct ApiDoc;

/// Configure SwaggerUI using `utoipa`
pub fn with_swagger() -> SwaggerUi {
    let (host, port) = get_host_port();
    println!("Visit Swagger UI at http://{}:{}/swagger-ui/#", host, port);

    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}
