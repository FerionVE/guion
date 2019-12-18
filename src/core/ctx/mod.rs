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
    type Context: Context<Self::HDeref> + Widgets<Self>;
    type Renderer: Render<Self>;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: WidgetID;
    type Commit: Eq + Ord;
    type Style: Style;
    type HDeref: for<'a> From<&'a mut Self::Context>;
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

pub trait Context<H>: Sized + 'static where H: for<'a> From<&'a mut Self> {
    type Handler: Handler<H>;
    //type Meta: ContextMeta;

    fn handler_mut(&mut self) -> &mut Self::Handler;

    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env<Context=Self,HDeref=H>>(&mut self, i: &E::WidgetID, r: E::Renderer) where Self: Widgets<E> {
        Self::Handler::_render::<E>(self.into(),i,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env<Context=Self,HDeref=H>>(&mut self, i: &E::WidgetID, e: E::Event) where Self: Widgets<E> {
        Self::Handler::_event::<E>(self.into(),i,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env<Context=Self,HDeref=H>>(&mut self, i: &E::WidgetID) -> Size where Self: Widgets<E> {
        Self::Handler::_size::<E>(self.into(),i)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn widget_fns<E: Env<Context=Self,HDeref=H>>(&self, i: &E::WidgetID) -> WidgetFns<E> where Self: Widgets<E> {
        Widget::_fns(self.widget(i).expect("Lost Widget"))
    }

    #[inline] fn link<'a,E: Env<Context=Self,HDeref=H>>(&'a mut self, i: &E::WidgetID) -> Link<'a,E> where Self: Widgets<E> {
        Link{
            ctx: self,
            widget_id: i.clone(),
        }
    }
}

pub trait Handler<H>: Sized + 'static {
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env>(senf: H, i: &E::WidgetID, r: E::Renderer);
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env>(senf: H, i: &E::WidgetID, e: E::Event);
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env>(senf: H, i: &E::WidgetID) -> Size;
}

pub trait AsHandler<H>: Sized where H: Handler<Self> {
    fn handler_mut(&mut self) -> &mut H;
}

/*pub trait HandlerDeref<'a> {
    type C: Handler;
    fn handler_mut(self) -> &'a mut C;
}

pub trait AsHandlerDeref<'a,H,C> where H: HandlerDeref<'a,C>, C: Handler {
    fn handler_mut(&'a mut self) -> H;
}*/

pub trait HandlerStateful<E>: 'static where E: Env {
    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        None
    }
}

pub trait ContextStateful<E>: Context<E::HDeref> where Self::Handler: HandlerStateful<E>, E: Env, E::HDeref: for<'a> From<&'a mut Self> {
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