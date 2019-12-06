use std::marker::PhantomData;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;

pub trait Context: Sized {
    type Renderer: Render<Self>;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: PartialEq + Clone;
    type Commit: Eq + Ord;

    fn widget(&self, i: &Self::WidgetID) -> Option<&Self::DynWidget>;
    fn widget_mut(&mut self, i: &Self::WidgetID) -> Option<&mut Self::DynWidget>;

    fn tune_id(&self, _i: &mut Self::WidgetID) {}
    fn tune_id_mut(&mut self, _i: &mut Self::WidgetID) {}
    
    fn fns<E>(&self) -> ContextFns<Self,E> where E: Context<WidgetID=Self::WidgetID> + BorrowMut<Self> {
        ContextFns{
            render: render::<E>,
            event: event::<E>,
            size: size::<E>,
            _e: PhantomData,
        }
    }

    #[inline] fn link<'a>(&'a mut self, i: Self::WidgetID) -> Link<'a,Self> {
        Link{
            ctx: self,
            widget_id: i
        }
    }

    #[inline] fn hovered(&self) -> Option<Self::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<Self::WidgetID> {
        None
    }
}

fn render<F: Context>(l: Link<F>, r: F::Renderer, f: fn(Link<F>,F::Renderer)) {
    f(l,r)
}
fn event<F: Context>(l: Link<F>, r: F::Event, f: fn(Link<F>,F::Event)) {
    f(l,r)
}
fn size<F: Context>(l: Link<F>, f: fn(Link<F>)-> Size) -> Size {
    f(l)
}

pub struct ContextFns<E,F> where E: Context, F: Context<WidgetID=E::WidgetID> + BorrowMut<E> {
    pub render: fn(Link<F>, F::Renderer, fn(Link<F>,F::Renderer)),
    pub event: fn(Link<F>, F::Event, fn(Link<F>,F::Event)),
    pub size: fn(Link<F>, fn(Link<F>)->Size)->Size,
    _e: PhantomData<E>,
}