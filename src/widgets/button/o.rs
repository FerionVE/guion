use crate::core::widget::link::Link;
use crate::core::widget::Widget;
use crate::core::lazout::Orientation;
use std::marker::PhantomData;
use crate::core::lazout::size::Size;
use crate::core::util::bounded_widget::BoundedWidget;
use crate::core::env::Env;

pub struct Button<E> where E: Env {
    id: E::WidgetID,
    invalid: bool,
    parent: Option<E::WidgetID>,
    caption: String,
    action: fn(Link<E>),
}

impl<E> super::IButton<E> for Button<E> where E: Env + 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn action(&self) -> fn(Link<E>) {
        self.action
    }
    fn caption(&self) -> &str {
        &self.caption
    }

    fn invalid(&self) -> bool {
        self.invalid
    }
    fn set_invalid(&mut self, v: bool) {
        self.invalid = v;
    }

    fn parent(&self) -> Option<&E::WidgetID> {
        self.parent.as_ref()
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.parent = v;
    }
}

impl<E> Widget<E> for Button<E> where E: Env + 'static {
    crate::impl_button!(Button<E>,E);
}