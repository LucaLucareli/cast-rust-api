use crate::modules::auth::AuthService;
use crate::modules::database::repositories::users_repository::UsersRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub user_repo: Arc<UsersRepository>,
}

impl AppState {
    pub fn new(auth_service: Arc<AuthService>, user_repo: Arc<UsersRepository>) -> Self {
        Self {
            auth_service,
            user_repo,
        }
    }
}
