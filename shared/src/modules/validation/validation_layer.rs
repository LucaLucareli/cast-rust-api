use axum::{extract::Json, http::StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use validator::Validate;

#[derive(Serialize)]
pub struct ValidationErrorResponse {
    pub message: String,
    pub errors: serde_json::Value,
}

pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub async fn validate_json<T>(
    json: Json<T>,
) -> Result<ValidatedJson<T>, (StatusCode, Json<ValidationErrorResponse>)>
where
    T: DeserializeOwned + Validate,
{
    let value = json.0;

    if let Err(validation_errors) = value.validate() {
        let mut errors = serde_json::Map::new();

        for (field, field_errors) in validation_errors.field_errors() {
            let messages: Vec<String> = field_errors
                .iter()
                .filter_map(|e| e.message.clone().map(|m| m.to_string()))
                .collect();
            errors.insert(field.to_string(), serde_json::json!(messages));
        }

        return Err((
            StatusCode::BAD_REQUEST,
            Json(ValidationErrorResponse {
                message: "Erro na validação dos dados".to_string(),
                errors: serde_json::Value::Object(errors),
            }),
        ));
    }

    Ok(ValidatedJson(value))
}
