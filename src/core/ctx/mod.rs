use std::any::Any;

use super::*;

//pub mod ctx_meta; TODO fix CtxMeta
//pub use ctx_meta::*;

#[allow(type_alias_bounds)]
pub mod aliases;
use aliases::*;

pub mod widgets;
pub use widgets::*;

pub mod queue;
pub use queue::*;

pub mod handler;
pub use handler::*;

pub trait Context: Sized + 'static {
    type Handler: Handler<Self>;
    //type Meta: ContextMeta;

    #[inline] 
    fn handler_mut<H: Handler<Self>>(&mut self) -> &mut H where Self::Handler: AsHandler<H,Self> {
        Self::Handler::as_mut(self)
    }
    #[inline] 
    fn handler<H: Handler<Self>>(&self) -> &H where Self::Handler: AsHandler<H,Self> {
        Self::Handler::as_ref(self)
    }

    fn _handler_mut(&mut self) -> &mut Self::Handler;
    fn _handler(&self) -> &Self::Handler;

    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env<Context=Self>>(&mut self, i: WPSlice<E>, r: (&mut ERenderer<E>,&Bounds)) where Self: Widgets<E> {
        Self::Handler::_render::<E>(self.link(i),r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env<Context=Self>>(&mut self, i: WPSlice<E>, e: (EEvent<E>,&Bounds)) where Self: Widgets<E> {
        Self::Handler::_event::<E>(self.link(i),e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env<Context=Self>>(&mut self, i: WPSlice<E>) -> Size where Self: Widgets<E> {
        Self::Handler::_size::<E>(self.link(i))
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn widget_fns<E: Env<Context=Self>>(&self, i: WPSlice<E>) -> WidgetFns<E> where Self: Widgets<E> {
        Widget::_fns(self.widget(i).expect("Lost Widget"))
    }

    #[inline] fn link<'a,E: Env<Context=Self>>(&'a mut self, i: WPSlice<'a,E>) -> Link<'a,E> where Self: Widgets<E> {
        Link{
            ctx: self,
            path: i,
        }
    }

    #[inline] fn state<E: Env<Context=Self>>(&self) -> &ECStateful<E> where Self: Widgets<E>, Self::Handler: AsHandlerStateful<E> {
        Self::Handler::stateful(self)
    }
}