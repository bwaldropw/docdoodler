mod pdf;
mod app_context;
use app_context::ApplicationContext;
use pdf::test_pdf_to_image;

use std::env;

use gtk::{glib, Application, DrawingArea};
use gtk::{prelude::*, ApplicationWindow, ScrolledWindow};
use cairo::Context;

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref APP_CONTEXT: Mutex<ApplicationContext> = Mutex::new(ApplicationContext {
        path: String::new(),
    });
}

const APP_ID: &str = "com.bwally.DocDoodler";

fn main() -> glib::ExitCode {
    let path = env::current_dir().unwrap();
    println!("workspace dir: {}", path.display());

    let mut app_context = APP_CONTEXT.lock().unwrap();
    app_context.path = String::from("test/galileo.pdf");

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

    // let images = test_pdf_to_image();
    // for image in &images {
    //     vbox.append(image);
    // }

    // pixbuf vector to drawing area for each page




    scrolled_window.set_child(Some(&vbox));

    window.set_child(Some(&scrolled_window));

    window.present();
}


// todo app context

// todo page drawing area

// todo file load pdf to context
