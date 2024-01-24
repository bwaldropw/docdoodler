use gtk::{
    gdk::Texture,
    gdk_pixbuf::{Colorspace, Pixbuf},
    glib,
    Image,
};
use pdfium_render::prelude::*;

pub fn test_pdf_to_image() -> Vec<Image> {
    let pdfium = Pdfium::default();
    let doc = pdfium.load_pdf_from_file("test/galileo.pdf", None).unwrap();
    let mut images: Vec<Image> = Vec::new();

    for (i, page) in doc.pages().iter().enumerate() {
        let image = test_page_to_gtk_image(&page);
        match image {
            Ok(image) => {
                image.set_pixel_size(500);
                images.push(image)
            }
            Err(e) => {
                println!("error at page {} {:?}", i, e);
                continue;
            }
        };
    }

    images
}

pub fn test_page_to_gtk_image(page: &PdfPage) -> Result<Image, Box<dyn std::error::Error>> {
    let render_config = PdfRenderConfig::new()
        .set_target_width(500)
        .set_maximum_height(1000);

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

    let texture = Texture::for_pixbuf(&pixbuf);
    let image = Image::from_paintable(Some(&texture));

    Ok(image)
}

// todo pdf_to_pixbuf

// todo page_to_pixbuf