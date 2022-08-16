<img align="left" alt="" src="https://raw.githubusercontent.com/FerionVE/guion/430c18e7/res/icon.svg" height="150" />

# guion

[![crates.io](https://img.shields.io/crates/v/guion?style=flat-square)](https://crates.io/crates/guion)
[![docs.rs](https://img.shields.io/docsrs/guion?style=flat-square)](https://docs.rs/guion)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/guion?style=flat-square)

&nbsp;

guion is an experimental GUI framework, focused on flexibility (e.g. guion core flexible Widget model) and modularity (e.g. separate backend/engine, interchangeable standard components).

guion being in an experimental state, API and concepts are still in flux.

Goals: 
- Data-oriented: Widgets abstract over model, can be directly implemented onto model, or be immediate
- Widgets own model/data or reference model/data as immediate widget
- Dynamic separate Context/states
- Widget Tree/Model separate from Context, State and side-data
- Modularity
  - Separation of core, backend, and higher features
  - Components are defined in generics and traits
  - Standard implementations can be combined with e.g. a custom ID implementation.
- guion-widgets Standard Widgets: common widgets, layouting widgets, etc.
  - Can own or reference data immediate, Widget state in model or Context
- guion-std Standard Window features like focused/hovered widgets, tabulating, drag/drop
- Widget Data caching/invalidation abstractions
- Tame complexity, provide efficient abstractions to make life easier

Related Project:
- [guion_sdl2](https://github.com/FerionVE/guion_sdl2) for a prototype backend implementation

## License

This project is licensed under either of

* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT])
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0])

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
