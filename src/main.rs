extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use std::env;

fn app( app:&gtk::Application) {
        let win = gtk::ApplicationWindow::new(app);

        win.set_default_size(1920, 1080);
        win.set_title("Basic example");

        win.show_all();
}

fn main() {
    let uiapp = gtk::Application::new("org.example.gtk_use.ops",
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(app);
    uiapp.run(&env::args().collect::<Vec<_>>());
}
