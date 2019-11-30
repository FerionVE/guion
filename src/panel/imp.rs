use crate::panel::ChildEntry;
use crate::widget::env::Env;

pub struct Pane<E> where E: Env {
    id: E::WidgetID,
    childs: Vec<PaneEntry<E>>,
    commit: E::Commit,
    parent: Option<E::WidgetID>,
}
#[derive(Clone)]
pub struct PaneEntry<E> where E: Env {
    pub bounds: (u32,u32,u32,u32),
    pub id: E::WidgetID,
}

impl<E> super::Pane<E> for Pane<E> where E: Env + 'static {
    type C = PaneEntry<E>;

    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn childs(&self) -> &[Self::C] {
        &self.childs[..]
    }

    fn commit(&self) -> &E::Commit {
        &self.commit
    }
    fn commit_mut(&mut self) -> &mut E::Commit {
        &mut self.commit
    }

    fn parent(&self) -> Option<&E::WidgetID> {
        self.parent.as_ref()
    }

    fn parent_mut(&mut self) -> &mut Option<E::WidgetID> {
        &mut self.parent
    }
}

impl<E> PaneEntry<E> where E: Env {
    pub fn from<C: ChildEntry<E>>(e: &C) -> Self {
        Self{
            id: e.child(),
            bounds: e.bounds(),
        }
    }
}

impl<E> ChildEntry<E> for PaneEntry<E> where E: Env {
    fn child(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn bounds(&self) -> (u32,u32,u32,u32) {
        self.bounds        
    }
}