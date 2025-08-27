use crate::dto::user_dto::CreateUserDto;

pub async fn fetch_users() -> Vec<String> {
    vec!["Alice".into(), "Bob".into()]
}

pub async fn create_user(payload: CreateUserDto) -> String {
    format!("user_{}_email_{}", payload.name, payload.email)
}
