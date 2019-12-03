use crate::core::lazout::size::Size;
use crate::core::util::bounded_widget::*;
use crate::core::widget::handler::HandlerFns;
use crate::core::widget::link::Link;
use std::any::Any;
use crate::core::widget::Widget;
use crate::core::env::*;
use crate::core::render::*;
use crate::core::event::Event;

pub mod imp;

pub trait Pane<E> where E: Env {
    type C: IBoundedWidget<E> + 'static;

    fn id(&self) -> E::WidgetID;

    fn childs(&self) -> &[Self::C];

    fn render_invalid(&self) -> bool;
    fn set_render_invalid(&mut self, v: bool);

    fn layout_invalid(&self) -> bool;
    fn set_layout_invalid(&mut self, v: bool);

    fn size(&self) -> Size;

    fn parent(&self) -> Option<&E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
    
    
}

impl<E,T> Widget<E> for T where T: Pane<E> + 'static, E: Env + 'static {
    fn id(&self) -> E::WidgetID {
        Pane::id(self)
    }

    fn _handler(&self) -> HandlerFns<E> {
        HandlerFns{
            render: render::<T,E>,
            event: event::<T,E>,
        }
    }

    fn render_invalid(&self) -> bool {
        Pane::render_invalid(self)
    }
    fn set_render_invalid(&mut self, v: bool) {
        Pane::set_render_invalid(self,v)
    }

    fn layout_invalid(&self) -> bool {
        Pane::layout_invalid(self)
    }
    fn set_layout_invalid(&mut self, v: bool) {
        Pane::set_layout_invalid(self,v)
    }

    fn size(&self) -> Size {
        Pane::size(self)
    }

    fn parent(&self) -> Option<&E::WidgetID> {
        Pane::parent(self)
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        Pane::set_parent(self,v)
    }

    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=BoundedWidget<E>> + 'a> {
        Box::new(
            Pane::childs(self)
            .iter()
            .map(IBoundedWidget::into_a)
        )
    }

    fn as_any(&self) -> &dyn Any {self}
    fn as_any_mut(&mut self) -> &mut dyn Any {self}
}

fn render<W: Pane<E> + 'static, E: Env + 'static>(mut l: Link<E>, mut r: E::Renderer) {
    for c in childs::<W,_>(&l) {
        l.widget(&c.id)
            .expect("Pane contains lost Widget")
            .handler()
            .render( &mut *l, r.slice(&c.bounds) );
    }
}

fn event<W: Pane<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: E::Event) {
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

fn childs<W: Pane<E> + 'static, E: Env + 'static>(l: &Link<E>) -> Vec<BoundedWidget<E>> {
    l.me::<W>().childs()
        .iter()
        .map(IBoundedWidget::into_a)
        .collect()
}