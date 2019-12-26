use crate::core::event::key::Key;
use crate::core::event::Destination;
use super::*;

pub trait Backend<E>: Sized + 'static where E: Env<Backend=Self> {
    type Renderer: Render<E>;
    type Event: Event<E>;
    type EventDest: Destination;
    type EventKey: Key;
    type EventConsuming;
    type Style: Style<E>;
}