// The service layer is where you process the business logic
// Meaning that you will be calling the repositories here
// The data retrieved from the repositories will be processed here
// E.g sorting, filtering, pagination, or even combining data from multiple repositories
// Complex validations should also be done here, or written as a middleware or extractor to be used by the controller layer

use axum::extract::FromRef;

use crate::{
    domain::user::view_models::UserViewModel, repositories::user_repository::UserRepository,
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

    pub async fn get_current_user(&self, id: String) -> anyhow::Result<UserViewModel> {
        // Get user from database
        let user = self.user_repository.get_user_by_id(id).await?;

        // Check if user exists
        if user.is_none() {
            return Err(anyhow::anyhow!("User not found"));
        }

        Ok(user.into())
    }

}
