use std::marker::PhantomData;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use super::*;
///NOTE that E is not the current env but the underlying
impl<E> Context<StandardCtxEnv<E>> for StandardCtx<E> where E: Env {
    fn widget(&self, i: &E::WidgetID) -> Option<&E::DynWidget> {
        self.sup.widget(i)
    }
    fn widget_mut(&mut self, i: &E::WidgetID) -> Option<&mut E::DynWidget> {
        self.sup.widget_mut(i)
    }

    fn tune_id(&self, i: &mut E::WidgetID) {
        self.sup.tune_id(i)
    }
    fn tune_id_mut(&mut self, i: &mut E::WidgetID) {
        self.sup.tune_id_mut(i)
    }
    
    #[inline] fn render_widget(&mut self, r: E::Renderer, i: &E::WidgetID, f: fn(Link<E>, E::Renderer)) {
        self.sup.render_widget(r,i,f)
    }
    #[inline] fn event_widget(&mut self, e: E::Event, i: &E::WidgetID, f: fn(Link<E>, E::Event)) {
        self.sup.event_widget(e,i,f)
    }
    #[inline] fn size_widget(&mut self, i: &E::WidgetID, f: fn(Link<E>)->Size) -> Size {
        self.sup.size_widget(i,f)
    }

    #[inline] fn link<'a>(&'a mut self, i: E::WidgetID) -> Link<'a,StandardCtxEnv<E>> {
        Link{
            ctx: self,
            widget_id: i
        }
    }

    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        None
    }
}
///Env type with a StandardCtx and wrapping an underlying Env
#[derive(Clone)]
pub struct StandardCtxEnv<E> where E: Env {
    _e: PhantomData<E>
}

impl<E> Env for StandardCtxEnv<E> where E: Env {
    type Renderer = E::Renderer;
    type Event = E::Event;
    ///regularly just dyn Widget
    type DynWidget = E::DynWidget;
    type WidgetID = E::WidgetID;
    type Commit = E::Commit;
    type Ctx = StandardCtx<E>;
}