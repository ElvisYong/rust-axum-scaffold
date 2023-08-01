use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    email: String,
    username: String,
    bio: String,
    image: Option<String>
}