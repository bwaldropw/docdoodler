mod context;
mod pdf;
use context::AppContext;
use gtk::gdk_pixbuf::Pixbuf;
use parking_lot::lock_api::Mutex;
use pdf::pdf_to_pixbuf;

use lazy_static::lazy_static;
use parking_lot;
use std::env;

use gtk::{glib, Application, DrawingArea};
use gtk::{prelude::*, ApplicationWindow, ScrolledWindow};

const APP_ID: &str = "com.bwally.DocDoodler";

lazy_static! {
    static ref APPCONTEXT: parking_lot::Mutex<AppContext> = Mutex::new(AppContext::new());
}

fn main() -> glib::ExitCode {
    let path = env::current_dir().unwrap();
    println!("workspace dir: {}", path.display());

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("DocDoodler")
        .default_width(800)
        .default_height(600)
        .build();

    window.set_widget_name("main_window");

    let scrolled_window = ScrolledWindow::builder().build();

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // todo pdf_pixbuf error handling
    let pdf_pixbuf = pdf_to_pixbuf().unwrap_or_else(|_| Vec::new());

    for page_pixbuf in &pdf_pixbuf {
        let drawing_area = DrawingArea::new();
        setup_drawing_area(&drawing_area, &page_pixbuf);

        let fixed_area = gtk::Fixed::new();
        fixed_area.put(&drawing_area, 0.0, 0.0);

        vbox.append(&fixed_area);
    }

    scrolled_window.set_child(Some(&vbox));

    window.set_child(Some(&scrolled_window));

    window.present();
}

fn setup_drawing_area(drawing_area: &DrawingArea, page_pixbuf: &Pixbuf) {
    let pixbuf = page_pixbuf.clone();
    drawing_area.set_size_request(pixbuf.width(), pixbuf.height());
    drawing_area.set_hexpand(false);
    drawing_area.set_vexpand(false);

    drawing_area.set_draw_func(move |_, cr, _width, _height| {
        cr.set_source_pixbuf(&pixbuf, 0.0, 0.0);
        cr.paint();
    });
}

// todo app context

// todo page drawing area

// todo file load pdf to context
