use super::*;

pub struct WidgetFns<E> where E: Env {
    pub render: fn(Link<E>, (&mut ERenderer<E>,&Bounds,&EStyle<E>)),
    pub event: fn(Link<E>, (EEvent<E>,&Bounds)),
    pub size: fn(Link<E>) -> Size,
}