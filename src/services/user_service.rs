// The service layer is where you process the business logic
// Meaning that you will be calling the repositories here
// The data retrieved from the repositories will be processed here
// E.g sorting, filtering, pagination, or even combining data from multiple repositories
// Business level validations should also be done here
// e.g checking if a user is already registered before creating a new user
// Input level validations should be done in the controller layer instead or by a middleware

use axum::extract::FromRef;
use serde_dynamo::from_item;
use tracing::log::error;

use crate::{
    domain::user::{models::User, view_models::UserViewModel},
    errors::{AppError, AppResult},
    repositories::user_repository::UserRepository,
};

use super::service_register::ServiceRegister;

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
}

/// This implementation is to for us to extract substates from our main state in handlers for each router
/// Implement FromRef for each service, alternatively we can use the #[derive(FromRef)] macro
/// However with Macro we will be required to extract State<Option<YourService>> instead of State<YourService>
/// Therefore the verbose implementation like this would be convenient
impl FromRef<ServiceRegister> for UserService {
    fn from_ref(state: &ServiceRegister) -> Self {
        state.user_service.clone().unwrap()
    }
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn get_current_user(self, id: String) -> AppResult<UserViewModel> {
        // Get user from database
        let dynamo_items = self.user_repository.get_user_by_id(id).await?;

        match dynamo_items {
            Some(dynamo_items) => {
                // Convert the dynamo item into a User model
                let user: User = match from_item(dynamo_items) {
                    Ok(users) => users,
                    Err(e) => {
                        error!("Error while converting dynamo item into User model: {}", e);
                        return Err(AppError::SerdeDynamoError(e));
                    }
                };

                // Convert the User model into a UserViewModel and return
                Ok(UserViewModel::from(user))
            }
            None => Err(AppError::NotFound("User not found".to_string())),
        }
    }
}
