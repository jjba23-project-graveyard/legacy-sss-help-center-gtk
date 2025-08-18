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

use adw::{NavigationPage, NavigationSplitView};
use gtk::prelude::*;
use gtk::{glib, Box, Button, Orientation};

use crate::keybinds::{fetch_all_binding_data, SystemKeybindings};
pub mod keybinds;

use crate::widgets::{make_keybinding_page, make_welcome_page};
pub mod widgets;

use gettextrs::*;

fn main() -> glib::ExitCode {
    let exe_path = std::env::current_exe().unwrap();
    let bin_path = exe_path.parent().unwrap();
    let store_path = bin_path.parent().unwrap();
    let locale_path = format!("{}/share/resources/locale", store_path.display());

    let _ = bindtextdomain("sss-help-center", locale_path);
    let _ = bind_textdomain_codeset("sss-help-center", "UTF-8");
    let _ = textdomain("sss-help-center");
    // let _ = setlocale(LocaleCategory::LcMessages, "en_US.UTF-8");

    let application = adw::Application::builder()
        .application_id("sss.Help")
        .build();

    application.connect_activate(|app| {
        // fetch keybdindings for SSS
        let system_keybindings: SystemKeybindings = fetch_all_binding_data().unwrap();

        // create app pages
        let power_user_page =
            make_keybinding_page(system_keybindings.power_user_bindings.as_object().unwrap());

        let universal_page = make_keybinding_page(
            system_keybindings
                .universal_session_bindings
                .as_object()
                .unwrap(),
        );

        let welcome_page = make_welcome_page();

        // create UI to navigate pages
        let welcome_page_button = Button::builder().label("Welcome to SSS").build();
        let universal_session_page_button = Button::builder().label("Universal session").build();
        let power_user_page_button = Button::builder().label("Power user").build();

        let sidebar_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(24)
            .margin_end(24)
            .build();

        sidebar_box.append(&welcome_page_button);
        sidebar_box.append(&universal_session_page_button);
        sidebar_box.append(&power_user_page_button);

        let app_sidebar = NavigationPage::builder().child(&sidebar_box).build();

        let navigation_split_view = NavigationSplitView::builder()
            .sidebar(&app_sidebar)
            .content(&welcome_page)
            .build();
        let navigation_split_view_clone = navigation_split_view.clone();
        let navigation_split_view_clone2 = navigation_split_view.clone();
        let navigation_split_view_clone3 = navigation_split_view.clone();

        welcome_page_button.connect_clicked(move |_| {
            let _ = &navigation_split_view_clone2.set_content(Some(&welcome_page));
        });

        power_user_page_button.connect_clicked(move |_| {
            let _ = &navigation_split_view_clone.set_content(Some(&power_user_page));
        });

        universal_session_page_button.connect_clicked(move |_| {
            let _ = &navigation_split_view_clone3.set_content(Some(&universal_page));
        });

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Supreme Sexp System (SSS)")
            .default_width(350)
            .default_height(200)
            .content(&navigation_split_view)
            .build();

        window.present();
    });

    application.run()
}
