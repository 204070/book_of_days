use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject::{self, Reject},
    Filter, Rejection,
};

use crate::iam::services::auth_service::decode_jwt;

const BEARER: &str = "Bearer ";

#[derive(Debug)]
pub struct AuthError;
impl Reject for AuthError {}

pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| headers)
        .and_then(authorize)
}

async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<String, Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let claims = decode_jwt(&jwt).map_err(|_| reject::custom(AuthError))?;
            Ok(claims.sub)
        }
        Err(_) => return Err(reject::custom(AuthError)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, AuthError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(AuthError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(AuthError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(AuthError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
