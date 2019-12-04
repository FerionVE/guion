use crate::core::lazout::size::Size;
use crate::core::widget::Widget;
use crate::core::env::Env;
use crate::core::env::Context;

pub mod fns;
pub mod imp;

pub use fns::*;
pub use imp::*;

pub struct Handler<E> where E: Env {
    pub(crate) id: E::WidgetID,
    pub(crate) fns: HandlerFns<E>,
}

impl<E> Handler<E> where E: Env {
    #[inline]
    pub fn render(&self, c: &mut E::Ctx, r: E::Renderer) {
        c.render_widget(r,&self.id,self.fns.render)
    }
    #[inline]
    pub fn event(&self, c: &mut E::Ctx, e: E::Event) {
        c.event_widget(e,&self.id,self.fns.event)
    }
    #[inline]
    pub fn size(&self, c: &mut E::Ctx) -> Size {
        c.size_widget(&self.id,self.fns.size)
    }
    /// iterate over childs
    #[inline]
    pub fn childs<'a>(&self, c: &'a E::Ctx, predicate: impl FnMut(&E::WidgetID)->bool ) -> impl Iterator<Item=&'a E::DynWidget> {
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
    pub fn childs_mut<'a>(&self, c: &'a mut E::Ctx, mut f: impl FnMut(&mut E::DynWidget), mut predicate: impl FnMut(&E::WidgetID)->bool) {
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
    pub fn parents<'a>(&self, c: &'a E::Ctx) -> Parents<'a,E> {
        Parents{
            ctx: c,
            next: Some(self.id.clone())
        }
    }
    /// iterate from current up to the root element mut
    #[inline]
    pub fn parents_mut<'a>(&self, c: &'a mut E::Ctx, mut f: impl FnMut(&mut E::DynWidget) ) {
        let mut next = Some(self.id.clone());

        while let Some(n) = next {
            let r = c.widget_mut(&n).expect("Lost Parent");
            f(r);
            next = r.parent().cloned();
        }
    }
}