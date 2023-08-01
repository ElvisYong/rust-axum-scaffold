use crate::domain::user::view_models::UserViewModel;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    components(schemas(UserViewModel)),
    info(description = "This is a sample generated openapi documentation for reference"),
    paths(
        crate::controllers::user::get_current_user,
    )
)]
pub struct ApiDoc;

pub fn generate_openapi_json() -> () {
    std::fs::write(
        "openapi.json",
        ApiDoc::openapi().to_pretty_json().unwrap(),
    )
    .expect("Unable to create file");
}
