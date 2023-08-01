use axum::extract::FromRef;

use crate::repositories::user_repository::UserRepository;

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
}
