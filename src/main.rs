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

use adw::{HeaderBar, NavigationPage, NavigationSplitView};
use gtk::{
    glib, Align, Box, Button, Label, ListBox, Orientation, PolicyType, ScrolledWindow, SizeGroup, SizeGroupMode
};
use gtk::{prelude::*, ListBoxRow};
use serde_json::Value;

use crate::keybinds::{fetch_all_binding_data, SystemKeybindings};

pub mod keybinds;

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
                    let title_label = Label::builder().label(format!("--------- {} ---------", key)).use_markup(true).build();
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

        let app_title = Label::builder().label("Supreme Sexp System (SSS) - Help").build();
        let app_button = Button::with_label("Click me!");
        app_button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });

        let welcome_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .vexpand(false)
            .build();

        let desc_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .vexpand(false)
            .halign(Align::Center)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(32)
            .margin_end(32)
            .build();        

        welcome_box.append(&app_title);
        welcome_box.append(&app_button);

        let sss_desc_labels: Vec<Label> = vec![
                        Label::builder()
                .label("SSS (Supreme Sexp System) is a modern version of the Lisp machines of yore.")
                                .wrap(true)
                .build(),
            Label::builder()
                .label("This is a custom GNU + Linux setup lets you customize everything endlessly, inspires creativity and problem-solving, and gives you a great user experience.")
                                .wrap(true)
                .build(),

            Label::builder()
                .label("With this one obtains a computing style and programming environment that can be referred to as Lisp user space.")
                .wrap(true)
                .build(),
            Label::builder()
                .label("You can be aware of all the code that is running on your machine, which puts free GNU systems among the most secure operating systems on Earth.")
                                .wrap(true)
                .build()
        ];   
        

        sss_desc_labels.into_iter().for_each(|l| desc_box.append(&l));
                welcome_box.append(&desc_box);

        // ------------------

        let system_keybindings: SystemKeybindings = fetch_all_binding_data().unwrap();

        let size_group = SizeGroup::new(SizeGroupMode::Horizontal);

        let sidebar_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(24)
            .margin_end(24)
            .build();

        let welcome_page_button = Button::with_label("Welcome to SSS");
        let power_user_page_button = Button::with_label("Power user");

        sidebar_box.append(&welcome_page_button);
        sidebar_box.append(&power_user_page_button);

        let app_sidebar = NavigationPage::builder().child(&sidebar_box).build();

        let power_user_page = make_power_user_page(
            system_keybindings.power_user_bindings.as_object().unwrap(),
            &size_group,
        );

        let welcome_page = NavigationPage::builder().child(&welcome_box).title("Supreme Sexp System (SSS) - Help").build();

        let navigation_split_view = NavigationSplitView::builder()
            .sidebar(&app_sidebar)
            .content(&welcome_page)
            .build();
        let navigation_split_view_clone = navigation_split_view.clone();
        let navigation_split_view_clone2 = navigation_split_view.clone();

        welcome_page_button.connect_clicked(move |_| {
            let _ = &navigation_split_view_clone2.set_content(Some(&welcome_page));
        });

        power_user_page_button.connect_clicked(move |_| {
            let _ = &navigation_split_view_clone.set_content(Some(&power_user_page));
        });

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Supreme Sexp System (SSS) - Help")
            .default_width(350)
            .default_height(200)
            .content(&navigation_split_view)            
            .build();

        window.present();
    });

    application.run()
}

fn make_power_user_page(
    bindings: &serde_json::Map<String, Value>,
    size_group: &SizeGroup,
) -> NavigationPage {
    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .vexpand(false)
        .build();

    let binding_group_boxes: Vec<ListBox> = bindings
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

    binding_group_boxes.iter().for_each(|b| main_box.append(b));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .min_content_height(360)
        .child(&main_box)
        .build();

    NavigationPage::builder()
        .title("SSS - Power user session")
        .child(&scrolled_window)
        .build()
}



