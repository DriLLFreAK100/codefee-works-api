use crate::modules::todo::{commands, models};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
      paths(
        // Todo
        commands::create_todo::execute,
        commands::delete_todo::execute,
        commands::get_todos::execute,
        commands::update_todo::execute,
      ),
      components(
          schemas(models::Todo, models::UpdateTodoRequest)
      ),
      tags(
          (name = "todo", description = "Todo API endpoints")
      ),
)]
struct ApiDoc;

/// Configure SwaggerUI using `utoipa`
pub fn with_swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}
