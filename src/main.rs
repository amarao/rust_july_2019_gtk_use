extern crate gtk;
extern crate gio;
extern crate gdk;

use gtk::prelude::*;
use gio::prelude::*;
use std::env;
use cairo;
use gtk::{BoxExt, ContainerExt, DrawingArea, ScrolledWindowExt, StateFlags, WidgetExt};
use std::cell::RefCell;
use gdk::{WindowExt};

fn app( app:&gtk::Application) {
        let win = gtk::ApplicationWindow::new(app);

        win.set_default_size(1920, 1080);
        win.set_title("Basic example");
        let frame = gtk::Frame::new(None);
        let area = DrawingArea::new();
        area.connect_draw(move|w, c|{
            println!("w: {} c:{}",w, c);
            c.rectangle(1.0, 1.0, 100.0, 200.0);
            c.fill();
            gtk::Inhibit(false)
        });
        frame.add(&area);
        win.add(&frame);
        // gtk::container_add(win, frame);
        win.show_all();
}

fn main() {
    let uiapp = gtk::Application::new("org.example.gtk_use.ops",
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(app);

    uiapp.run(&env::args().collect::<Vec<_>>());
}
