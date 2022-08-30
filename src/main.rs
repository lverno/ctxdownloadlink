// Prevent terminal window from appearing (in release mode)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;

use data::ResponseData;
use notify_rust::Notification;
use reqwest::blocking::{multipart::Form, Client};
use std::time::Duration;

/// Upload URL
const URL: &str = "https://api.bayfiles.com/upload";

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        // Create HTTP client and send POST request
        let resp: ResponseData = Client::new()
            .post(URL)
            .multipart(
                // Create form data with filepath
                Form::new()
                    .file("file", path)
                    .unwrap_or_notify("Error: unable to read file"),
            )
            // Disable timeout
            .timeout(Duration::from_secs(999999))
            .send()
            .unwrap_or_notify("Error: failed to upload file. Check your internet connection.")
            // Deserialize the JSON data into ResponseData
            .json()
            .unwrap_or_notify("Error: received invalid response");

        // Check response status
        if resp.status {
            // Get the URL from the response data
            let url = resp.url();
            // Copy URL to clipboard and send notification
            if clipboard_win::set_clipboard_string(url).is_ok() {
                notify2("Link copied to clipboard", url);
            } else {
                notify2(url, "Could not copy link to clipboard");
            }
        } else {
            // Send an error notification with the error message provided by the response.
            notify(&format!("Error: {}", resp.err_msg()));
        }
    }
}

/// Send a toast notification with a message.
fn notify(text: &str) {
    drop(Notification::new().body(text).show())
}

/// Send a toast notification with a title and a message.
fn notify2(title: &str, text: &str) {
    drop(Notification::new().summary(title).body(text).show())
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
