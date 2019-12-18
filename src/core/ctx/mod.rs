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

pub mod queue;
pub use queue::*;

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
    type Link: for<'a> From<&'a mut Self> + AsMut<Self> + AsMut<Self::Handler>;
    type Handler: Handler<Self>;
    //type Meta: ContextMeta;

    fn handler_mut(&mut self) -> &mut Self::Handler;

    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env<Context=Self>>(&mut self, i: &E::WidgetID, r: E::Renderer) where Self: Widgets<E> {
        Self::Handler::_render::<E>(self.into(),i,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env<Context=Self>>(&mut self, i: &E::WidgetID, e: E::Event) where Self: Widgets<E> {
        Self::Handler::_event::<E>(self.into(),i,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env<Context=Self>>(&mut self, i: &E::WidgetID) -> Size where Self: Widgets<E> {
        Self::Handler::_size::<E>(self.into(),i)
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
}

pub trait Handler<C>: Sized + 'static where C: Context, C::Link: AsMut<Self> {
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env>(senf: C::Link, i: &E::WidgetID, r: E::Renderer);
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env>(senf: C::Link, i: &E::WidgetID, e: E::Event);
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env>(senf: C::Link, i: &E::WidgetID) -> Size;
}



pub trait AsHandlerStateful<E> where E: Env, <E::Context as Context>::Link: AsMut<Self::T> {
    type T: HandlerStateful<E>;

    fn stateful_mut(&mut self) -> &mut Self::T;
}

pub trait HandlerStateful<E>: Handler<E::Context> + 'static where E: Env, <E::Context as Context>::Link: AsMut<Self> {
    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        None
    }
}

pub trait ContextStateful<E>: Context + Widgets<E> where E: Env<Context=Self> {
    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        None
    }

    #[inline]
    fn is_hovered(&self, i: &E::WidgetID) -> bool {
        self.hovered().map_or(false, |w| w == *i )
    }
    #[inline]
    fn is_selected(&self, i: &E::WidgetID) -> bool {
        self.selected().map_or(false, |w| w == *i )
    }
}