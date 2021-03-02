<img align="left" alt="" src="https://raw.githubusercontent.com/FerionVE/guion/430c18e7/res/icon.svg" height="150" />

# guion

[![crates.io](https://img.shields.io/crates/v/guion?style=flat-square)](https://crates.io/crates/guion)
[![docs.rs](https://img.shields.io/badge/docs-docs.rs-0f?style=flat-square)](https://docs.rs/guion)
[![rustc 1.50+](https://img.shields.io/badge/rustc-1.50+-ab6000?style=flat-square)](https://blog.rust-lang.org/2021/02/11/Rust-1.50.0.html)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/guion?style=flat-square)

&nbsp;

guion is heavily WIP right now, API and concepts are still in flux.

Goals: 
- Data is the Widget: Widgets own model/data or reference model/data as immediate widget.
- Widget Tree/Model separate from Context, State and side-data
- Modularity: Components are defined in generics and traits
  - Standard implementations can be combined with e.g. a custom ID implementation.
- Standard Widgets: Rich Set of Standard Widgets
  - Ability to own or reference data (immediate widgets)
- Standard Context: with features like hover, tabulating, and drag/drop in the future...
- Layout Calculation, Invalidation, ...

Future Goals:
- Simplicity
    - Right now the flexibility causes a degree of complexity
    - A simple interface limited to specific contexes and backends can still be implemented
    - Some complexitiy is currenty caused by language limitations
- Performance
    - is not the priority right not
    - although guion is designed in a manner of performance
    - Rust is strong in inlining the complexity away
    - (Performance should be good enough because stuff is implemented inline-friendly)

Check out [guion_sdl2](https://github.com/ferionve/guion_sdl2) for a (also WIP) backend implementation.
