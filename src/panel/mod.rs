use crate::widget::Widget;
use crate::widget::env::Env;

pub trait AbsolutePanel<E> where E: Env {
    type C: ChildEntry<E> + 'static;

    fn childs<'a,I: Iterator<Item=&'a Self::C> + 'static>(&'a self) -> I;

    fn commit(&self) -> E::Commit;
    fn commit_mut(&mut self) -> &mut E::Commit;
    fn parent(&self) -> Option<&E::WidgetID>;
}

pub trait ChildEntry<E> where E: Env {
    fn child(&self) -> E::WidgetID;
    fn bounds(&self) -> (u32,u32,u32,u32);
}

impl<E,T> Widget<E> for T where T: AbsolutePanel<E>, E: Env {
    type H = PanelWidgetHandler<E>;

    fn handler(&self) -> Self::H {
        PanelWidgetHandler
    }

    fn commit(&self) -> E::Commit {
        AbsolutePanel::commit(self)
    }
    fn commit_mut(&mut self) -> &mut E::Commit {
        AbsolutePanel::commit_mut(self)
    }

    fn parent(&self) -> Option<&E::WidgetID> {
        AbsolutePanel::parent(self)
    }

    fn childs(&self) -> Box<dyn Iterator<Item=((u32,u32,u32,u32),E::WidgetID)>> {
        AbsolutePanel::childs(self)
        .map(|c| (c.bounds(),c.child()) )
    }
}

pub struct PanelWidgetHandler<E>;

