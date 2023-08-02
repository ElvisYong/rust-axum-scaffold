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

// For this test we are also not mocking but doing an actual test
// calling from the database defined in the environment
// Feel free to add mocks if required.
#[cfg(test)]
mod test {
    use crate::{
        domain::user::view_models::UserViewModel,
        repositories::user_repository::UserRepository,
        services::{service_register::get_aws_shared_config, user_service::UserService}, get_app_config,
    };

    #[tokio::test]
    async fn get_current_user_service() {
        // Arrange
        let app_config = get_app_config();
        let shared_config = get_aws_shared_config(app_config).await;
        let user_repository = UserRepository::new(&shared_config, None).await;
        let user_service = UserService::new(user_repository);

        // Act
        let res = user_service
            .get_current_user("ppId123".to_string())
            .await
            .unwrap();

        // Assert
        assert_eq!(
            res,
            UserViewModel {
                id: "ppId123".to_string(),
                email: "pp@gmail.com".to_string(),
                username: "pplogin".to_string(),
                bio: "I love to eat".to_string(),
                image: Some("https://www.pexels.com/photo/selective-focus-photography-of-orange-tabby-cat-1170986".to_string())
            }
        )
    }
}
