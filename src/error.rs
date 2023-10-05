use std::io;

use serde_json;
use thiserror::Error;

#[doc(hidden)]
#[derive(Debug, Error)]
pub enum InkErrorCode {
    #[error("Ink error: {0}")]
    Message(String),
    #[error("IO error from Ink")]
    Io(#[from] io::Error),
    #[error("JSON parsing error from Ink")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct InkError {
    #[from]
    code: InkErrorCode,
}
