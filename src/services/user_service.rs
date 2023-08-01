use crate::repositories::user_repository::UserRepository;

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }
}
