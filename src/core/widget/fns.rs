use super::*;

pub struct WidgetFns<E> where E: Env {
    pub render: fn(Link<E>, RenderLink<E>),
    pub event: fn(Link<E>, (EEvent<E>,&Bounds,u64)),
    pub size: fn(Link<E>) -> Size,
}