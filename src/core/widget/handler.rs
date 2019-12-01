use crate::core::widget::Widget;
use crate::core::util::bounds::Bounds;
use crate::core::util::bounded_widget::BoundedWidget;
use crate::core::widget::link::Link;
use crate::core::env::Env;
use crate::core::env::Context;
use crate::core::env::WidgetStore;

pub struct Handler<E> where E: Env {
    pub(crate) own_id: E::WidgetID,
    pub(crate) fns: HandlerFns<E>,
}

pub struct HandlerFns<E> where E: Env {
    pub render: fn(Link<E>, E::Renderer),
    pub event: fn(Link<E>, E::Event),
}

impl<E> Handler<E> where E: Env {
    pub fn render(&self, c: &mut E::Ctx, r: E::Renderer) {
        (self.fns.render)(self.link(c),r)
    }

    pub fn event(&self, c: &mut E::Ctx, e: E::Event) {
        (self.fns.event)(self.link(c),e)
    }
    /// iterate over childs
    #[inline]
    pub fn childs<'a>(&self, c: &'a E::Ctx, predicate: impl FnMut(&BoundedWidget<E>)->bool ) -> impl Iterator<Item=(Bounds,&'a E::DynWidget)> {
        c.widgets().get(&self.own_id).unwrap()
            .childs()
            .filter(predicate)
            .map(move |e| {
                (
                    e.bounds,
                    c.widgets().get(&e.id).expect("Lost Child")
                )
            })
    }
    /// iterate over childs mut
    #[inline]
    pub fn childs_mut<'a>(&self, c: &'a mut E::Ctx, mut f: impl FnMut(Bounds,&mut E::DynWidget), mut predicate: impl FnMut(&BoundedWidget<E>)->bool) {
        let childs: Vec<BoundedWidget<E>> = c.widgets().get(&self.own_id).unwrap().childs().collect();

        for e in childs {
            if predicate(&e) {
                f(
                    e.bounds,
                    c.widgets_mut().get_mut(&e.id).expect("Lost Child")
                );
            }
        }
    }
    /// iterate from current up to the root element
    #[inline]
    pub fn parents<'a>(&self, c: &'a E::Ctx) -> Parents<'a,E> {
        Parents{
            ctx: c,
            next: Some(self.own_id.clone())
        }
    }
    /// iterate from current up to the root element mut
    #[inline]
    pub fn parents_mut<'a>(&self, c: &'a mut E::Ctx, mut f: impl FnMut(&mut E::DynWidget) ) {
        let mut next = Some(self.own_id.clone());

        while let Some(n) = next {
            let r = c.widgets_mut().get_mut(&n).expect("Lost Parent");
            f(r);
            next = r.parent().cloned();
        }
    }

    fn link<'a>(&self, c: &'a mut E::Ctx) -> Link<'a,E> {
        Link{
            ctx: c,
            widget_id: self.own_id.clone()
        }
    }
}

pub struct Parents<'a,E> where E: Env {
    ctx: &'a E::Ctx,
    next: Option<E::WidgetID>,
}

impl<'a,E> Iterator for Parents<'a,E> where E: Env {
    type Item = &'a E::DynWidget;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = &self.next {
            let r = self.ctx.widgets().get(n).expect("Lost Parent");
            self.next = r.parent().cloned();
            Some(r)
        }else{
            None
        }
    }
}