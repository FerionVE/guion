use crate::core::ctx::widgets::Widgets;
use super::*;

pub struct Parents<'a,E> where E: Env {
    pub(super) stor: &'a E::Storage,
    pub(super) next: Option<WPSlice<'a,E>>,
}

impl<'a,E> Iterator for Parents<'a,E> where E: Env {
    type Item = Resolved<'a,E>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.next {
            let r = self.stor.widget(n).expect("Lost Parent");
            self.next = n.parent();
            Some(r)
        }else{
            None
        }
    }
}