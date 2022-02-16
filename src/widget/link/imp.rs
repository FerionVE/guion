use super::*;

/*pub struct Parents<'a,E> where E: Env {
    pub(super) stor: &'a E::Storage<'a>,
    pub(super) next: Option<E::WidgetPath>,
}

impl<'a,E> Iterator for Parents<'a,E> where E: Env {
    type Item = Resolved<'a,E>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.next.take() {
            let r = self.stor.widget(n.refc()).expect("Lost Parent");
            self.next = n.parent();
            Some(r)
        }else{
            None
        }
    }
}*/
