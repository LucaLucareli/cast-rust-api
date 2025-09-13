use chrono::{Datelike, Utc};
use validator::ValidationError;

pub fn validate_release_year(year: &Option<i32>) -> Result<(), ValidationError> {
    if let Some(y) = year {
        let current_year = Utc::now().year();

        if *y > current_year {
            let mut err = ValidationError::new("future_year");
            err.message = Some("O ano não pode ser no futuro".into());
            return Err(err);
        }

        if *y < 1800 {
            let mut err = ValidationError::new("ancient_year");
            err.message = Some("O ano parece ser muito antigo".into());
            return Err(err);
        }
    }
    Ok(())
}

pub struct ReleaseYear(pub Option<i32>);

impl ReleaseYear {
    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_release_year(&self.0)
    }
}
