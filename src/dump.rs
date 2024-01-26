let pixbuf = page_pixbuf.clone();
drawing_area.set_size_request(pixbuf.width(), pixbuf.height());
drawing_area.set_hexpand(false);
drawing_area.set_vexpand(false);

let line_points = Rc::new(RefCell::new(Vec::new()));
let erase_points = Rc::new(RefCell::new(Vec::new()));

let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, pixbuf.width(), pixbuf.height()).unwrap();
let surface = Rc::new(RefCell::new(surface));

let gesture = gtk::GestureDrag::new();
gesture.connect_drag_begin(clone!(@weak line_points => move |gesture, _, _| {
    let (start_x, start_y) = gesture.start_point();
    line_points.borrow_mut().push((start_x, start_y));
}));
gesture.connect_drag_update(clone!(@weak line_points, @weak drawing_area, @weak surface => move |gesture, _, _| {
    let (offset_x, offset_y) = gesture.offset();
    let (start_x, start_y) = gesture.start_point();
    line_points.borrow_mut().push((start_x + offset_x, start_y + offset_y));

    let cr = cairo::Context::new(&*surface.borrow());
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.arc(start_x + offset_x, start_y + offset_y, 10.0, 0.0, 2.0 * std::f64::consts::PI);
    cr.fill();

    drawing_area.queue_draw();
}));
gesture.connect_drag_end(clone!(@weak line_points => move |_, _, _| {
    line_points.borrow_mut().clear();
}));

let erase_gesture = gtk::GestureDrag::new();
erase_gesture.connect_drag_begin(clone!(@weak erase_points => move |gesture, _, _| {
    let (start_x, start_y) = gesture.start_point();
    erase_points.borrow_mut().push((start_x, start_y));
}));
erase_gesture.connect_drag_update(clone!(@weak erase_points, @weak drawing_area, @weak surface => move |gesture, _, _| {
    let (offset_x, offset_y) = gesture.offset();
    let (start_x, start_y) = gesture.start_point();
    erase_points.borrow_mut().push((start_x + offset_x, start_y + offset_y));

    let cr = cairo::Context::new(&*surface.borrow());
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.0); // transparent color
    cr.arc(start_x + offset_x, start_y + offset_y, 10.0, 0.0, 2.0 * std::f64::consts::PI);
    cr.fill();

    drawing_area.queue_draw();
}));
erase_gesture.connect_drag_end(clone!(@weak erase_points => move |_, _, _| {
    erase_points.borrow_mut().clear();
}));

drawing_area.add_controller(gesture);
drawing_area.add_controller(erase_gesture);

drawing_area.set_draw_func(clone!(@weak surface => move |_, cr, _width, _height| {
    cr.set_source_pixbuf(&pixbuf, 0.0, 0.0);
    cr.paint();

    cr.set_source_surface(&*surface.borrow(), 0.0, 0.0);
    cr.paint();
}));