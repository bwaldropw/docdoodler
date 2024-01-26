mod context;
mod pdf;
use cairo::glib::clone;
use cairo::Context;
use context::AppContext;
use gtk::gdk::{ButtonEvent, EventType};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::graphene::Point;
use parking_lot::lock_api::Mutex;
use pdf::pdf_to_pixbuf;

use lazy_static::lazy_static;
use parking_lot;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

use gtk::{
    gdk, glib, Application, DrawingArea, EventController, EventControllerKey,
    EventControllerLegacy, GestureClick,
};
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

    let click_pos = Rc::new(RefCell::new(None));

    let gesture = GestureClick::new();
    gesture.connect_pressed(clone!(@weak click_pos, @weak drawing_area => move |_, _, x, y| {
        *click_pos.borrow_mut() = Some((x, y));
        drawing_area.queue_draw();
    }));

    drawing_area.add_controller(gesture);

    drawing_area.set_draw_func(move |_, cr, _width, _height| {
        cr.set_source_pixbuf(&pixbuf, 0.0, 0.0);
        cr.paint();

        draw_circle(cr, 500.0, 500.0);

        if let Some((x, y)) = *click_pos.borrow() {
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.arc(x, y, 10.0, 0.0, 2.0 * std::f64::consts::PI);
            cr.fill();
        }
    });
}

fn draw_circle(cr: &Context, x: f64, y: f64) {
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.arc(x, y, 100.0, 0.0, 2.0 * std::f64::consts::PI);
    cr.fill();
}
