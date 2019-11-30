use std::any::Any;
use crate::panel::imp::PaneEntry;
use crate::widget::Widget;
use crate::widget::env::*;
use crate::render::Render;

pub mod imp;

pub trait Pane<E> where E: Env {
    type C: ChildEntry<E> + 'static;

    fn childs(&self) -> &[Self::C];

    fn commit(&self) -> &E::Commit;
    fn commit_mut(&mut self) -> &mut E::Commit;
    fn parent(&self) -> Option<&E::WidgetID>;
    fn parent_mut(&mut self) -> &mut Option<E::WidgetID>;
}

pub trait ChildEntry<E>: Clone where E: Env {
    fn child(&self) -> E::WidgetID;
    fn bounds(&self) -> (u32,u32,u32,u32);
}

impl<E,T> Widget<E> for T where T: Pane<E> + 'static, E: Env + 'static {
    fn render(&self) -> fn(&mut E::Ctx, &E::WidgetID, E::Renderer) {
        render::<T,E>
    }

    fn event(&self) -> fn(&mut E::Ctx, &E::WidgetID, E::Event) {
        event::<T,E>
    }

    fn commit(&self) -> &E::Commit {
        Pane::commit(self)
    }
    fn commit_mut(&mut self) -> &mut E::Commit {
        Pane::commit_mut(self)
    }

    fn parent(&self) -> Option<&E::WidgetID> {
        Pane::parent(self)
    }

    fn parent_mut(&mut self) -> &mut Option<E::WidgetID> {
        Pane::parent_mut(self)
    }

    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=((u32,u32,u32,u32),E::WidgetID)> + 'a> {
        Box::new(
            Pane::childs(self)
            .iter()
            .map(|c| (c.bounds(),c.child()) )
        )
    }

    fn _as_any(&self) -> &dyn Any {self}
    fn _as_any_mut(&mut self) -> &mut dyn Any {self}
}

fn render<W: Pane<E> + 'static, E: Env + 'static>(cx: &mut E::Ctx, me: &E::WidgetID, mut r: E::Renderer) {
    for c in childs::<W,_>(cx, me) {
        let h = cx.widgets().get(&c.id)
        .expect("Pane contains lost Widget")
        .render();

        h(cx, &c.id, r.slice(c.bounds) );
    }
}

fn event<W: Pane<E> + 'static, E: Env + 'static>(cx: &mut E::Ctx, me: &E::WidgetID, mut r: E::Event) {
    unimplemented!()
}

fn childs<W: Pane<E> + 'static, E: Env + 'static>(cx: &E::Ctx, me: &E::WidgetID) -> Vec<PaneEntry<E>> {
    cx.me::<W>(me).unwrap().childs()
        .iter()
        .map(|e| PaneEntry::from(e) )
        .collect()
}