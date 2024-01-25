use pdfium_render::render_config::PdfRenderConfig;

pub struct AppContext {
    pub pdf_path: String,
    pub render_config: PdfRenderConfig,
}

impl AppContext {
    pub fn new() -> Self {
        //todo load context from file
        Self {
            pdf_path: "test/galileo.pdf".to_string(),
            render_config: PdfRenderConfig::new().set_target_width(500),
        }
    }
}
