mod context;
mod pdf;
use cairo::glib::{clone, Propagation};
use context::{AppContext, DrawType};
use gtk::gdk_pixbuf::Pixbuf;
use parking_lot::lock_api::Mutex;
use pdf::pdf_to_pixbuf;

use lazy_static::lazy_static;
use parking_lot;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::sync::Arc;

use gtk::{gdk, glib, Align, Application, DrawingArea, Label};
use gtk::{prelude::*, ApplicationWindow, ScrolledWindow};

const APP_ID: &str = "com.bwally.DocDoodler";

lazy_static! {
    static ref APPCONTEXT: Arc<parking_lot::Mutex<AppContext>> =
        Arc::new(Mutex::new(AppContext::new()));
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
        .default_width(850)
        .default_height(650)
        .build();

    let app_context = Arc::clone(&APPCONTEXT);

    let keyboard_controller = gtk::EventControllerKey::new();
    keyboard_controller.connect_key_pressed(move |_, key, _, _| {
        let mut state = app_context.lock();

        match key {
            gdk::Key::b => {
                state.draw_type = DrawType::PEN;
                println!("pen tool");
            }
            gdk::Key::e => {
                state.draw_type = DrawType::ERASE;
                println!("erase tool");
            }
            _ => (),
        }

        Propagation::Proceed
    });

    let scrolled_window = ScrolledWindow::builder().build();

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    vbox.set_halign(Align::Center);

    // todo pdf_pixbuf error handling
    let pdf_pixbuf = pdf_to_pixbuf().unwrap_or_else(|_| Vec::new());

    let mut page_index: u32 = 1;
    for page_pixbuf in &pdf_pixbuf {
        let drawing_area = DrawingArea::new();
        setup_drawing_area(&drawing_area, &page_pixbuf);

        let fixed_area = gtk::Fixed::new();
        fixed_area.put(&drawing_area, 0.0, 0.0);

        vbox.append(&fixed_area);

        let label_str = format!("<span foreground=\"#9b9b9e\">Page {}</span>", page_index);
        let page_label = Label::builder()
            .use_markup(true)
            .label(&label_str)
            .margin_top(10)
            .margin_bottom(10)
            .build();
        page_index += 1;

        vbox.append(&page_label);
    }

    scrolled_window.set_child(Some(&vbox));

    window.add_controller(keyboard_controller);
    window.set_child(Some(&scrolled_window));
    window.present();
}

fn setup_drawing_area(drawing_area: &DrawingArea, page_pixbuf: &Pixbuf) {
    let app_context = Arc::clone(&APPCONTEXT);
    let pixbuf = page_pixbuf.clone();
    drawing_area.set_size_request(pixbuf.width(), pixbuf.height());
    drawing_area.set_hexpand(false);
    drawing_area.set_vexpand(false);

    let line_points = Rc::new(RefCell::new(Vec::new()));

    let gesture = gtk::GestureDrag::new();
    gesture.connect_drag_begin(
        clone!(@weak line_points, @weak app_context => move |gesture, _, _| {
        let (start_x, start_y) = gesture.start_point().unwrap();
        let app_context = app_context.lock();

            match app_context.draw_type {
                DrawType::PEN => {
                    line_points.borrow_mut().push((start_x, start_y));
                },
                _ => (),
            }
        }),
    );
    gesture.connect_drag_update(
        clone!(@weak line_points, @weak drawing_area, @weak app_context => move |gesture, _, _| {
            let (offset_x, offset_y) = gesture.offset().unwrap();
            let (start_x, start_y) = gesture.start_point().unwrap();
            let point = (start_x + offset_x, start_y + offset_y);

            let app_context = app_context.lock();
            match app_context.draw_type {
                DrawType::PEN => {
                    line_points.borrow_mut().push(point);
                },
                DrawType::ERASE => {
                    line_points.borrow_mut().retain(|&(x,y)| {
                        let dx = x - point.0;
                        let dy = y - point.1;
                        dx * dx + dy * dy > 100.0
                    })
                },
            }

            drawing_area.queue_draw();
        }),
    );
    gesture.connect_drag_end(clone!(@weak line_points => move |_, _, _| {
        //line_points.borrow_mut().clear();
    }));

    drawing_area.add_controller(gesture);

    drawing_area.set_draw_func(move |_, cr, _width, _height| {
        cr.set_source_pixbuf(&pixbuf, 0.0, 0.0);
        cr.paint();

        cr.set_source_rgb(0.0, 0.0, 0.0);
        for &(x, y) in line_points.borrow().iter() {
            cr.arc(x, y, 10.0, 0.0, 2.0 * std::f64::consts::PI);
            cr.fill();
        }
    });
}
