use std::{env, path};
use fltk::{*, prelude::*};
use img::ImageFormat;
use pdfium_render::{document, prelude::*};


fn main() {
    let path = env::current_dir().unwrap();
    println!("workspace dir: {}", path.display());

    match test_pdf_to_jpg() {
        Ok(_) => println!("pdf -> jpegs"),
        Err(e) => println!("{}", e),
    }

    let app = app::App::default();
    let mut window_main = window::Window::new(0, 0, 800, 600, "DocDoodler");
    window_main.end();
    window_main.show();
    app.run().unwrap();
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
