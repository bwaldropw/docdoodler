mod pdf_utils;
use pdf_utils::{test_pdf_to_jpg, test_pdf_to_image};

use std::env;

use gtk::{glib, Application, Image};
use gtk::{prelude::*, ApplicationWindow, ScrolledWindow};

const APP_ID: &str = "com.bwally.DocDoodler";

fn main() -> glib::ExitCode {
    let path = env::current_dir().unwrap();
    println!("workspace dir: {}", path.display());

    // TODO logging
    // match test_pdf_to_jpg() {
    //     Ok(_) => println!("pdf -> jpegs"),
    //     Err(e) => println!("{}", e),
    // }

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("DocDoodler")
        .build();

    window.set_default_size(800, 600);

    let scrolled_window = ScrolledWindow::builder().build();

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let images = test_pdf_to_image();
    for image in &images {
        vbox.append(image);
    }
        
    scrolled_window.set_child(Some(&vbox));

    window.set_child(Some(&scrolled_window));

    window.present();
}
