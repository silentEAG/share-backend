use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    Json, TypedHeader, response::{Response, IntoResponse},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{error::ServerError, CONFIG};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = CONFIG.jwt_secret();
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
}

impl Claims {
    pub fn new() -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(CONFIG.jwt_exp_time());
        Self {
            sub: "shark-share".into(),
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    secret_key: String,
}

fn sign() -> crate::Result<String> {
    jsonwebtoken::encode(
        &Header::default(), 
        &Claims::new(), 
        &KEYS.encoding)
        .map_err(|_| ServerError::Auth("Token Create Error".into()))
}

pub async fn handler(Json(auth): Json<AuthPayload>) -> crate::Result<Response> {
    if auth.secret_key != CONFIG.app_secret_key() {
        return Err(ServerError::Auth("Wrong Credentials".into()));
    }
    let token = sign()?;

    Ok(Json(json!({
        "status": "ok",
        "access_token": token,
    }))
    .into_response())
}

#[async_trait]
impl<S> FromRequest<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = req
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ServerError::Auth("InvalidToken".into()))?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| ServerError::Auth("InvalidToken".into()))?;

        Ok(token_data.claims)
    }
}
