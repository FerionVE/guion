pub mod render;
pub mod layout;
pub mod widget;
pub mod event;
pub mod ctx;
pub mod handler;
pub mod style;
pub mod util;
// traits for standard states
pub mod state;
// env type compound
pub mod env;
// backend type compound
pub mod backend;
// widget id
pub mod id;
// path to resolve widgets
pub mod path;

#[allow(type_alias_bounds)]
pub mod aliases;

pub(crate) use env::*;
pub(crate) use backend::*;
pub(crate) use ctx::Context;
pub(crate) use handler::*;
pub(crate) use ctx::queue::*;
pub(crate) use aliases::*;
pub(crate) use widget::*;
pub(crate) use widget::cast::*;
pub(crate) use widget::link::*;
pub(crate) use widget::as_widget::*;
pub(crate) use widget::resolvable::*;
pub(crate) use widget::resolved::*;
pub(crate) use widget::widgets::*;
pub(crate) use event::key::PressedKey;
pub(crate) use event::{Event,variant::{Variant,VariantSupport},imp::*,variants::*};
pub(crate) use layout::*;
pub(crate) use render::*;
pub(crate) use render::widgets::*;
pub(crate) use render::link::*;
pub(crate) use util::*;
pub(crate) use util::border::*;
pub(crate) use util::bounded_widget::*;
pub(crate) use util::bounds::*;
pub(crate) use util::shortlt::*;
pub(crate) use style::*;
pub(crate) use state::handler::*;
pub(crate) use id::*;
pub(crate) use path::*;
pub(crate) use std::any::Any;
pub(crate) use qwutils::*;