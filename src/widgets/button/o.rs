use crate::core::event::variants::KbdDown;
use crate::core::event::VariantSupport;
use crate::core::*;
use ctx::aliases::*;
use render::widgets::RenderStdWidgets;
use widget::link::Link;
use ctx::*;
use state::handler::*;

pub struct Button<E> where E: Env, E::Renderer: RenderStdWidgets<E>, ECHandler<E>: AsHandlerStateful<E> {
    id: E::WidgetID,
    invalid: bool,
    parent: Option<E::WidgetID>,
    style: E::Style,
    caption: String,
    action: fn(Link<E>),
}

impl<E> super::IButton<E> for Button<E> where E: Env + 'static, E::Renderer: RenderStdWidgets<E>, ECHandler<E>: AsHandlerStateful<E>, E::Event: VariantSupport<KbdDown<E::EventKey>,E> {
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