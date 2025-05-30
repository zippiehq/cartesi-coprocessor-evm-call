use thiserror::Error;

use revm::database_interface::DBErrorMarker;

#[derive(Error, Debug)]
pub enum GIOError {
    #[error("ivalid gio url")]
    InvalidURL,

    #[error("failed to emit gio: {0}")]
    EmitFailed(String),

    #[error("{message:?}: gio response code - {response_code:?}")]
    BadResponse { message: String, response_code: u32 },

    #[error("gio returned invalid data: {0}")]
    BadResponseData(String),
}

impl DBErrorMarker for GIOError {}
