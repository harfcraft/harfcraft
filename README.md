# HarfCraft

**HarfCraft** is an experimental, UFO font IDE built in Rust using [gpui-kit](https://github.com/longbridge/gpui-kit) and [Google Fontations](https://github.com/googlefonts/fontations).

Right now, making a font usually means choosing between three options:
- Graphical editors that are proprietary and macOS exclusive. 
- FontForge which is cross-platform but has severely started showing its age.
- Code IDE + Python scripts powerful for logic, but painful when you need a visual feedback.

That’s where HarfCraft comes in: a proper font IDE that hits the sweet spot. You get a high-performance visual canvas for vector editing, but your font is managed like source code—stored as clean UFO directories, fully inspectable, and easy to track in Git.

## Roadmap

- [ ] **Workspace & File System**
  - [ ] UFO v3 project loading and file tree navigation
  - [ ] DesignSpace file support

- [ ] **Git & Version Control Integration**
  - [ ] Native Git status indicators in file tree and glyph grid
  - [ ] Visual diff for glyphs (side-by-side bezier path overlay for modified glyphs)
  - [ ] Branch switching and stash management without corrupting UFO XML state
  
- [ ] **Glyph Canvas & Vector Engine**
  - [ ] High-performance GPUI canvas with pan/zoom
  - [ ] Cubic and quadratic Bezier node editing via `kurbo`
  - [ ] Smart guides, snapping, and component transformation

- [ ] **Metrics & Kerning**
  - [ ] Visual metrics editor (sidebearings and advance widths)
  - [ ] Class-based kerning editor and preview pairs

- [ ] **OpenType Features & Code**
  - [ ] `.fea` (OpenType feature) editor with LSP support (autocompletion, diagnostics, syntax highlighting)
  - [ ] Live compilation and feature hot-reloading

- [ ] **Shaping, Inspection & Debugging**
  - [ ] Live preview pane
  - [ ] Interactive shaping debugger (inspect GSUB/GPOS lookup chains step-by-step)
  - [ ] Glyph ID, unicode, and table metadata inspector

- [ ] **Build & Export**
  - [ ] Export to OTF, TTF, and WOFF2 binaries
  - [ ] Variable font target compilation

## Building from source

You need Rust installed along with standard GPUI dependencies on Linux.

```bash
git clone https://github.com/aalouaoui/harfcraft 
cd harfcraft
cargo build --release
```

## License

Apache 2.0

## Contributing

HarfCraft is currently a solo experiment to test the feasibility of a GPUI + Fontations font IDE.

Contributions are closed for now until I validate the concept and core architecture.

In the meantime, feel free to star the repo, test builds locally, or follow along with the progress!
