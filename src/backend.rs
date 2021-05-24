//! part of [`Env`]
use super::*;

/// Type compound
pub trait Backend<E>: Sized + 'static where E: Env {
    type Renderer<'a>: Render<E>+'a;
    type Event: Event<E>;
    ///TODO move tree'd back to Event
    type EventFilter: Filter<E>;
    type Style: Style<E>;
    type Size: Gonstraints;
}
