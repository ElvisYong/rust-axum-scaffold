// View models is where we define the data that will be returned to the client
// This is also where we can define the data that will be accepted from the client
use serde::{Deserialize, Serialize};

use super::models::User;

#[derive(Serialize, Deserialize)]
struct UserViewModel {
    id: String,
    email: String,
    username: String,
    bio: String,
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
