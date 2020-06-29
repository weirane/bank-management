use actix_http::ResponseBuilder;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use log::{error, info};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("actix: {0}")]
    Actix(#[from] actix_web::Error),

    #[error("database: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("template: {0}")]
    Tera(#[from] tera::Error),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    /// Malformed request
    #[error("bad request: {0}")]
    BadRequest(&'static str),

    /// Error with a message
    #[error("{0}")]
    Msg(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        if let StatusCode::OK = code {
            info!("error: {:?}", self);
        } else {
            error!("{:?}", self);
        }
        ResponseBuilder::new(code).body(format!("{}", self))
    }

    fn status_code(&self) -> StatusCode {
        use Error::*;
        match self {
            Actix(e) => e.as_response_error().status_code(),
            BadRequest(_) => StatusCode::BAD_REQUEST,
            Msg(_) => StatusCode::OK,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
