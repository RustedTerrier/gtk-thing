extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn make_hello_window(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("Click me!");

    window.add(&button);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.rustedterrier.gtk-thing"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        make_hello_window(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
