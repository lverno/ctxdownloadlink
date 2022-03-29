//! Deserialization structs for JSON response data.

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ResponseData {
    pub(crate) status: bool,
    data: Option<_Data>,
    error: Option<_Error>,
}

#[derive(Deserialize)]
struct _Data {
    file: _File,
}

#[derive(Deserialize)]
struct _File {
    url: _Url,
}

#[derive(Deserialize)]
struct _Url {
    short: String,
}

#[derive(Deserialize)]
struct _Error {
    message: String,
}

/// Helper methods for getting the URL and error message
impl ResponseData {
    /// Get the URL (assuming data is present).
    pub(crate) fn url(&self) -> &str {
        &self.data.as_ref().unwrap().file.url.short
    }

    /// Get the error message (assuming there is an error).
    pub(crate) fn err_msg(&self) -> &str {
        &self.error.as_ref().unwrap().message
    }
}
