use crate::core::lazout::calc::calc_bounds;
use crate::core::util::bounds::Bounds;
use crate::core::lazout::size::Size;
use crate::core::util::bounded_widget::*;
use crate::core::widget::handler::HandlerFns;
use crate::core::widget::link::Link;
use std::any::Any;
use crate::core::widget::Widget;
use crate::core::env::*;
use crate::core::render::*;
use crate::core::event::Event;
use crate::core::lazout::Orientation;

pub mod imp;
//pub mod state;

pub trait Pane<E> where E: Env {

    fn id(&self) -> E::WidgetID;

    fn cached(&mut self) -> Option<&mut Option<Vec<Bounds>>> {
        None
    }

    fn childs(&self) -> &[E::WidgetID];

    fn orientation(&self) -> Orientation;
    fn set_orientation(&mut self, v: Orientation);

    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<&E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}

impl<E,T> Widget<E> for T where T: Pane<E> + 'static, E: Env + 'static {
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
    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=E::WidgetID> + 'a> {
        Box::new(
            Pane::childs(self)
            .iter()
            .cloned()
        )
    }
    
    #[inline] fn as_any(&self) -> &dyn Any {self}
    #[inline] fn as_any_mut(&mut self) -> &mut dyn Any {self}
}

fn render<W: Pane<E> + 'static, E: Env + 'static>(mut l: Link<E>, mut r: E::Renderer) {
    let o = l.me::<W>().orientation();

    let c = childs::<W,E>(&l);

    let b = c.iter()
        .map(|c| 
            l.widget(c)
            .expect("Lost Widget")
            .handler()
            .size(&mut l)
        )
        .collect::<Vec<_>>();

    let b = calc_bounds(r.bounds_abs().size, &b[..], o);

    for (cc,bb) in c.iter().zip(b.iter()) {
        l.widget(cc)
            .expect("Pane contains lost Widget")
            .handler()
            .render( &mut *l, r.slice(bb) );
    }

}

fn event<W: Pane<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: E::Event) {
    unimplemented!()
}

fn size<W: Pane<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size {
    let o = l.me::<W>().orientation();

    let mut s = Size::empty();
    
    for c in childs::<W,E>(&l) {
        let cs = l.widget(&c)
            .expect("Lost Widget")
            .handler()
            .size(&mut l);
        
        s.add(&cs, o)
    }

    s
}
#[inline]
fn childs<W: Pane<E> + 'static, E: Env + 'static>(l: &Link<E>) -> Vec<E::WidgetID> {
    l.me::<W>().childs().to_owned()
}