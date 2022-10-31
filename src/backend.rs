//! part of [`Env`]

use crate::env::Env;
use crate::event::Event;
use crate::layout::Gonstraints;
use crate::render::Render;
use crate::style::Style;

/// Type compound
pub trait Backend<E>: Sized + 'static where E: Env {
    type Renderer<'a>: Render<E>+'a;
    type Event: Event<E>;
    ///TODO move tree'd back to Event
    //type EventFilter: Filter<E>;
    type Style: Style<E>;
    type Size: Gonstraints;
    type TextLayout: crate::text::layout::TxtLayout<E>;
}
