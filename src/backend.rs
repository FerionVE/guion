//! part of Env
use super::*;

/// Type compound
pub trait Backend<E>: Sized + 'static where E: Env<Backend=Self> {
    type Renderer: Render<E>;
    type Event: Event<E>;
    ///TODO move tree'd back to Event
    type EventFilter: Filter<E>;
    // TODO pending [implied_bounds](https://github.com/rust-lang/rfcs/pull/2089) feature so that the messy StdSelector deps can be moved into one trait
    type Style: Style<E> + StyleQuery<StdSelector<E>,E> + for<'a> StyleQuery<&'a [StdSelector<E>],E> + for<'a,'b> StyleQuery<&'a [&'b StdSelector<E>],E>;
    type Size: Gonstraints;
}
