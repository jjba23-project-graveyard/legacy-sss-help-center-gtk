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

use gtk::builders::GridBuilder;
use gtk::{glib, Box, Button, Label, ListBox, Orientation, PolicyType, ScrolledWindow};
use gtk::{prelude::*, ListBoxRow};
use serde_json::Value;

fn make_labels_from_bindings(vla: Value) -> Vec<ListBoxRow> {
    if let Some(pu_object_map) = vla.as_object() {
        pu_object_map
            .iter()
            .map(|t| {
                if let Some(pp_object_map) = t.1.as_object() {
                    let xs: Vec<ListBoxRow> = pp_object_map
                        .values()
                        .into_iter()
                        .map(|vv| make_labels_from_bindings(vv.clone()))
                        .flatten()
                        .collect();

                    let b = ListBoxRow::new();
                    let tit = Label::new(Some(t.0));
                    tit.set_use_markup(true);
                    b.set_child(Some(&tit));
                    let xx = vec![b];

                    xx.into_iter().chain(xs).collect()
                } else if let Some(pp_s) = t.1.as_str() {
                    let b = ListBoxRow::new();
                    let kb = Label::new(Some(format!("<tt>{}</tt>", t.0).as_str()));
                    kb.set_use_markup(true);
                    kb.set_xalign(0.0);
                    kb.set_hexpand(false);
                    kb.set_width_request(10);
                    let expl = Label::new(Some(pp_s));
                    expl.set_use_markup(true);
                    expl.set_wrap(true);
                    expl.set_xalign(0.0);
                    expl.set_hexpand(true);

                    let row_grid = gtk::Grid::builder()
                        .row_spacing(8)
                        .column_spacing(24)
                        .build();

                    row_grid.attach(&kb, 0, 0, 1, 1);
                    row_grid.attach(&expl, 1, 0, 1, 1);

                    b.set_child(Some(&row_grid));
                    vec![b]
                } else {
                    vec![]
                }
            })
            .flatten()
            .collect()
    } else {
        vec![]
    }
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

        let system_keybindings: SystemKeybindings = fetch_and_display_data().unwrap();

        let pu_labels = make_labels_from_bindings(system_keybindings.power_user_bindings);

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

#[derive(Debug, Clone)]
struct FetchKeybindingsError(String);

#[derive(Debug, Clone)]
struct SystemKeybindings {
    power_user_bindings: Value,
    universal_session_bindings: Value,
}

fn fetch_and_display_data() -> Result<SystemKeybindings, FetchKeybindingsError> {
    let power_user_dconf_key = "/sss/power-user-keybindings-docs";
    let universal_session_dconf_key = "/sss/universal-session-keybindings-docs";

    let power_user_output: Result<std::process::Output, std::io::Error> = Command::new("dconf")
        .arg("read")
        .arg(power_user_dconf_key)
        .output();

    let power_user_bindings = parse_bindings(power_user_output)?;

    let universal_session_output: Result<std::process::Output, std::io::Error> =
        Command::new("dconf")
            .arg("read")
            .arg(universal_session_dconf_key)
            .output();

    let universal_session_bindings = parse_bindings(universal_session_output)?;

    Ok(SystemKeybindings {
        power_user_bindings: power_user_bindings,
        universal_session_bindings: universal_session_bindings,
    })
}

fn parse_bindings(
    command_output: Result<std::process::Output, std::io::Error>,
) -> Result<Value, FetchKeybindingsError> {
    let dconf_output: Result<String, FetchKeybindingsError> = match command_output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                return Err(FetchKeybindingsError(format!(
                    "Error: dconf command failed.\nStderr: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
        }
        Err(e) => {
            return Err(FetchKeybindingsError(format!(
                "Error executing dconf command: {}",
                e
            )));
        }
    };

    match dconf_output {
        Ok(d) => {
            // dconf often returns string values wrapped in single quotes.
            // We need to remove these quotes before attempting Base16 decoding.
            let base16_encoded = if d.starts_with('\'') && d.ends_with('\'') {
                d[1..d.len() - 1].to_string()
            } else {
                d
            };

            let decoded_bytes = match base16::decode(&base16_encoded) {
                Ok(bytes) => bytes, // If successful, we get a vector of bytes.
                Err(e) => {
                    // If Base16 decoding fails, return an error message with the original input.
                    return Err(FetchKeybindingsError(format!(
                        "Error decoding Base16: {}. Input: '{}'",
                        e, base16_encoded
                    )));
                }
            };

            let json_string = match String::from_utf8(decoded_bytes) {
                Ok(s) => s,
                Err(e) => {
                    return Err(FetchKeybindingsError(format!(
                        "Error converting bytes to UTF-8: {}",
                        e
                    )));
                }
            };

            let parsed_json: serde_json::Result<serde_json::Value> =
                serde_json::from_str(&json_string);
            let pp = parsed_json.map_err(|e| FetchKeybindingsError(e.to_string()));
            pp
        }
        Err(e) => return Err(e),
    }
}
