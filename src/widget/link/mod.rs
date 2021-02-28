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
    /// enqueue message-style invoking of [WidgetMut::message]
    #[inline]
    pub fn message_mut(&mut self, m: E::Message) {
        self.message_mut_at(m,StdOrder::PostCurrent,0)
    }
    /// enqueue message-style invoking of [WidgetMut::message]
    #[inline]
    pub fn message_mut_at<O>(&mut self, m: E::Message, o: O, p: i64) where ECQueue<E>: Queue<StdEnqueueable<E>,O> {
        self.enqueue(StdEnqueueable::MutMessage{path: self.widget.path.refc(),msg:m},o,p)
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

    #[inline]
    pub fn path(&self) -> E::WidgetPath {
        self.widget.path.refc()
    }

    #[inline]
    pub fn ident(&self) -> WidgetIdent<E> {
        self.widget.ident()
    }

    #[inline]
    pub fn render(&mut self, r: &mut RenderLink<E>) {
        self.ctx.render(self.widget.reference(),r)
    }
    /// send event to this widget
    #[inline]
    pub fn event_direct(&mut self, e: &EventCompound<E>) -> EventResp {
        self.ctx.event_direct(self.widget.reference(),e)
    }
    /// send event to subpath
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Widgets::send_event`](Widgets::send_event)
    #[inline]
    pub fn send_event(&mut self, e: &EventCompound<E>, child: E::WidgetPath) -> Result<EventResp,GuionError<E>> {
        self.ctx.send_event(self.widget.reference(),e,child)
    }
    /// layout constraints of this widget
    #[inline]
    pub fn size(&mut self, e: &EStyle<E>) -> ESize<E> {
        self.ctx.size(self.widget.reference(),e)
    }
    #[inline]
    #[deprecated="Non-root link is panic"]
    pub fn _event_root(&mut self, e: &EventCompound<E>) -> EventResp {
        assert!(self.path().is_empty());
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
    pub fn _event_direct(&mut self, e: &EventCompound<E>) -> EventResp {
        let w = self.ctx.link(self.widget.reference());
        (**self.widget)._event_direct(w,e)
    }
    #[inline]
    pub fn _send_event(&mut self, e: &EventCompound<E>, child: E::WidgetPath) -> Result<EventResp,GuionError<E>> {
        let e = EventCompound{
            filter: e.filter.clone().attach_path_prefix(child.clone()),
            ..e.clone()
        };
        let _ = self.widget.resolve(child)?;
        let w = self.ctx.link(self.widget.reference());
        Ok( (**self.widget)._event_direct(w,&e) )
    }
    /// bypasses Context and Handler(s)
    #[inline]
    pub fn _size(&mut self, e: &EStyle<E>) -> ESize<E> {
        let w = self.ctx.link(self.widget.reference());
        (**self.widget)._size(w,e)
    }
    #[inline]
    pub fn _tabulate(&mut self, op: TabulateOrigin<E>, dir: TabulateDirection) -> Result<TabulateResponse<E>,GuionError<E>> {
        let w = self.ctx.link(self.widget.reference());
        (**self.widget)._tabulate(w,op,dir)
    }

    #[deprecated="Not needed in OOF anymore"]
    pub fn trace_bounds(&mut self, root_bounds: &Bounds, e: &EStyle<E>, force: bool) -> Bounds {
        self.widget.stor.trace_bounds(self.ctx,self.widget.path.refc(),root_bounds,e,force).unwrap()
    }

    #[inline]
    #[deprecated="Not needed in OOF anymore"]
    pub fn _trace_bounds(&mut self, sub: E::WidgetPath, root_bounds: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,GuionError<E>> {
        let w = self.ctx.link(self.widget.reference());
        (**self.widget).trace_bounds(w,sub,root_bounds,e,force)
    }

    #[inline]
    pub fn is_hovered(&self) -> bool where E::Context: CtxStdState<E> {
        self.ctx.state().is_hovered(&self.id())
    }
    #[inline]
    pub fn is_focused(&self) -> bool where E::Context: CtxStdState<E> {
        self.ctx.state().is_focused(&self.id())
    }

    #[deprecated]
    #[allow(deprecated)]
    #[inline]
    pub fn child_paths(&self) -> Vec<E::WidgetPath> {
        self.widget.child_paths()
    }

    /// Run closure for every child
    #[inline]
    pub fn for_childs<'s>(&'s mut self, mut f: impl FnMut(Link<E>)) -> Result<(),GuionError<E>> where 'c: 's {
        for i in 0..self.widget.childs() {
            let l = self.for_child(i)?;
            f(l);
        }
        Ok(())
    }
    /// Get Link for specific child by index
    #[inline]
    pub fn for_child<'s>(&'s mut self, i: usize) -> Result<Link<E>,GuionError<E>> where 'c: 's { //TODO rename to child(i)
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
            widget: w,
            ctx: self.ctx,
        };

        Ok(l)
    }
    #[inline]
    pub fn _with_link<'s>(ctx: &mut E::Context, w: Resolved<'s,E>, f: impl FnOnce(Link<E>)) where 'c: 's {
        let l = Link{
            widget: w,
            ctx,
        };
        f(l);
    }

    pub fn child_sizes(&mut self, e: &EStyle<E>) -> Result<Vec<ESize<E>>,GuionError<E>> {
        let mut dest = Vec::with_capacity(self.widget.childs());
        self.for_childs(#[inline] |mut w| dest.push(w.size(e)) )?;
        Ok(dest)
    }
    pub fn child_bounds(&mut self, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> {
        let w = self.ctx.link(self.widget.reference());
        (**self.widget).child_bounds(w,b,e,force)
    }

    /// Link for Widget by Path, resolves Widget by Path
    #[inline]
    pub fn with_widget<'s>(&'s mut self, p: E::WidgetPath) -> Result<Link<'s,E>,GuionError<E>> where 'c: 's {
        Ok(
            Link{
                widget: self.widget.stor.widget(p)?,
                ctx: self.ctx
            }
        )
    }

    /// current widget must be parent of rw
    #[inline]
    pub fn with_child_specific<'s>(&'s mut self, rw: Resolvable<'s,E>) -> Result<Link<'s,E>,GuionError<E>> where 'c: 's {
        let path = rw.in_parent_path(self.path());
        self.with_resolvable(rw,path)
    }

    #[inline]
    pub fn with_resolvable<'s>(&'s mut self, rw: Resolvable<'s,E>, path: E::WidgetPath) -> Result<Link<'s,E>,GuionError<E>> where 'c: 's {
        let rw = rw.resolve_widget(&self.widget.stor)?;
        let w = Resolved{
            path,
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

    #[inline]
    pub fn with_root<'s>(&'s mut self) -> Result<Link<'s,E>,()> where 'c: 's {
        self.with_widget(WidgetPath::empty()).map_err(|_| ()) //TODO GuionError everywhere
    }

    pub fn resolve_sub<'s>(&'s mut self, p: E::WidgetPath) -> Result<Link<'s,E>,GuionError<E>> where 'c: 's {
        let mut new_path = self.widget.path.refc().attached_subpath(&p); //TODO w h a t
        let rw = self.widget.wref.resolve(p.refc())?;
        rw.extract_path(&mut new_path); //TODO extract FINAL path
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

    #[inline]
    pub fn reference<'s>(&'s mut self) -> Link<'s,E> where 'c: 's {
        Link{
            widget: self.widget.reference(),
            ctx: self.ctx
        }
    }

    #[inline]
    pub fn childs<'s>(&'s self) -> impl Iterator<Item=Resolvable<'s,E>>+'s where 'c: 's {
        let w = &self.widget; //TODO this looks like a fkn move and ref
        (0..w.childs())
            .map(#[inline] move |i| w.child(i).unwrap() )
    }


    /// iterate from current up to the root element
    #[inline]
    pub fn parents(&'c self) -> Parents<'c,E> {
        Parents{
            stor: self.widget.stor,
            next: Some(self.widget.path.refc()),
        }
    }
    
    #[inline]
    pub fn with_ctx<F: Env<WidgetPath=E::WidgetPath,Storage=E::Storage>>(self, ctx: &'c mut F::Context) -> Link<'c,F> where E::WidgetPath: WidgetPath<F>, E::Storage: Widgets<F> {
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
