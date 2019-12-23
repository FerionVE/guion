use crate::core::*;
use ctx::*;
use widget::*;

pub struct Parents<'a,E> where E: Env {
    pub(super) ctx: &'a E::Context,
    pub(super) next: Option<E::WidgetID>,
}

impl<'a,E> Iterator for Parents<'a,E> where E: Env {
    type Item = &'a E::DynWidget;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = &self.next {
            let r = self.ctx.widget(n).expect("Lost Parent");
            self.next = r.parent();
            Some(r)
        }else{
            None
        }
    }
}