// View models is where we define the data that will be returned to the client
// This is also where we can define the data that will be accepted from the client
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::models::User;

/// This is the view model that will be returned to the client
/// Utoipa's ToSchema is used to generate the openapi documentation
/// We should include example values for the fields so that we can use the generated openapi documentation
/// and a simple postman test generation to test our endpoints using the generated json
/// see https://github.com/allenheltondev/postman-contract-test-generator
#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq, PartialOrd)]
pub struct UserViewModel {
    #[schema(example = "ppId123")]
    pub id: String,
    #[schema(example = "pp@gmail.com")]
    pub email: String,
    #[schema(example = "pplogin")]
    pub username: String,
    #[schema(example = "I love to eat")]
    pub bio: String,
    #[schema(
        example = "https://www.pexels.com/photo/selective-focus-photography-of-orange-tabby-cat-1170986"
    )]
    pub image: Option<String>,
}

/// This is for quick conversion from the model to the view model which can be used in the handlers
/// by calling user.into() or UserViewModel::from(user)
impl From<User> for UserViewModel {
    fn from(user: User) -> Self {
        UserViewModel {
            id: user.id,
            email: user.email,
            username: user.username,
            bio: user.bio,
            image: user.image,
        }
    }
}
