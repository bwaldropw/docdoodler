## DocDoodler
PDF annotation tool written in Rust.

### Stack
`gtk4-rs`: Widget toolkit for creating the GUI.
`pdfium-render`: PDF rendering to bitmaps and creating new PDF files.
`lazy_static`: Global context for app states.

### Build Instructions
Requires Cargo and Rust to be installed on your system.
```bash
git clone https://github.com/bwaldropw/docdoodler.git
cd docdoodler
cargo build
cargo run
```

### TODOs
- [x] load pdf with pdfium
- [x] pdf -> bitmap
- [x] create gtk app
- [x] render pdf pages
- [x] create drawing surfaces
- [x] pen tool
    - [] line interpolation
- [] erase tool
- [] tool buttons
- [] menu bar
- [] app context
    - [] draw states
    - [] settings
- [] file i/o
    - [] load pdf file from computer
    - [] save annotations
    - [] export pdf w/ annotations
- [] more tools
    - [] highlights
    - [] precision pen
...
