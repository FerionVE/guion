//! part of [`Env`]
use super::*;

/// Type compound
pub trait Backend<E>: Sized + 'static where E: Env<Backend=Self> {
    type Renderer: Render<E>;
    type Event: Event<E>;
    ///TODO move tree'd back to Event
    type EventFilter: Filter<E>;
    type Style: Style<E>;
    type Size: Gonstraints;
}
