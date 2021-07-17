use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}
pub type JWTToken = String;

pub fn sign_jwt(username: &String) -> Result<JWTToken, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(15))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.clone(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret("JWT_SECRET".as_ref()),
    )
}

pub fn decode_jwt(jwt_token: &String) -> Result<Claims, Error> {
    let decoded = decode::<Claims>(
        jwt_token,
        &DecodingKey::from_secret("JWT_SECRET".as_ref()),
        &Validation::new(Algorithm::HS512),
    )?;

    Ok(decoded.claims)
}
