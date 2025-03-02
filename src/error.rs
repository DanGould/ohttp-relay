use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};

use crate::{empty, full};

#[derive(Debug)]
pub(crate) enum Error {
    BadGateway,
    MethodNotAllowed,
    UnsupportedMediaType,
    BadRequest(String),
}

impl Error {
    pub fn to_response(&self) -> Response<BoxBody<Bytes, hyper::Error>> {
        let mut res = Response::new(empty());
        match self {
            Self::UnsupportedMediaType => *res.status_mut() = StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::BadGateway => *res.status_mut() = StatusCode::BAD_GATEWAY,
            Self::MethodNotAllowed => *res.status_mut() = StatusCode::METHOD_NOT_ALLOWED,
            Self::BadRequest(e) => {
                *res.status_mut() = StatusCode::BAD_REQUEST;
                *res.body_mut() = full(e.to_string()).boxed();
            }
        };

        res
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnsupportedMediaType => write!(f, "Unsupported media type"),
            Self::BadGateway => write!(f, "Bad gateway"),
            Self::MethodNotAllowed => write!(f, "Method not allowed"),
            Self::BadRequest(e) => write!(f, "Bad request: {}", e),
        }
    }
}

impl std::error::Error for Error {}
