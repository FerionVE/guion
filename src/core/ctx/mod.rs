use std::borrow::BorrowMut;
use crate::core::widget::handlez::fns::WidgetFns;
use crate::core::style::Style;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;
use std::any::Any;

//pub mod ctx_meta; TODO fix CtxMeta
//pub use ctx_meta::*;

pub mod id;
pub use id::*;

pub mod aliases;
use aliases::*;

pub mod queue;
pub use queue::*;

pub mod handler;
pub use handler::*;

pub mod stateful;
pub use stateful::*;

mod imp;

pub trait Env: Sized + 'static {
    type Context: Context + Widgets<Self>;
    type Renderer: Render<Self>;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: WidgetID;
    type Commit: Eq + Ord;
    type Style: Style;
}

pub trait Widgets<E>: 'static where E: Env {
    fn widget(&self, i: &E::WidgetID) -> Option<&E::DynWidget>;
    fn widget_mut(&mut self, i: &E::WidgetID) -> Option<&mut E::DynWidget>;

    #[inline]
    fn has_widget(&self, i: &E::WidgetID) -> bool {
        self.widget(i).is_some()
    }

    #[inline] fn tune_id(&self, _i: &mut E::WidgetID) {}
    #[inline] fn tune_id_mut(&mut self, _i: &mut E::WidgetID) {}
}

pub trait Context: Sized + 'static {
    type Link: for<'a> HandlerAccess<'a,Self>;
    type Handler: Handler<Self>;
    //type Meta: ContextMeta;

    #[inline] 
    fn handler<H: Handler<Self>>(&mut self) -> &mut H where Self::Link: for<'a> AsHandler<'a,H> {
        self.handler_access().into_mut()
    }
    #[inline] 
    fn handler_access(&mut self) -> Self::Link {
        HandlerAccess::from_ctx(self)
    }

    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env<Context=Self>>(&mut self, i: &E::WidgetID, r: E::Renderer) where Self: Widgets<E> {
        Self::Handler::_render::<E>(self.handler_access(),i,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env<Context=Self>>(&mut self, i: &E::WidgetID, e: E::Event) where Self: Widgets<E> {
        Self::Handler::_event::<E>(self.handler_access(),i,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env<Context=Self>>(&mut self, i: &E::WidgetID) -> Size where Self: Widgets<E> {
        Self::Handler::_size::<E>(self.handler_access(),i)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn widget_fns<E: Env<Context=Self>>(&self, i: &E::WidgetID) -> WidgetFns<E> where Self: Widgets<E> {
        Widget::_fns(self.widget(i).expect("Lost Widget"))
    }

    #[inline] fn link<'a,E: Env<Context=Self>>(&'a mut self, i: &E::WidgetID) -> Link<'a,E> where Self: Widgets<E> {
        Link{
            ctx: self,
            widget_id: i.clone(),
        }
    }

    #[inline] fn hovered<E: Env<Context=Self>>(&self) -> Option<E::WidgetID> where Self: Widgets<E>, Self::Link: AsHandlerStateful<E>, Self::Link: for<'a> AsHandler<'a,ECStateful<E>> {
        self.handler_access().stateful().hovered()
    }
    #[inline] fn selected<E: Env<Context=Self>>(&self) -> Option<E::WidgetID> where Self: Widgets<E>, Self::Link: AsHandlerStateful<E>, Self::Link: for<'a> AsHandler<'a,ECStateful<E>> {
        self.handler_access().stateful().selected()
    }

    #[inline]
    fn is_hovered<E: Env<Context=Self>>(&self, i: &E::WidgetID) -> bool where Self: Widgets<E>, Self::Link: AsHandlerStateful<E>, Self::Link: for<'a> AsHandler<'a,ECStateful<E>> {
        self.hovered().map_or(false, |w| w == *i )
    }
    #[inline]
    fn is_selected<E: Env<Context=Self>>(&self, i: &E::WidgetID) -> bool where Self: Widgets<E>, Self::Link: AsHandlerStateful<E>, Self::Link: for<'a> AsHandler<'a,ECStateful<E>> {
        self.selected().map_or(false, |w| w == *i )
    }
}