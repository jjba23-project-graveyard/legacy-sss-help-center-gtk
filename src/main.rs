// sss-help

// Copyright © Josep Bigorra <jjbigorra@gmail.com>

// sss-help is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// sss-help is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with sss-help.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::process::Command;

use gtk::builders::TextViewBuilder;
use gtk::{glib, Box, Button, Label, Orientation, TextView};
use gtk::{prelude::*, TextBuffer};

fn main() -> glib::ExitCode {
    let application = adw::Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let app_title = Label::new(Some("Supreme Sexp System (SSS) - Help"));
        let app_button = Button::with_label("Click me!");
        app_button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });

        let power_user_keybindings_docs = fetch_and_display_data();
        let app_puk = Label::new(Some(&power_user_keybindings_docs));

        let app_box = Box::new(Orientation::Vertical, 8);

        app_box.append(&app_title);
        app_box.append(&app_button);
        app_box.append(&app_puk);

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .content(&app_box)
            .build();

        window.present();
    });

    application.run()
}

// Asynchronous function to handle the core logic:
// 1. Execute `dconf read`.
// 2. Base16 decode the output.
// 3. Parse the decoded string as JSON.
// 4. Format the result for display.
fn fetch_and_display_data() -> String {
    // Define the dconf key that we want to read.
    let dconf_key = "/sss/power-user-keybindings-docs";

    // Execute the `dconf read` command.
    // `Command::new` creates a new command, `arg` adds arguments, and `output()` executes it.
    let output = Command::new("dconf").arg("read").arg(dconf_key).output();

    // Process the result of the dconf command execution.
    let dconf_output = match output {
        Ok(output) => {
            // Check if the command executed successfully (exit code 0).
            if output.status.success() {
                // Convert the stdout (standard output) bytes to a UTF-8 string.
                // `String::from_utf8_lossy` handles invalid UTF-8 characters gracefully.
                // `.trim()` removes leading/trailing whitespace (like newlines).
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                // If the command failed, return an error message including its stderr.
                return format!(
                    "Error: dconf command failed.\nStderr: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            // If the command itself could not be executed (e.g., `dconf` not found in PATH),
            // return an error message.
            return format!("Error executing dconf command: {}", e);
        }
    };

    // dconf often returns string values wrapped in single quotes (e.g., `'deadbeef'`).
    // We need to remove these quotes before attempting Base16 decoding.
    let base16_encoded = if dconf_output.starts_with('\'') && dconf_output.ends_with('\'') {
        // Slice the string to remove the first and last characters (the quotes).
        dconf_output[1..dconf_output.len() - 1].to_string()
    } else {
        // If no quotes are present, use the output string as is.
        dconf_output
    };

    // Attempt to decode the Base16 encoded string into raw bytes.
    let decoded_bytes = match base16::decode(&base16_encoded) {
        Ok(bytes) => bytes, // If successful, we get a vector of bytes.
        Err(e) => {
            // If Base16 decoding fails, return an error message with the original input.
            return format!("Error decoding Base16: {}. Input: '{}'", e, base16_encoded);
        }
    };

    // Attempt to convert the decoded bytes into a UTF-8 string.
    // This assumes the Base16 decoded content is valid UTF-8 JSON.
    let json_string = match String::from_utf8(decoded_bytes) {
        Ok(s) => s, // If successful, we get the JSON string.
        Err(e) => {
            // If the bytes cannot be interpreted as UTF-8, return an error.
            return format!("Error converting bytes to UTF-8: {}", e);
        }
    };

    // Attempt to parse the UTF-8 string as a generic JSON `serde_json::Value`.
    // This is flexible and can handle any valid JSON structure (objects, arrays, primitives).
    let parsed_json: serde_json::Result<serde_json::Value> = serde_json::from_str(&json_string);

    match parsed_json {
        Ok(json_value) => {
            // If parsing to `serde_json::Value` is successful, pretty-print the JSON.
            // Pretty-printing makes the JSON human-readable with indentation.
            match serde_json::to_string_pretty(&json_value) {
                Ok(pretty_json) => pretty_json, // Return the pretty-printed JSON string.
                Err(e) => format!("Error pretty-printing JSON: {}", e), // Error during pretty-printing.
            }
        }
        Err(e) => {
            // If parsing to `serde_json::Value` fails (e.g., invalid JSON format),
            // attempt to parse it as a simple `HashMap<String, String>`.
            // This is a fallback for simpler JSON structures.
            let parsed_hashmap: serde_json::Result<HashMap<String, String>> =
                serde_json::from_str(&json_string);
            match parsed_hashmap {
                Ok(hashmap) => {
                    // If parsing to HashMap is successful, format it as simple key-value pairs.
                    let mut display_text = String::new();
                    for (key, value) in hashmap {
                        display_text.push_str(&format!("{}: {}\n", key, value));
                    }
                    display_text // Return the formatted key-value string.
                }
                Err(_) => {
                    // If neither `serde_json::Value` nor `HashMap` parsing works,
                    // it means the content is likely not valid JSON or is in an unexpected format.
                    // Return the original JSON parsing error and the raw decoded content for debugging.
                    format!(
                        "Error parsing JSON: {}\nRaw decoded content:\n{}",
                        e, json_string
                    )
                }
            }
        }
    }
}
