use axum::{body::Body, http::Request};
use lazy_static::lazy_static;
use regex::Regex;

pub fn extract_bearer(request: &Request<Body>) -> Result<String, &'static str> {
    /* Check if Authorization header is present */
    let header = request.headers().get("Authorization");
    if header.is_none() {
        return Err("No Authorization header present on request");
    }

    /* Check if Bearer is prensent on Authorization header */
    let inner_string = String::from_utf8(header.unwrap().as_bytes().to_owned()).unwrap();

    if !validate_header(&inner_string) {
        return Err("Malformed auth header");
    }

    let token = has_token(&inner_string)?;

    Ok(token)
}

fn validate_header(header: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Bearer [a-zA-z0-9]+").unwrap();
    }
    RE.is_match(header)
}

fn has_token(header: &str) -> Result<String, &'static str> {
    let token = header.get(7..).map(|value| value.to_string());
    if token.is_none() {
        return Err("No Bearer present on Authorization header");
    }
    Ok(token.unwrap())
}
