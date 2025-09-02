use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInputDTO {
    #[validate(length(min = 5, message = "Email deve ter pelo menos 5 caracteres"))]
    #[validate(email(message = "Email inválido"))]
    pub email: String,

    #[validate(length(min = 8, message = "Senha deve ter pelo menos 8 caracteres"))]
    pub password: String,

    #[validate(length(min = 3, message = "Nome tem que ter pelo menos 3 caracteres"))]
    pub name: String,
}
