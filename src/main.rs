mod context;
mod pdf;
use cairo::glib::{clone, Propagation};
use context::{AppContext, DrawType};
use gtk::gdk::Display;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::File;
use parking_lot::lock_api::Mutex;
use pdf::pdf_to_pixbuf;

use lazy_static::lazy_static;
use parking_lot;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::sync::Arc;

use gtk::{gdk, glib, Align, Application, Button, CssProvider, DrawingArea, Image, Label, Picture};
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
    app.connect_startup(|_| load_css());
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

    // orient containers horizontally
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    // page container
    let scrolled_window = ScrolledWindow::builder().hexpand(true).build();

    let page_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    page_container.set_halign(Align::Center);

    let pdf_pixbuf = pdf_to_pixbuf().unwrap_or_else(|_| Vec::new());

    // create drawing area for each page
    let mut page_index: u32 = 1;
    for page_pixbuf in &pdf_pixbuf {
        let drawing_area = DrawingArea::new();
        setup_drawing_area(&drawing_area, &page_pixbuf);

        let fixed_area = gtk::Fixed::new();
        fixed_area.put(&drawing_area, 0.0, 0.0);

        page_container.append(&fixed_area);

        let label_str = format!("<span foreground=\"#9b9b9e\">Page {}</span>", page_index);
        let page_label = Label::builder()
            .use_markup(true)
            .label(&label_str)
            .margin_top(10)
            .margin_bottom(10)
            .build();
        page_index += 1;

        page_container.append(&page_label);
    }

    scrolled_window.set_child(Some(&page_container));

    // button container
    let button_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .width_request(30)
        .build();

    // buttons
    let pen_image = Image::from_file("assets/pen-fill.png");

    let pen_button = Button::new();
    pen_button.set_child(Some(&pen_image));
    
    let app_context = Arc::clone(&APPCONTEXT);
    pen_button.connect_clicked(move |_| {
        let mut state = app_context.lock();
        state.draw_type = DrawType::PEN;
    });

    let eraser_image = Image::from_file("assets/eraser-fill.png");

    let eraser_button = Button::new();
    eraser_button.set_child(Some(&eraser_image));

    let app_context = Arc::clone(&APPCONTEXT);
    eraser_button.connect_clicked(move |_| {
        let mut state = app_context.lock();
        state.draw_type = DrawType::ERASE;
    });

    button_container.append(&pen_button);
    button_container.append(&eraser_button);

    hbox.append(&scrolled_window);
    hbox.append(&button_container);

    window.add_controller(keyboard_controller);
    window.set_child(Some(&hbox));
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
                        dx * dx + dy * dy > 200.0
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

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("assets/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}