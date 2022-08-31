// Prevent terminal window from appearing (in release mode)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notify_rust::Notification;
use reqwest::blocking::{multipart::Form, Client};
use serde_json::Value;
use std::{path::Path, time::Duration};

/// Upload URL
const URL: &str = "https://api.bayfiles.com/upload";

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        // Extract filename from path
        let filename = Path::new(&path)
            .file_name()
            .map(|s| s.to_str())
            .flatten()
            .unwrap_or("file");

        // Send request with file
        let resp: Value = Client::new()
            .post(URL)
            .multipart(Form::new().file("file", &path).unwrap_or_else(|_| {
                notify_exit(
                    &format!("Error uploading {filename}"),
                    "failed to read file",
                    1,
                )
            }))
            // Disable timeout
            .timeout(Duration::from_secs(999999))
            .send()
            .unwrap_or_else(|_| {
                notify_exit(
                    &format!("Error uploading {filename}"),
                    "failed to upload file",
                    1,
                )
            })
            // Deserialize JSON
            .json()
            .unwrap_or_else(|_| {
                notify_exit(
                    &format!("Error uploading {filename}"),
                    "received invalid response",
                    1,
                )
            });

        if resp["status"].as_bool().unwrap_or(false) {
            if let Some(link) = resp["data"]["file"]["url"]["short"].as_str() {
                if clipboard_win::set_clipboard_string(link).is_ok() {
                    notify_exit("Link copied to clipboard", link, 0)
                } else {
                    notify_exit("Failed to copy link to clipboard", link, 1)
                }
            }
        }
    }
}

/// Send a toast notification with a title and a message, then exit with an exit code.
fn notify_exit(title: &str, text: &str, code: i32) -> ! {
    drop(Notification::new().summary(title).body(text).show());
    std::process::exit(code);
}
