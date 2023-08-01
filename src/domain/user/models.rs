// Models are the defined structures of the data that will be stored in the database.
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    created_at: String,
    updated_at: String
}