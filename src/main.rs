// Prevent terminal window from appearing
#![windows_subsystem = "windows"]

use reqwest::blocking::{multipart::Form, Client};
use serde::Deserialize;
use winrt_notification::Toast;

const URL: &str = "https://api.bayfiles.com/upload";

/// Masquerade as PowerShell to avoid issues with toast notifications.
const APP_ID: &str = Toast::POWERSHELL_APP_ID;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        // Create HTTP client and send POST request
        let resp = Client::new()
            .post(URL)
            .multipart(
                // Create form data with filepath
                Form::new()
                    .file("file", path)
                    .unwrap_or_notify("Error: unable to read file"),
            )
            .send()
            .unwrap_or_notify("Error: failed to send request. Check your internet connection.")
            // Deserialize JSON response data
            .json::<ResponseData>()
            .unwrap_or_notify("Error: received invalid response");

        // Check response status
        if resp.status {
            // Get the URL from the response data.
            // Unwrapping is safe because if the status is true, there must be data.
            let url = resp.data.unwrap().file.url.short;

            // Copy URL to clipboard and send notification
            if clipboard_win::set_clipboard_string(&url).is_ok() {
                notify2(&format!("Link copied to clipboard"), &url);
            } else {
                notify2(&url, "Could not copy link to clipboard");
            }
        } else {
            // Send an error notification with the error message provided by the response.
            // Unwrapping is safe because if the status is false, there must be an error message.
            notify(&format!("Error: {}", resp.error.unwrap().message));
        }
    } else {
        eprintln!("no file path provided");
    }
}

/// Send a toast notification with a message.
fn notify(text: &str) {
    let _ = Toast::new(APP_ID).title(text).show();
}

/// Send a toast notification with a title and a message.
fn notify2(title: &str, text: &str) {
    let _ = Toast::new(APP_ID).title(title).text1(text).show();
}

/// Custom error handling extension method.
trait ResultExt<T> {
    fn unwrap_or_notify(self, msg: &str) -> T;
}
impl<T, E: std::error::Error> ResultExt<T> for Result<T, E> {
    fn unwrap_or_notify(self, msg: &str) -> T {
        self.unwrap_or_else(|e| {
            // Send an error notification with the message and then panic with the error object
            notify(msg);
            panic!("{}", e);
        })
    }
}

/// Deserialization structs for JSON response data.
#[derive(Deserialize)]
struct ResponseData {
    status: bool,
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
    #[serde(skip_deserializing)]
    _metadata: _Metadata,
}

#[derive(Deserialize)]
struct _Url {
    #[serde(skip_deserializing)]
    _full: String,
    short: String,
}

#[derive(Deserialize, Default)]
struct _Metadata {
    #[serde(skip_deserializing)]
    _id: String,
    #[serde(skip_deserializing)]
    _name: String,
    #[serde(skip_deserializing)]
    _size: _Size,
}

#[derive(Deserialize, Default)]
struct _Size {
    #[serde(skip_deserializing)]
    _bytes: u64,
    #[serde(skip_deserializing)]
    _readable: String,
}

#[derive(Deserialize)]
struct _Error {
    message: String,
    #[serde(skip_deserializing)]
    _kind: String,
    #[serde(skip_deserializing)]
    _code: i64,
}
