#![doc(html_logo_url="https://raw.githubusercontent.com/FerionVE/guion/430c18e7/res/icon.svg")]
#![doc(html_favicon_url="https://raw.githubusercontent.com/FerionVE/guion/8cc44b2e/res/favicon.svg")]

#![feature(generic_associated_types)]

//#![warn(clippy::all)]

pub mod widget;
pub mod backend;
pub mod env;
pub mod id;
pub mod path;
pub mod render;
pub mod event;
pub mod style;
pub mod layout;
pub mod ctx;
pub mod handler;
pub mod state;
pub mod util;
pub mod validation;
pub mod aliases;
pub mod widgets;

pub(crate) use aliases::*;
pub(crate) use backend::*;
pub(crate) use ctx::queue::*;
pub(crate) use ctx::clipboard::*;
pub(crate) use ctx::*;
pub(crate) use env::*;
pub(crate) use event::imp::*;
pub(crate) use event::key::*;
pub(crate) use event::variant::*;
pub(crate) use event::standard::variants::*;
pub(crate) use event::filter::*;
pub(crate) use event::compound::*;
pub(crate) use event::*;
pub(crate) use handler::*;
pub(crate) use id::standard::*;
pub(crate) use id::*;
pub(crate) use layout::*;
pub(crate) use path::standard::*;
pub(crate) use path::*;
pub(crate) use qwutils::*;
pub(crate) use render::link::*;
pub(crate) use render::widgets::*;
pub(crate) use render::*;
pub(crate) use state::*;
pub(crate) use state::dyn_state::*;
pub(crate) use std::any::Any;
pub(crate) use style::color::*;
pub(crate) use style::font::*;
pub(crate) use style::selectag::*;
pub(crate) use style::selectag::standard::*;
pub(crate) use style::*;
pub(crate) use util::border::*;
pub(crate) use util::bounded_widget::*;
pub(crate) use util::bounds::*;
pub(crate) use util::error::*;
pub(crate) use util::tabulate::*;
pub(crate) use util::traitcast::*;
pub(crate) use util::*;
pub(crate) use widget::array::*;
pub(crate) use widget::as_widget::*;
pub(crate) use widget::cast::*;
pub(crate) use widget::link::*;
pub(crate) use widget::resolvable::*;
pub(crate) use widget::resolved::*;
pub(crate) use widget::root::*;
pub(crate) use widget::ident::*;
pub(crate) use widget::*;

pub type EventResp = bool;
