use crate::controllers::user_controller::__path_get_current_user;
use crate::domain::user::view_models::UserViewModel;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "http://localhost:5000", description = "Local server"),
    ),
    components(schemas(UserViewModel)),
    info(description = "This is a sample generated openapi documentation for reference"),
    paths(
        get_current_user,
    ),
    tags(
        (name = "user", description = "Operations about use")
    )
)]
pub struct ApiDoc;

pub fn generate_openapi_json() -> () {
    std::fs::write("openapi.json", ApiDoc::openapi().to_pretty_json().unwrap())
        .expect("Unable to create file");
}
