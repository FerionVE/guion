use crate::core::*;
use lazout::size::Size;
use util::bounded_widget::*;
use widget::handlez::WidgetFns;
use widget::link::Link;
use std::any::Any;
use widget::Widget;
use ctx::*;
use render::*;
use event::Event;

pub mod imp;

pub trait Pane<E> where E: Env {
    type C: IBoundedWidget<E> + 'static;

    fn id(&self) -> E::WidgetID;

    fn childs(&self) -> &[Self::C];

    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn size(&self) -> Size;
    fn style(&self) -> &EStyle<E>;

    fn parent(&self) -> Option<&E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
    
    
}

#[doc(hidden)]
impl<E,T> Widget<E> for T where T: Pane<E> + 'static, E: Env + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        Pane::id(self)
    }
    #[inline]
    fn _fns(&self) -> WidgetFns<E> {
        WidgetFns{
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
}

fn render<W: Pane<E> + 'static, E: Env + 'static>(mut l: Link<E>, mut r: ERenderer<E>) {
    for c in childs::<W,_>(&l) {
        l.widget(&c.id)
            .expect("Pane contains lost Widget")
            .handler()
            .render( &mut *l, r.slice(&c.bounds) );
    }
}

fn event<W: Pane<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: EEvent<E>) {
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

fn size<W: Pane<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size {
    todo!()
}
#[inline]
fn childs<W: Pane<E> + 'static, E: Env + 'static>(l: &Link<E>) -> Vec<BoundedWidget<E>> {
    l.me::<W>().childs()
        .iter()
        .map(IBoundedWidget::into_a)
        .collect()
}