use crate::core::widget::link::Link;
use crate::core::ctx::Context;

pub struct Button<E> where E: Context {
    id: E::WidgetID,
    invalid: bool,
    parent: Option<E::WidgetID>,
    style: E::Style,
    caption: String,
    action: fn(Link<E>),
}

impl<E> super::IButton<E> for Button<E> where E: Context + 'static {
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