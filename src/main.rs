use std::{env, path};

use gtk::{prelude::*, ApplicationWindow, ScrolledWindow};
use gtk::{glib, Application};

const APP_ID: &str = "com.bwally.DocDoodler";

use img::ImageFormat;
use pdfium_render::{document, prelude::*};


fn main() -> glib::ExitCode {
    let path = env::current_dir().unwrap();
    println!("workspace dir: {}", path.display());

    // TODO logging
    match test_pdf_to_jpg() {
        Ok(_) => println!("pdf -> jpegs"),
        Err(e) => println!("{}", e),
    }

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

    for i in 0..5 {
        let image = gtk::Image::from_file("./export-test-page-0.jpg");
        image.set_pixel_size(500);
        vbox.append(&image);
    }

    scrolled_window.set_child(Some(&vbox));

    window.set_child(Some(&scrolled_window));

    window.present();
}

fn test_pdf_to_jpg() -> Result<(), PdfiumError> {
    let pdfium = Pdfium::default();
    let doc = pdfium.load_pdf_from_file("test/test-file.pdf", None)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000);

    for (index, page) in doc.pages().iter().enumerate() {
        let result = page
            .render_with_config(&render_config)?
            .as_image()
            .as_rgba8()
            .ok_or(PdfiumError::ImageError)?
            .save_with_format(format!("export-test-page-{}.jpg", index), ImageFormat::Jpeg);

        assert!(result.is_ok());
    }

    Ok(())
}

// fn pdf_to_image

// fn render_image

// fn
