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

use std::process::Command;

use gtk::{glib, Box, Button, Label, ListBox, Orientation, PolicyType, ScrolledWindow};
use gtk::{prelude::*, ListBoxRow};
use serde_json::Value;
use std::fmt;

#[derive(Debug)]
enum KeybindingError {
    DconfCommand(std::io::Error),
    DconfFailed(String),
    Base16(base16::DecodeError),
    Utf8(std::string::FromUtf8Error),
    Json(serde_json::Error),
}

impl fmt::Display for KeybindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DconfCommand(e) => write!(f, "Failed to execute dconf command: {}", e),
            Self::DconfFailed(stderr) => write!(f, "dconf command failed: {}", stderr),
            Self::Base16(e) => write!(f, "Failed to decode Base16 string: {}", e),
            Self::Utf8(e) => write!(f, "Failed to parse UTF-8 string: {}", e),
            Self::Json(e) => write!(f, "Failed to parse JSON: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
struct SystemKeybindings {
    power_user_bindings: Value,
    universal_session_bindings: Value,
}

fn make_labels_from_bindings(vla: &Value) -> Vec<ListBoxRow> {
    // Use flat_map to simplify collecting nested results
    vla.as_object()
        .into_iter()
        .flatten()
        .flat_map(|(key, val)| {
            let mut rows = Vec::new();
            match val {
                // If the value is an object, it's a category title.
                // Create a title row and recurse to get its children.
                Value::Object(_) => {
                    let title_row = ListBoxRow::new();
                    let title_label = Label::builder().label(key).use_markup(true).build();
                    title_row.set_child(Some(&title_label));

                    rows.push(title_row);
                    rows.extend(make_labels_from_bindings(val)); // Recursive call
                }
                // If it's a string, it's a keybinding entry.
                Value::String(s) => {
                    let row = ListBoxRow::new();
                    let kb_label = Label::new(Some(&format!("<tt>{}</tt>", key)));
                    kb_label.set_use_markup(true);
                    kb_label.set_xalign(0.0);

                    let expl_label = Label::new(Some(s.as_str()));
                    expl_label.set_wrap(true);
                    expl_label.set_xalign(0.0);

                    let row_grid = gtk::Grid::builder().column_spacing(24).build();

                    row_grid.attach(&kb_label, 0, 0, 1, 1);
                    row_grid.attach(&expl_label, 1, 0, 1, 1);

                    row.set_child(Some(&row_grid));
                    rows.push(row);
                }
                // Ignore other JSON value types
                _ => {}
            }
            rows
        })
        .collect()
}

fn main() -> glib::ExitCode {
    let application = adw::Application::builder()
        .application_id("sss.Help")
        .build();

    application.connect_activate(|app| {
        let app_title = Label::new(Some("Supreme Sexp System (SSS) - Help"));
        let app_button = Button::with_label("Click me!");
        app_button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });

        let system_keybindings: SystemKeybindings = fetch_all_data().unwrap();

        let pu_labels = make_labels_from_bindings(&system_keybindings.power_user_bindings);

        let list_box = ListBox::new();
        list_box.set_margin_top(12);
        list_box.set_margin_bottom(12);
        list_box.set_margin_start(18);
        list_box.set_margin_end(18);
        pu_labels.iter().for_each(|l| list_box.append(l));

        let app_box = Box::new(Orientation::Vertical, 12);

        app_box.append(&app_title);
        app_box.append(&app_button);
        app_box.append(&list_box);

        let scrolled_window = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .child(&app_box)
            .build();

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("SSS Help")
            .default_width(350)
            .default_height(70)
            .content(&scrolled_window)
            .build();

        window.present();
    });

    application.run()
}

fn fetch_single_binding(dconf_key: &str) -> Result<Value, KeybindingError> {
    let command_output = Command::new("dconf").arg("read").arg(dconf_key).output();
    parse_bindings(command_output)
}

fn fetch_all_data() -> Result<SystemKeybindings, KeybindingError> {
    let power_user_bindings = fetch_single_binding("/sss/power-user-keybindings-docs")?;
    let universal_session_bindings =
        fetch_single_binding("/sss/universal-session-keybindings-docs")?;

    Ok(SystemKeybindings {
        power_user_bindings,
        universal_session_bindings,
    })
}

fn parse_bindings(
    command_output: Result<std::process::Output, std::io::Error>,
) -> Result<Value, KeybindingError> {
    let output = command_output.map_err(KeybindingError::DconfCommand)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(KeybindingError::DconfFailed(stderr));
    }

    let raw_stdout = String::from_utf8(output.stdout).map_err(KeybindingError::Utf8)?;

    // dconf wraps strings in single quotes, remove them
    let base16_encoded = raw_stdout.trim().trim_matches('\'');

    // Decode from Base16, parse to UTF-8, then parse as JSON
    let decoded_bytes = base16::decode(base16_encoded).map_err(KeybindingError::Base16)?;
    let json_string = String::from_utf8(decoded_bytes).map_err(KeybindingError::Utf8)?;
    let parsed_json = serde_json::from_str(&json_string).map_err(KeybindingError::Json)?;

    Ok(parsed_json)
}
