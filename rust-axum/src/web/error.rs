use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFail,

    // -- Auth
    AuthFailNoAuthTokenCookies,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

// region:          --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion:       --- Error Boilerplate

// region:          --- IntoResponse impl
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the error into the response
        response.extensions_mut().insert(self);

        response
    }
}
// endregion:       --- IntoResponse impl

// region:          --- Client Error impl
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            // -- Login
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth
            Self::AuthFailCtxNotInRequestExt
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailNoAuthTokenCookies => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
        }
    }
}
// endregion:       --- Client Error impl

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
}
