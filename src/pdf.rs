use gtk::{
    gdk_pixbuf::{Colorspace, Pixbuf},
    glib,
};

use pdfium_render::prelude::*;

// todo pdf_to_pixbuf
pub fn pdf_to_pixbuf() -> Result<Vec<Pixbuf>, Box<dyn std::error::Error>> {
    let pdfium = Pdfium::default();
    let doc = pdfium.load_pdf_from_file("test/galileo.pdf", None)?;
    let mut pdf_pixbuf: Vec<Pixbuf> = Vec::new();

    for (i, page) in doc.pages().iter().enumerate() {
        let pixbuf = page_to_pixbuf(&page);
        match pixbuf {
            Ok(pixbuf) => pdf_pixbuf.push(pixbuf),
            Err(e) => {
                println!("error at page {} {:?}", i, e);
                continue;
            }
        };
    }
    
    Ok(pdf_pixbuf)
}

// todo page_to_pixbuf
pub fn page_to_pixbuf(page: &PdfPage) -> Result<Pixbuf, Box<dyn std::error::Error>> {
    let render_config = PdfRenderConfig::new()
        .set_target_width(500);
    
    let bitmap = page.render_with_config(&render_config)?;
    let width = bitmap.width();
    let height = bitmap.height();
    let image_data = glib::Bytes::from_owned(bitmap.as_rgba_bytes());

    let pixbuf = Pixbuf::from_bytes(
        &image_data,
        Colorspace::Rgb,
        true,
        8,
        width as i32,
        height as i32,
        4 * width as i32,
    ); 

    Ok(pixbuf)
}