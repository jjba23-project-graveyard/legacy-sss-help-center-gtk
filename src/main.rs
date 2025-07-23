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

use std::iter::Map;
use std::process::Command;

use adw::builders::NavigationSplitViewBuilder;
use adw::{NavigationPage, NavigationSplitView};
use gtk::glib::closure_local;
use gtk::{
    glib, Box, Button, Label, ListBox, Orientation, PolicyType, ScrolledWindow, SizeGroup,
    SizeGroupMode,
};
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

fn make_labels_from_bindings(vla: &Value, size_group: &SizeGroup) -> Vec<ListBoxRow> {
    vla.as_object()
        .into_iter()
        .flatten()
        .flat_map(|(key, val)| {
            let mut rows = Vec::new();
            match val {
                // If the value is an object, it's a category.
                // Create a title row and recurs to get its children.
                Value::Object(_) => {
                    let title_row = ListBoxRow::new();
                    let title_label = Label::builder().label(key).use_markup(true).build();
                    title_row.set_child(Some(&title_label));

                    rows.push(
                        ListBoxRow::builder()
                            .child(&Label::builder().label("").build())
                            .build(),
                    );

                    rows.push(title_row);
                    rows.push(
                        ListBoxRow::builder()
                            .child(&Label::builder().label("").build())
                            .build(),
                    );
                    rows.extend(make_labels_from_bindings(val, size_group)); // Recursive call
                }
                // If it's a string, it's a keybinding entry.
                Value::String(s) => {
                    let row = ListBoxRow::new();
                    let kb_label = Label::builder()
                        .label(format!("<tt>{}</tt>", key))
                        .use_markup(true)
                        .selectable(true)
                        .xalign(0.0)
                        .margin_start(6)
                        .build();

                    size_group.add_widget(&kb_label);

                    let expl_label = Label::builder()
                        .label(s.as_str())
                        .wrap(true)
                        .xalign(0.0)
                        .selectable(true)
                        .margin_start(6)
                        .build();

                    let row_grid = gtk::Grid::builder().column_spacing(24).build();

                    row_grid.attach(&kb_label, 0, 0, 1, 1);
                    row_grid.attach(&expl_label, 1, 0, 1, 1);

                    row.set_child(Some(&row_grid));
                    row.set_vexpand(false);
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

        let welcome_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .vexpand(false)
            .build();

        welcome_box.append(&app_title);
        welcome_box.append(&app_button);

        let system_keybindings: SystemKeybindings = fetch_all_data().unwrap();

        let size_group = SizeGroup::new(SizeGroupMode::Horizontal);

        let sidebar_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(24)
            .margin_end(24)
            .build();

        let welcome_page_button = Button::with_label("SSS");
        let power_user_page_button = Button::with_label("Power user");

        sidebar_box.append(&welcome_page_button);
        sidebar_box.append(&power_user_page_button);

        let app_sidebar = NavigationPage::builder().child(&sidebar_box).build();

        let power_user_page = make_power_user_page(
            system_keybindings.power_user_bindings.as_object().unwrap(),
            &size_group,
        );

        let welcome_page = NavigationPage::builder().child(&welcome_box).build();

        let navigation_split_view = NavigationSplitView::builder()
            .sidebar(&app_sidebar)
            .content(&welcome_page)
            .build();

        // welcome_page_button.connect_clicked(move |_| {
        //     navigation_split_view.set_content(Some(&welcome_page));
        // });

        power_user_page_button.connect_clicked(move |_| {
            &navigation_split_view.set_content(Some(&power_user_page));
        });

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("SSS Help")
            .default_width(350)
            .default_height(200)
            .content(&navigation_split_view)
            .build();

        window.present();
    });

    application.run()
}

fn make_power_user_page(
    pubb: &serde_json::Map<String, Value>,
    size_group: &SizeGroup,
) -> NavigationPage {
    let power_user_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .vexpand(false)
        .build();

    let boxes: Vec<ListBox> = pubb
        .iter()
        .map(|(kk, vv)| {
            let pu_labels = make_labels_from_bindings(&vv, size_group);

            let list_box = ListBox::builder()
                .margin_top(16)
                .margin_bottom(16)
                .margin_start(24)
                .margin_end(24)
                .vexpand(false)
                .build();

            let lll = Label::new(Some(format!("{}", kk).as_str()));
            list_box.append(&lll);
            pu_labels.iter().for_each(|l| list_box.append(l));
            list_box
        })
        .collect();

    boxes.iter().for_each(|b| power_user_box.append(b));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .min_content_height(360)
        .child(&power_user_box)
        .build();

    NavigationPage::builder()
        .title("Power user session")
        .child(&scrolled_window)
        .build()
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
