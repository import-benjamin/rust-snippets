use gtk4 as gtk;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, HeaderBar, Button, BoxLayout};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {

        let headerbar = HeaderBar::builder()
            .opacity(0.3)
            .build();

        let button = Button::builder()
            .label("Hello, there")
            .build();
        
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .show_menubar(true)
            .build();

        win.set_titlebar(Some(&headerbar));
        win.set_child(Some(&button));
        // Don't forget to make all widgets visible.
        win.present();
    });

    app.run();
}