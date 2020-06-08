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
    pub fn mutate(&mut self, f: fn(WidgetRefMut<E>,&mut E::Context,E::WidgetPath)) {
        self.mutate_at(f,StdOrder::PostCurrent,0)
    }
    #[inline] 
    pub fn mutate_at<O>(&mut self, f: fn(WidgetRefMut<E>,&mut E::Context,E::WidgetPath), o: O, p: i64) where ECQueue<E>: Queue<StdEnqueueable<E>,O> {
        self.enqueue(StdEnqueueable::MutateWidget{path: self.widget.path.refc(),f},o,p)
    }
    /// enqueue mutable access to this widget
    #[inline] 
    pub fn mutate_closure(&mut self, f: Box<dyn FnOnce(WidgetRefMut<E>,&mut E::Context,E::WidgetPath)+'static>) {
        self.mutate_closure_at(f,StdOrder::PostCurrent,0)
    }
    #[inline] 
    pub fn mutate_closure_at<O>(&mut self, f: Box<dyn FnOnce(WidgetRefMut<E>,&mut E::Context,E::WidgetPath)+'static>, o: O, p: i64) where ECQueue<E>: Queue<StdEnqueueable<E>,O> {
        self.enqueue(StdEnqueueable::MutateWidgetClosure{path: self.widget.path.refc(),f},o,p)
    }
    /// enqueue immutable access to this widget
    #[inline] 
    pub fn later(&mut self, f: fn(WidgetRef<E>,&mut E::Context)) {
        self.later_at(f,StdOrder::PostCurrent,0)
    }
    #[inline] 
    pub fn later_at<O>(&mut self, f: fn(WidgetRef<E>,&mut E::Context), o: O, p: i64) where ECQueue<E>: Queue<StdEnqueueable<E>,O> {
        self.enqueue(StdEnqueueable::AccessWidget{path: self.widget.path.refc(),f},o,p)
    }
    /// enqueue immutable access to this widget
    #[inline] 
    pub fn later_closure(&mut self, f: Box<dyn FnOnce(WidgetRef<E>,&mut E::Context)+Sync>) {
        self.later_closure_at(f,StdOrder::PostCurrent,0)
    }
    #[inline] 
    pub fn later_closure_at<O>(&mut self, f: Box<dyn FnOnce(WidgetRef<E>,&mut E::Context)+Sync>, o: O, p: i64) where ECQueue<E>: Queue<StdEnqueueable<E>,O> {
        self.enqueue(StdEnqueueable::AccessWidgetClosure{path: self.widget.path.refc(),f},o,p)
    }
    #[inline]
    pub fn enqueue_invalidate(&mut self) {
        self.enqueue(StdEnqueueable::InvalidateWidget{path: self.widget.path.refc()},StdOrder::PreRender,0)
    }
    /// mark the current widget as validated
    /// this should and should only be called from widget's render fn
    #[inline]
    pub fn enqueue_validate_render(&mut self) {
        self.enqueue(StdEnqueueable::ValidateWidgetRender{path: self.widget.path.refc()},StdOrder::RenderValidation,0)
    }
    #[inline]
    pub fn enqueue_validate_size(&mut self, s: ESize<E>) {
        self.enqueue(StdEnqueueable::ValidateWidgetSize{path: self.widget.path.refc(),size: s},StdOrder::RenderValidation,0)
    }

    #[inline]
    pub fn id(&self) -> E::WidgetID {
        self.widget.id()
    }

    pub fn path(&self) -> E::WidgetPath {
        self.widget.path.refc()
    }

    pub fn ident(&self) -> WidgetIdent<E> {
        self.widget.ident()
    }

    #[inline]
    pub fn render(&mut self, r: &mut RenderLink<E>) {
        self.ctx.render(self.widget.reference(),r)
    }
    #[deprecated]
    #[inline]
    pub fn event(&mut self, e: (EEvent<E>,&Bounds,u64)) -> bool {
        self.ctx.event(self.widget.reference(),e)
    }
    #[inline]
    pub fn route_event(&mut self, e: (EEvent<E>,&Bounds,u64), child: E::WidgetPath) -> Result<bool,()> {
        self.ctx.route_event(self.widget.reference(),e,child)
    }
    #[inline]
    pub fn size(&mut self) -> ESize<E> {
        self.ctx.size(self.widget.reference())
    }
    #[inline]
    pub fn _event_root(&mut self, e: (EEvent<E>,&Bounds,u64)) {
        self.ctx._event_root(self.widget.reference(),e)
    }
    /// bypasses Context and Handler(s)
    #[inline]
    pub fn _render(&mut self, r: &mut RenderLink<E>) {
        let w = self.ctx.link(self.widget.reference());
        (**self.widget)._render(w,r)
    }
    /// bypasses Context and Handler(s)
    #[inline]
    pub fn _event(&mut self, e: (EEvent<E>,&Bounds,u64)) -> bool {
        let mut b = *self.ctx.default_border();
        self.widget.border(&mut b);
        let b = e.1.inside_border(&b); //TODO unify border opt fns into on layer (why tf is border for event done here and border for render done in RenderLink??)

        if let Some(ee) = e.0.filter(&b) {
            let e = (ee,&b,e.2);
            let w = self.ctx.link(self.widget.reference());
            (**self.widget)._event(w,e)
        }else{
            false
        }
    }
    #[inline]
    pub fn _route_event(&mut self, e: (EEvent<E>,&Bounds,u64), child: E::WidgetPath) -> Result<bool,()> {
        let w = self.ctx.link(self.widget.reference());
        (**self.widget)._route_event(w,e,child)
    }
    /// bypasses Context and Handler(s)
    #[inline]
    pub fn _size(&mut self) -> ESize<E> {
        let w = self.ctx.link(self.widget.reference());
        (**self.widget)._size(w)
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

    #[inline]
    pub fn for_childs<'s>(&'s mut self, mut f: impl FnMut(Link<E>)) -> Result<(),()> where 'c: 's {
        for i in 0..self.widget.childs() {
            let l = self.for_child(i)?;
            f(l);
        }
        Ok(())
    }
    #[inline]
    pub fn for_child<'s>(&'s mut self, i: usize) -> Result<Link<E>,()> where 'c: 's {
        let path = self.widget.path.refc();
        let stor = self.widget.stor;

        let c = self.widget.child(i)?;

        let w = c.resolve_widget(stor)?;
        let w = Resolved{
            path: w.in_parent_path(path.refc()).into(),
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
        let mut dest = Vec::with_capacity(self.widget.childs());
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
        let mut new_path = self.widget.path.refc().attached_subpath(&p);
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


    pub fn reference<'s>(&'s mut self) -> Link<'s,E> where 'c: 's {
        Link{
            widget: self.widget.reference(),
            ctx: self.ctx
        }
    }

    pub fn childs<'s>(&'s self) -> impl Iterator<Item=Resolvable<'s,E>>+'s where 'c: 's {
        let w = (&self.widget).short_lt(); //TODO this looks like a fkn move and ref
        (0..w.childs())
            .map(move |i| w.child(i).unwrap() )
    }


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
    pub fn enqueue<I,O>(&mut self, i: I, o: O, p: i64) where ECQueue<E>: Queue<I,O> {
        self.ctx.queue_mut().push(i,o,p)
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