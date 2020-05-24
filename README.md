# guion

[Documentation](https://docs.rs/guion/0.2.0-dev2/)

guion is heavily WIP right now, API and concepts are still in flux.

Goals: 
- Data is the Widget: Widgets own data or reference data as immediate widget.
- Widget Tree separate from Context, State and side-data
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
