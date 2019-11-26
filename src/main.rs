#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Builder, Button};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let overvew = include_str!("./ui/canteen_list.glade");
    let builder = Builder::new_from_string(overvew);

    let window: gtk::Window = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    set_titlebar(&window);

    window.show_all();
}

fn set_titlebar(window: &gtk::Window) {
    let bar = gtk::HeaderBar::new();
    bar.set_title(Some("Gnome Canteens"));
    bar.set_subtitle(Some("Select your favorite canteens"));
    bar.set_show_close_button(true);
    let back_button = Button::new_from_icon_name(Some("gtk-go-back"), gtk::IconSize::Menu);
    back_button.connect_clicked(|button: &gtk::Button| {
        println!("foo, button clicked");
    });
    bar.pack_start(&back_button);
    window.set_titlebar(Some(&bar));
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
