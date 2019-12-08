use crate::core::widget::link::Link;
use crate::core::lazout::size::Size;
use crate::core::widget::Widget;
use crate::core::ctx::*;

pub mod fns;
pub mod imp;

pub use fns::*;
pub use imp::*;

pub struct Handler<'a,E> where E: Context {
    pub(crate) id: E::WidgetID,
    pub(crate) ctx: &'a mut E,
}

impl<'a,E> Handler<'a,E> where E: Context {
    #[deprecated]
    #[inline]
    pub fn render(&mut self, r: E::Renderer) { //TODO fix &mut Renderer back to owned
        self.id._render(self.ctx,r)
    }
    #[deprecated]
    #[inline]
    pub fn event(&mut self, e: E::Event) {
        self.id._event(self.ctx,e)
    }
    #[deprecated]
    #[inline]
    pub fn size(&mut self) -> Size {
        self.id._size(self.ctx)
    }
    #[deprecated]
    #[inline]
    pub fn link(&'a mut self) -> Link<'a,E> {
        Link{
            widget_id: self.id.clone(),
            ctx: self.ctx,
        }
    }
    #[inline]
    pub fn is_hovered(&self) -> bool {
        self.ctx.hovered().map_or(false, |i| i == self.id )
    }
    #[inline]
    pub fn is_selected(&self) -> bool {
        self.ctx.selected().map_or(false, |i| i == self.id )
    }
    /// iterate over childs
    #[inline]
    pub fn childs(&'a self, predicate: impl FnMut(&E::WidgetID)->bool ) -> impl Iterator<Item=&'a E::DynWidget> {
        self.ctx.widget(&self.id).unwrap()
            .childs()
            .filter(predicate)
            .map(move |e| {
                (
                    self.ctx.widget(&e).expect("Lost Child")
                )
            })
    }
    /// iterate over childs mut
    #[inline]
    pub fn childs_mut(&'a mut self, mut f: impl FnMut(&mut E::DynWidget), mut predicate: impl FnMut(&E::WidgetID)->bool) {
        let childs: Vec<E::WidgetID> = self.ctx.widget(&self.id).unwrap().childs().collect();

        for e in childs {
            if predicate(&e) {
                f(
                    self.ctx.widget_mut(&e).expect("Lost Child")
                );
            }
        }
    }
    /// iterate from current up to the root element
    #[inline]
    pub fn parents(&'a self) -> Parents<'a,E> {
        Parents{
            ctx: self.ctx,
            next: Some(self.id.clone())
        }
    }
    /// iterate from current up to the root element mut
    #[inline]
    pub fn parents_mut(&'a mut self, mut f: impl FnMut(&mut E::DynWidget) ) {
        let mut next = Some(self.id.clone());

        while let Some(n) = next {
            let r = self.ctx.widget_mut(&n).expect("Lost Parent");
            f(r);
            next = r.parent();
        }
    }
}