// This is where we piece all the openapi documentation together
// generate_openapi_json() will run on cargo run and will generate the openapi.json file
// This openapi.json file can then be used for documentation and testing
// Test generators such as postman-contract-test-generator can be used
// see: https://github.com/allenheltondev/postman-contract-test-generator

// For paths, we have to use __path as a prefix to import the handlers
// see https://github.com/juhaku/utoipa/blob/cea4c50112c6cc0883767a43ff611db367cd13b5/README.md?plain=1#L171
use crate::controllers::health::__path_get_health_check;
use crate::controllers::user_controller::__path_get_current_user;
use crate::domain::user::view_models::UserViewModel;
use utoipa::OpenApi;

// We use the OpenApi macro to generate the openapi documentation
// For our example spec to fulfill and use the usage of postman-contract-test-generator
// we have to include the following:
// servers, components, info description, paths, tags
#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "http://localhost:5000", description = "Local server"),
    ),
    components(schemas(UserViewModel)),
    info(description = "This is a sample generated openapi documentation for reference"),
    paths(
       get_health_check, get_current_user,
    ),
    tags(
        (name = "health", description = "Basic health check to see if the server is up"),
        (name = "user", description = "Operations about use")
    )
)]
pub struct ApiDoc;

pub fn generate_openapi_json() -> () {
    std::fs::write("openapi.json", ApiDoc::openapi().to_pretty_json().unwrap())
        .expect("Unable to create file");
}
