use crate::core::ctx::Context;
use crate::core::widget::*;

pub struct Parents<'a,E> where E: Context {
    pub(super) ctx: &'a E,
    pub(super) next: Option<E::WidgetID>,
}

impl<'a,E> Iterator for Parents<'a,E> where E: Context {
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