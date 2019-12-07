use crate::core::lazout::size::Size;
use crate::core::util::bounded_widget::*;
use crate::core::widget::handler::HandlerFns;
use crate::core::widget::link::Link;
use std::any::Any;
use crate::core::widget::Widget;
use crate::core::ctx::Context::*;
use crate::core::render::*;
use crate::core::event::Event;

pub mod imp;

pub trait Pane<E> where E: Context {
    type C: IBoundedWidget<E> + 'static;

    fn id(&self) -> E::WidgetID;

    fn childs(&self) -> &[Self::C];

    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn size(&self) -> Size;
    fn style(&self) -> &E::Style;

    fn parent(&self) -> Option<&E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
    
    
}

impl<E,T> Widget<E> for T where T: Pane<E> + 'static, E: Context + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        Pane::id(self)
    }
    #[inline]
    fn _handler(&self) -> HandlerFns<E> {
        HandlerFns{
            render: render::<T,E>,
            event: event::<T,E>,
            size: size::<T,E>,
        }
    }
    #[inline]
    fn invalid(&self) -> bool {
        Pane::invalid(self)
    }
    fn set_invalid(&mut self, v: bool) {
        Pane::set_invalid(self,v)
    }
    #[inline]
    fn parent(&self) -> Option<&E::WidgetID> {
        Pane::parent(self)
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        Pane::set_parent(self,v)
    }
    #[inline]
    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=BoundedWidget<E>> + 'a> {
        Box::new(
            Pane::childs(self)
            .iter()
            .map(IBoundedWidget::into_a)
        )
    }
    
    #[inline] fn as_any(&self) -> &dyn Any {self}
    #[inline] fn as_any_mut(&mut self) -> &mut dyn Any {self}
}

fn render<W: Pane<E> + 'static, E: Context + 'static>(mut l: Link<E>, mut r: E::Renderer) {
    for c in childs::<W,_>(&l) {
        l.widget(&c.id)
            .expect("Pane contains lost Widget")
            .handler()
            .render( &mut *l, r.slice(&c.bounds) );
    }
}

fn event<W: Pane<E> + 'static, E: Context + 'static>(mut l: Link<E>, e: E::Event) {
    //TODO special focus/hover enter/leave handling
    for c in childs::<W,_>(&l).into_iter().rev() {
        if let Some(e) = e.filter_cloned(&c.bounds) {
            let consuming = e.consuming();

            l.widget(&c.id)
                .expect("Pane contains lost Widget")
                .handler()
                .event( &mut *l, e );

            if consuming {return;}
        }
    }
}

fn size<W: Pane<E> + 'static, E: Context + 'static>(mut l: Link<E>) -> Size {
    unimplemented!()
}
#[inline]
fn childs<W: Pane<E> + 'static, E: Context + 'static>(l: &Link<E>) -> Vec<BoundedWidget<E>> {
    l.me::<W>().childs()
        .iter()
        .map(IBoundedWidget::into_a)
        .collect()
}