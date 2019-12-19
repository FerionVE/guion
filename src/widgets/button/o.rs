use crate::core::ctx::aliases::ECStateful;
use crate::core::ctx::aliases::ECHLink;
use crate::core::render::widgets::RenderStdWidgets;
use crate::core::widget::link::Link;
use crate::core::ctx::*;

pub struct Button<E> where E: Env, E::Renderer: RenderStdWidgets<E>, ECHLink<E>: AsHandlerStateful<E,E::Context> + AsHandler<ECStateful<E>,E::Context> {
    id: E::WidgetID,
    invalid: bool,
    parent: Option<E::WidgetID>,
    style: E::Style,
    caption: String,
    action: fn(Link<E>),
}

impl<E> super::IButton<E> for Button<E> where E: Env + 'static, E::Renderer: RenderStdWidgets<E>, ECHLink<E>: AsHandlerStateful<E,E::Context> + AsHandler<ECStateful<E>,E::Context> {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn action(&self) -> fn(Link<E>) {
        self.action
    }
    fn caption(&self) -> &str {
        &self.caption
    }
    fn style(&self) -> &E::Style {
        &self.style
    }

    fn invalid(&self) -> bool {
        self.invalid
    }
    fn set_invalid(&mut self, v: bool) {
        self.invalid = v;
    }

    fn parent(&self) -> Option<E::WidgetID> {
        self.parent.clone()
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.parent = v;
    }
}

crate::impl_button!(Button<E>);