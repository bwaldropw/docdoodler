## DocDoodler
PDF annotation tool written in Rust.

![Alt Text](https://i.giphy.com/h8yjHJwiibuxe3lFVS.webp)

### Stack
[gtk4-rs](https://gtk-rs.org/) : Widget toolkit for creating the GUI. <br>
[pdfium-render](https://github.com/ajrcarey/pdfium-render) : PDF rendering to bitmaps and creating new PDF files. <br>
[lazy_static](https://crates.io/crates/lazy_static) : Global context for app states.

### Build Instructions
```bash
git clone https://github.com/bwaldropw/docdoodler.git
cd docdoodler
cargo build
cargo run
```

### Features
`Pen Tool`(b): Draw lines <br>
`Erase Tool`(e): Erase lines <br>

### TODOs
- [x] load pdf with pdfium
- [x] pdf -> bitmap
- [x] create gtk app
- [x] render pdf pages
- [x] create drawing surfaces
- [x] pen tool
    - [ ] line interpolation
- [x] erase tool
- [x] tool buttons
- [ ] tool cursors
    - [ ] pen tool
    - [ ] erase tool
- [ ] menu bar
- [ ] refactor main.rs
- [ ] app context
    - [ ] draw states
    - [ ] settings
    - [ ] save/load context from file
- [ ] file i/o
    - [ ] load pdf file from computer
    - [ ] save annotations
    - [ ] export pdf w/ annotations
- [ ] more tools
    - [ ] undo/redo
    - [ ] highlights
    - [ ] precision pen
 ...
