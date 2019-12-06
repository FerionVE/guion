use std::borrow::BorrowMut;
use crate::core::ctx::ContextFns;
use crate::core::widget::Widget;
use crate::core::render::Render;
use crate::core::ctx::Context;
use std::marker::PhantomData;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use super::*;
///NOTE that E is not the current Context but the underlying
impl<E> Context for StandardCtx<E> where E: Context, E::Renderer: Render<Self>, E::DynWidget: Widget<Self> {
    type Renderer = E::Renderer;
    type Event = E::Event;
    ///regularly just dyn Widget
    type DynWidget = E::DynWidget;
    type WidgetID = E::WidgetID;
    type Commit = E::Commit;

    fn widget(&self, i: &Self::WidgetID) -> Option<&Self::DynWidget> {
        self.sup.widget(i)
    }
    fn widget_mut(&mut self, i: &Self::WidgetID) -> Option<&mut Self::DynWidget> {
        self.sup.widget_mut(i)
    }

    fn tune_id(&self, i: &mut Self::WidgetID) {
        self.sup.tune_id(i)
    }
    fn tune_id_mut(&mut self, i: &mut Self::WidgetID) {
        self.sup.tune_id_mut(i)
    }
    
    fn fns<F>(&self) -> ContextFns<Self,F> where F: Context<WidgetID=Self::WidgetID> + BorrowMut<Self> + BorrowMut<E> {
        ContextFns{
            render: render::<E,F>,
            event: event::<E,F>,
            size: size::<E,F>,
            _e: PhantomData,
        }
    }

    #[inline] fn link<'a>(&'a mut self, i: Self::WidgetID) -> Link<'a,Self> {
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

fn render<E,F>(l: Link<F>, r: F::Renderer, f: fn(Link<F>,F::Renderer)) 
    where
    E: Context,
    E::Renderer: Render<StandardCtx<E>>,
    E::DynWidget: Widget<StandardCtx<E>>,
    F: Context<WidgetID=<StandardCtx<E> as Context>::WidgetID> + BorrowMut<StandardCtx<E>>
{
    let senf = (*l).borrow_mut();
    let fns = senf.sup.fns::<F>();
    (fns.render)(l,r,f)
}
fn event<E: Context, F: Context<WidgetID=E::WidgetID> + BorrowMut<StandardCtx<E>>>(l: Link<F>, r: F::Event, f: fn(Link<F>,F::Event)) {
    f(l,r)
}
fn size<E: Context, F: Context<WidgetID=E::WidgetID> + BorrowMut<StandardCtx<E>>>(l: Link<F>, f: fn(Link<F>)-> Size) -> Size {
    f(l)
}