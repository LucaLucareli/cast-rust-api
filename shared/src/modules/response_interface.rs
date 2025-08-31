use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseInterface<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
}

impl<T> Default for ResponseInterface<T> {
    fn default() -> Self {
        Self {
            message: None,
            result: None,
        }
    }
}
