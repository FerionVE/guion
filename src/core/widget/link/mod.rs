use crate::core::ctx::widgets::Widgets;
use std::ops::DerefMut;
use std::ops::Deref;
use super::*;

pub mod imp;
use imp::*;

pub struct Link<'c,'p,E> where E: Env, for<'e> &'e E: EnvLt<'e> {
    pub ctx: &'c mut E::Context,
    pub path: EWPSlice<'p,E>,
    // absolute pos ans size of current widget
    //pub bounds: Bounds,
}

impl<'c,'p,E> Link<'c,'p,E> where E: Env, for<'e> &'e E: EnvLt<'e> {
    #[inline]
    pub fn me<S: Widget<E> + 'static>(&self) -> &S {
        self.widget()
            .downcast_ref::<S>().expect("Link: Wrong Widget Type")
    }
    #[inline] 
    pub fn me_mut<S: Widget<E> + 'static>(&mut self) -> &mut S {
        self.widget_mut()
            .downcast_mut::<S>().expect("Link: Wrong Widget Type")
    }

    #[inline]
    pub fn widget(&self) -> &E::DynWidget {
        self.ctx.widget(&self.path)
            .expect("Link: Widget Gone")
    }
    #[inline] 
    pub fn widget_mut(&mut self) -> &mut E::DynWidget {
        self.ctx.widget_mut(&self.path)
            .expect("Link: Widget Gone")
    }

    #[inline]
    pub fn widget_fns(&self) -> WidgetFns<E> {
        self.ctx.widget_fns(&self.path)
    }

    /*#[inline]
    pub fn render(&mut self, r: (&mut ERenderer<E>,&Bounds)) { //TODO fix &mut Renderer back to owned
        self.id._render::<E>(self.ctx,r)
    }
    #[inline]
    pub fn event(&mut self, e: (EEvent<E>,&Bounds)) {
        self.id._event::<E>(self.ctx,e)
    }
    #[inline]
    pub fn size(&mut self) -> Size {
        self.id._size::<E>(self.ctx)
    }*/

    #[inline]
    pub fn is_hovered(&self) -> bool where ECHandler<E>: AsHandlerStateful<E> {
        self.ctx.state().is_hovered(self.path.id())
    }
    #[inline]
    pub fn is_selected(&self) -> bool where ECHandler<E>: AsHandlerStateful<E> {
        self.ctx.state().is_selected(self.path.id())
    }

    /// iterate over childs
    #[inline]
    pub fn childs(&'a self, predicate: impl FnMut(&E::WidgetID)->bool ) -> impl Iterator<Item=&'a E::DynWidget> {
        self.ctx.widget(&self.path).unwrap()
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
        let childs: Vec<E::WidgetID> = self.ctx.widget(&self.path).unwrap().childs().collect();

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
            next: Some(self.path.slice()),
        }
    }
    /// iterate from current up to the root element mut
    #[inline]
    pub fn parents_mut(&'a mut self, mut f: impl FnMut(&mut E::DynWidget) ) {
        let mut next = Some(self.path.slice());

        while let Some(n) = next {
            let r = self.ctx.widget_mut(&n).expect("Lost Parent");
            f(r);
            next = r.parent();
        }
    }

    pub fn with_ctx<'b,F: Env<WidgetID=E::WidgetID>>(self, ctx: &'b mut F::Context) -> Link<'b,F> {
        Link{
            ctx,
            id: self.id,
            //bounds: self.bounds,
        }
    }
    #[inline]
    pub fn enqueue<Q: Queue<E>>(&'a mut self, args: Q::Args, f: Q::Callback) -> Q::Return where E::Context: AccessQueue<Q,E> {
        self.ctx.queue_mut().add(args,f)
    }
}

impl<'a,E> Deref for Link<'a,E> where E: Env {
    type Target = E::Context;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}
impl<'a,E> DerefMut for Link<'a,E> where E: Env {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

