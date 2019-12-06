use crate::core::lazout::size::Size;
use crate::core::widget::Widget;
use crate::core::ctx::Context;

pub mod fns;
pub mod imp;

pub use fns::*;
pub use imp::*;

pub struct Handler<E> where E: Context {
    pub(crate) id: E::WidgetID,
    pub(crate) fns: HandlerFns<E>,
}

impl<E> Handler<E> where E: Context {
    #[inline]
    pub fn render(&self, c: &mut E, r: E::Renderer) {
        (c.fns().render)(c.link(self.id),r,self.fns.render)
    }
    #[inline]
    pub fn event(&self, c: &mut E, e: E::Event) {
        (c.fns().event)(c.link(self.id),e,self.fns.event)
    }
    #[inline]
    pub fn size(&self, c: &mut E) -> Size {
        (c.fns().size)(c.link(self.id),self.fns.size)
    }
    #[inline]
    pub fn is_hovered(&self, c: &mut E) -> bool {
        c.hovered().map_or(false, |i| i == self.id )
    }
    #[inline]
    pub fn is_selected(&self, c: &mut E) -> bool {
        c.selected().map_or(false, |i| i == self.id )
    }
    /// iterate over childs
    #[inline]
    pub fn childs<'a>(&self, c: &'a E, predicate: impl FnMut(&E::WidgetID)->bool ) -> impl Iterator<Item=&'a E::DynWidget> {
        c.widget(&self.id).unwrap()
            .childs()
            .filter(predicate)
            .map(move |e| {
                (
                    c.widget(&e).expect("Lost Child")
                )
            })
    }
    /// iterate over childs mut
    #[inline]
    pub fn childs_mut<'a>(&self, c: &'a mut E, mut f: impl FnMut(&mut E::DynWidget), mut predicate: impl FnMut(&E::WidgetID)->bool) {
        let childs: Vec<E::WidgetID> = c.widget(&self.id).unwrap().childs().collect();

        for e in childs {
            if predicate(&e) {
                f(
                    c.widget_mut(&e).expect("Lost Child")
                );
            }
        }
    }
    /// iterate from current up to the root element
    #[inline]
    pub fn parents<'a>(&self, c: &'a E) -> Parents<'a,E> {
        Parents{
            ctx: c,
            next: Some(self.id.clone())
        }
    }
    /// iterate from current up to the root element mut
    #[inline]
    pub fn parents_mut<'a>(&self, c: &'a mut E, mut f: impl FnMut(&mut E::DynWidget) ) {
        let mut next = Some(self.id.clone());

        while let Some(n) = next {
            let r = c.widget_mut(&n).expect("Lost Parent");
            f(r);
            next = r.parent().cloned();
        }
    }
}