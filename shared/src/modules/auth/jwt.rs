use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // User ID
    pub email: String,      // User email
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
    pub jti: String,        // JWT ID (unique identifier)
    pub token_type: String, // "access" or "refresh"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
}

pub struct JwtManager {
    access_secret: String,
    refresh_secret: String,
}

impl JwtManager {
    pub fn new(access_secret: String, refresh_secret: String) -> Self {
        Self {
            access_secret,
            refresh_secret,
        }
    }

    pub fn generate_token_pair(
        &self,
        user_id: &str,
        email: &str,
    ) -> Result<TokenPair, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let access_exp = now + Duration::hours(1); // 1 hora
        let refresh_exp = now + Duration::days(7); // 7 dias

        // Access Token
        let access_claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: access_exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: "access".to_string(),
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.access_secret.as_ref()),
        )?;

        // Refresh Token
        let refresh_claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: refresh_exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: "refresh".to_string(),
        };

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.refresh_secret.as_ref()),
        )?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in: access_exp.timestamp(),
            refresh_expires_in: refresh_exp.timestamp(),
        })
    }

    pub fn validate_access_token(
        &self,
        token: &str,
    ) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.access_secret.as_ref()),
            &Validation::default(),
        )?;

        if token_data.claims.token_type != "access" {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }

        Ok(token_data.claims)
    }

    pub fn validate_refresh_token(
        &self,
        token: &str,
    ) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.refresh_secret.as_ref()),
            &Validation::default(),
        )?;

        if token_data.claims.token_type != "refresh" {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }

        Ok(token_data.claims)
    }

    pub fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = self.validate_refresh_token(refresh_token)?;

        let now = Utc::now();
        let access_exp = now + Duration::hours(1);

        let access_claims = Claims {
            sub: claims.sub,
            email: claims.email,
            exp: access_exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: "access".to_string(),
        };

        encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.access_secret.as_ref()),
        )
    }
}
