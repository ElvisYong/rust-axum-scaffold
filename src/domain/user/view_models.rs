// View models is where we define the data that will be returned to the client
// This is also where we can define the data that will be accepted from the client
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::models::User;

/// This is the view model that will be returned to the client
/// Utoipa's ToSchema is used to generate the openapi documentation
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserViewModel {
    #[schema(example = "ppId123")]
    id: String,
    #[schema(example = "pp@gmail.com")]
    email: String,
    #[schema(example = "pplogin")]
    username: String,
    #[schema(example = "I love to eat")]
    bio: String,
    #[schema(example = "https://www.pexels.com/photo/selective-focus-photography-of-orange-tabby-cat-1170986")]
    image: Option<String>,
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
