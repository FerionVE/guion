use crate::core::env::Env;
use crate::core::env::*;
use crate::core::widget::*;

pub struct Parents<'a,E> where E: Env {
    pub(super) ctx: &'a E::Ctx,
    pub(super) next: Option<E::WidgetID>,
}

impl<'a,E> Iterator for Parents<'a,E> where E: Env {
    type Item = &'a E::DynWidget;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = &self.next {
            let r = self.ctx.widget(n).expect("Lost Parent");
            self.next = r.parent().cloned();
            Some(r)
        }else{
            None
        }
    }
}