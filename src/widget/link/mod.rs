//! The Link is used to interface widgets thru and tracks the current path
use std::ops::DerefMut;
use std::ops::Deref;
use super::*;

#[doc(hidden)]
pub mod imp;
use imp::*;

/// holds a immutable reference to the current widget and the widget tree, also a mutable reference to the context
pub struct Link<'c,E> where E: Env {
    pub ctx: &'c mut E::Context,
    pub widget: Resolved<'c,E>,
}

impl<'c,E> Link<'c,E> where E: Env {
    /// enqueue mutable access to this widget
    #[inline] 
    pub fn mutate(&mut self, f: fn(WidgetRefMut<E>,&mut E::Context,E::WidgetPath), invalidate: bool) {
        self.enqueue(StdEnqueueable::MutateWidget{path: self.widget.path.refc(),f,invalidate})
    }
    /// enqueue mutable access to this widget
    #[inline] 
    pub fn mutate_closure(&mut self, f: Box<dyn FnOnce(WidgetRefMut<E>,&mut E::Context,E::WidgetPath)+'static>, invalidate: bool) {
        self.enqueue(StdEnqueueable::MutateWidgetClosure{path: self.widget.path.refc(),f,invalidate})
    }
    /// enqueue immutable access to this widget
    #[inline] 
    pub fn later(&mut self, f: fn(WidgetRef<E>,&mut E::Context)) {
        self.enqueue(StdEnqueueable::AccessWidget{path: self.widget.path.refc(),f})
    }
    /// enqueue immutable access to this widget
    #[inline] 
    pub fn later_closure(&mut self, f: Box<dyn FnOnce(WidgetRef<E>,&mut E::Context)+Sync>) {
        self.enqueue(StdEnqueueable::AccessWidgetClosure{path: self.widget.path.refc(),f})
    }
    #[inline]
    pub fn enqueue_invalidate(&mut self) {
        self.enqueue(StdEnqueueable::InvalidateWidget{path: self.widget.path.refc()})
    }
    /// mark the current widget as validated
    /// this should and should only be called from widget's render fn
    #[inline]
    pub fn enqueue_validate_render(&mut self) {
        self.enqueue(StdEnqueueable::ValidateWidgetRender{path: self.widget.path.refc()})
    }
    #[inline]
    pub fn enqueue_validate_size(&mut self, s: ESize<E>) {
        self.enqueue(StdEnqueueable::ValidateWidgetSize{path: self.widget.path.refc(),size: s})
    }
    #[inline]
    pub fn widget(&self) -> &dyn Widget<'c,E> {
        &**self.widget.widget()
    }

    #[inline]
    pub fn id(&self) -> E::WidgetID {
        self.widget().id()
    }

    pub fn path(&self) -> E::WidgetPath {
        self.widget.path.refc()
    }

    /*#[inline]
    pub fn for_child<'s>(&'s self, child: &'s dyn Widget<E>) -> Link<'s> where 'c: 's {
        
    }*/

    #[inline]
    pub fn render(&mut self, r: &mut RenderLink<E>) -> bool {
        self.ctx.render(self.widget.clone(),r)
    }
    #[inline]
    pub fn event(&mut self, e: (EEvent<E>,&Bounds,u64)) {
        self.ctx.event(self.widget.clone(),e)
    }
    #[inline]
    pub fn size(&mut self) -> ESize<E> {
        self.ctx.size(self.widget.clone())
    }
    #[inline]
    pub fn _event_root(&mut self, e: (EEvent<E>,&Bounds,u64)) {
        self.ctx._event_root(self.widget.clone(),e)
    }
    /// bypasses Context and Handler(s)
    #[inline]
    pub fn _render(&mut self, r: &mut RenderLink<E>) -> bool {
        let w = self.ctx.link(self.widget.clone());
        self.widget.widget()._render(w,r)
    }
    /// bypasses Context and Handler(s)
    #[inline]
    pub fn _event(&mut self, e: (EEvent<E>,&Bounds,u64)) {
        let w = self.ctx.link(self.widget.clone());
        self.widget.widget()._event(w,e)
    }
    /// bypasses Context and Handler(s)
    #[inline]
    pub fn _size(&mut self) -> ESize<E> {
        let w = self.ctx.link(self.widget.clone());
        self.widget.widget()._size(w)
    }

    pub fn trace_bounds(&mut self, root_bounds: &Bounds, force: bool) -> Bounds {
        self.widget.stor.trace_bounds(self.ctx,self.widget.path.refc(),root_bounds,force).unwrap()
    }

    #[inline]
    pub fn is_hovered(&self) -> bool where E::Context: AsHandlerStateful<E> {
        self.ctx.state().is_hovered(&self.id())
    }
    #[inline]
    pub fn is_focused(&self) -> bool where E::Context: AsHandlerStateful<E> {
        self.ctx.state().is_focused(&self.id())
    }

    #[deprecated]
    #[allow(deprecated)]
    #[inline]
    pub fn child_paths(&self) -> Vec<E::WidgetPath> {
        self.widget.child_paths()
    }

    //THIS IS AN ULTRA HACK
    //(as shortening teh lifetime even more aggresse we MAY can put an iterator on it)
    #[inline]
    pub fn for_childs<'s>(&'s mut self, mut f: impl FnMut(Link<E>)) -> Result<(),()> where 'c: 's {
        //let wref = self.widget.wref.refc();
        let path = self.widget.path.refc();
        let ch = self.widget.widget().childs_ref();
        let stor = self.widget.stor;
        for c in ch {
            let w = c.resolve_widget(stor)?;
            let w = Resolved{
                path: w.self_in_parent(path.refc()).into(),
                wref: w,
                stor,
            };
            Self::_with_link(self.ctx,w,&mut f);
        }
        Ok(())
    }
    #[inline]
    pub fn for_child<'s>(&'s mut self, i: usize) -> Result<Link<E>,()> where 'c: 's {
        let path = self.widget.path.refc();
        let stor = self.widget.stor;

        let c = self.widget.widget().child(i)?;

        let w = c.resolve_widget(stor)?;
        let w = Resolved{
            path: w.self_in_parent(path.refc()).into(),
            wref: w,
            stor,
        };
        let l = Link{
            widget: w.short_lt(),
            ctx: self.ctx,
        };

        Ok(l)
    }
    #[inline]
    pub fn _with_link<'s>(ctx: &mut E::Context, w: Resolved<'s,E>, f: impl FnOnce(Link<E>)) where 'c: 's {
        let l = Link{
            widget: w.short_lt(),
            ctx,
        };
        f(l);
    }

    pub fn child_sizes(&mut self) -> Result<Vec<ESize<E>>,()> {
        let mut dest = Vec::with_capacity(self.widget().childs());
        self.for_childs(#[inline] |mut w| dest.push(w.size()) )?;
        Ok(dest)
    }

    pub fn with_widget<'s>(&'s mut self, p: E::WidgetPath) -> Result<Link<'s,E>,()> where 'c: 's {
        Ok(
            Link{
                widget: self.widget.stor.widget(p)?,
                ctx: self.ctx
            }
        )
    }

    pub fn resolve_sub<'s>(&'s mut self, p: E::WidgetPath) -> Result<Link<'s,E>,()> where 'c: 's {
        let mut new_path = self.widget.path.clone().attached_subpath(&p);
        let rw = self.widget.wref.resolve(p.refc())?;
        rw.extract_path(&mut new_path);
        let rw = rw.resolve_widget(&self.widget.stor)?;
        let w = Resolved{
            path: new_path,
            wref: rw,
            stor: self.widget.stor,
        };
        Ok(
            Link{
                widget: w,
                ctx: self.ctx,
            }
        )
    }

    /*pub fn with_resolvable<'l,'s>(&'s mut self, r: Resolvable<'w,E>) -> Result<Link<'s,E>,()> where 'l: 's {
        r.extract_path(&mut new_path);
        let rw = r.resolve_widget(&self.widget.stor)?;
        let w = Resolved{
            path: new_path,
            wref: rw,
            stor: self.widget.stor,
        };
        Ok(
            Link{
                widget: w,
                ctx: self.ctx,
            }
        )
    }*/

    pub fn reference<'s>(&'s mut self) -> Link<'s,E> where 'c: 's {
        Link{
            widget: self.widget.reference(),
            ctx: self.ctx
        }
    }

    /*
    /// iterate over childs
    #[inline]
    pub fn childs(&'c self, predicate: impl Fn(WPSlice<'c,E>)->bool + 'c ) -> impl Iterator<Item=&'c dyn Widget<E>> + 'c {
        self.ctx.widget(self.path).unwrap()
            .child_paths(self.path)
            .into_iter()
            .filter(#[inline] move |s| predicate(s) )
            .map(move |e| {
                (
                    self.ctx.widget(e).expect("Lost Child")
                )
            })
    }
    /// iterate over childs mut
    #[inline]
    pub fn childs_mut(&'c mut self, mut f: impl FnMut(&mut dyn WidgetMut<E>), mut predicate: impl FnMut(&E::WidgetPath)->bool) {
        let childs: Vec<E::WidgetPath> = self.ctx.widget(self.path).unwrap().child_paths(self.path);

        for e in childs {
            if predicate(&e) {
                f(
                    self.ctx.widget_mut(e).expect("Lost Child")
                );
            }
        }
    }*/
    /// iterate from current up to the root element
    #[inline]
    pub fn parents(&'c self) -> Parents<'c,E> {
        Parents{
            stor: self.widget.stor,
            next: Some(self.widget.path.refc()),
        }
    }
    
    pub fn with_ctx<F: Env<WidgetPath=E::WidgetPath,Storage=E::Storage>>(self, ctx: &'c mut F::Context) -> Link<'c,F> where E::WidgetPath: WidgetPath<F,SubPath=EWPSub<E>>, EWPSub<E>: SubPath<F>, E::Storage: Widgets<F> {
        Link{
            widget: self.widget.with_env::<F>(),
            ctx,
        }
    }
    #[inline]
    pub fn enqueue<I>(&mut self, i: I) where ECQueue<E>: Queue<I> {
        self.ctx.queue_mut().push(i)
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