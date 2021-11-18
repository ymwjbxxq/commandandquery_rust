use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    InternalError(String),
    NotFound(String),
    ValidationError(String),
}

impl std::error::Error for ApplicationError {}

impl fmt::Display for ApplicationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ApplicationError::InternalError(msg) => write!(f, "InternalError: {}", msg),
      ApplicationError::NotFound(msg) => write!(f, "NotFound: {}", msg),
      ApplicationError::ValidationError(err) => write!(f, "ValidationError: {}", err),
    }
  }
}
