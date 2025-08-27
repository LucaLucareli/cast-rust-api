// dtos/user_dto.rs
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
}
