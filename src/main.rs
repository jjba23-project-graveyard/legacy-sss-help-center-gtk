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

use gtk::prelude::*;
use gtk::{glib, Box, Button, Label, Orientation};

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

        let app_box = Box::new(Orientation::Vertical, 8);

        app_box.append(&app_title);
        app_box.append(&app_button);

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
