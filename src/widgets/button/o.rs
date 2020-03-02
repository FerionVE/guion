use crate::core::event::variants::KbdDown;
use crate::core::event::VariantSupport;
use crate::core::*;
use ctx::aliases::*;
use render::widgets::RenderStdWidgets;
use widget::link::Link;
use ctx::*;
use state::handler::*;

pub struct Button<E> where E: Env, ERenderer<E>: RenderStdWidgets<E>, ECHandler<E>: AsHandlerStateful<E> {
    id: E::WidgetID,
    invalid: bool,
    parent: Option<E::WidgetID>,
    style: EStyle<E>,
    caption: String,
    action: fn(Link<E>),
}

impl<E> super::IButton<E> for Button<E> where E: Env + 'static, ERenderer<E>: RenderStdWidgets<E>, ECHandler<E>: AsHandlerStateful<E>, (EEvent<E>,&Bounds,u64): VariantSupport<KbdDown<EEKey<E>>,E> {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn action(&self) -> fn(Link<E>) {
        self.action
    }
    fn caption(&self) -> &str {
        &self.caption
    }
    fn style(&self) -> &EStyle<E> {
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