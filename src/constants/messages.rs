pub struct DatabaseMessages;

impl DatabaseMessages {
    pub const NOT_EXISTS: &'static str = "User does not exists";
    pub const ALREADY_EXISTS: &'static str = "User already exists";
}

pub struct TokenMessages;

impl TokenMessages {
    pub const INVALID: &'static str = "Invalid token";
}

pub struct BearerValidationMessages;

impl BearerValidationMessages {
    pub const MISSING_HEADER: &'static str = "No Authorization header present on request";
    pub const MALFORMED_HEADER: &'static str = "Malformed auth header";
    pub const MISSING_BEARER: &'static str = "No Bearer present on Authorization header";
}