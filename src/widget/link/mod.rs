//! The [`Link`] is used to interface [widgets](Widget) thru and tracks the current [path](Env::WidgetPath)
use std::ops::DerefMut;
use std::ops::Deref;
use crate::root::RootRef;

use super::*;

#[doc(hidden)]
pub mod imp;
use imp::*;

/// Holds a immutable reference to the current [`Widget`] and the [widget tree](Env::Storage), also a mutable reference to the [`Context`](Env::Context)
pub struct Link<'c,'cc: 'c,E> where E: Env {
    pub ctx: &'c mut E::Context<'cc>,
    pub widget: Resolved<'c,E>,
}

impl<'c,'cc: 'c,E> Link<'c,'cc,E> where E: Env {
    /// Enqueue mutable access to this widget
    #[inline] 
    pub fn mutate(&mut self, f: PtrMutEvent<E>) {
        self.mutate_at(f,StdOrder::PostCurrent,0)
    }
    #[inline] 
    pub fn mutate_at<O>(&mut self, f: PtrMutEvent<E>, o: O, p: i64) where for<'a> ECQueue<'a,E>: Queue<StdEnqueueable<E>,O> {
        self.enqueue(StdEnqueueable::MutateRoot{f},o,p)
    }
    /// Enqueue mutable access to this widget
    #[inline] 
    pub fn mutate_closure(&mut self, f: BoxMutEvent<E>) {
        self.mutate_closure_at(f,StdOrder::PostCurrent,0)
    }
    #[inline] 
    pub fn mutate_closure_at<O>(&mut self, f: BoxMutEvent<E>, o: O, p: i64) where for<'a> ECQueue<'a,E>: Queue<StdEnqueueable<E>,O> {
        self.enqueue(StdEnqueueable::MutateRootClosure{f},o,p)
    }
    /// Enqueue message-style invoking of [WidgetMut::message]
    #[inline]
    pub fn message_mut(&mut self, m: E::Message) {
        self.message_mut_at(m,StdOrder::PostCurrent,0)
    }
    /// Enqueue message-style invoking of [WidgetMut::message]
    #[inline]
    pub fn message_mut_at<O>(&mut self, m: E::Message, o: O, p: i64) where for<'a> ECQueue<'a,E>: Queue<StdEnqueueable<E>,O> {
        self.enqueue(StdEnqueueable::MutMessage{path: self.widget.direct_path.refc(),msg:m},o,p)
    }
    /// Enqueue immutable access to this widget
    #[inline] 
    pub fn later(&mut self, f: PtrAccessWidget<E>) {
        self.later_at(f,StdOrder::PostCurrent,0)
    }
    #[inline] 
    pub fn later_at<O>(&mut self, f: PtrAccessWidget<E>, o: O, p: i64) where for<'a> ECQueue<'a,E>: Queue<StdEnqueueable<E>,O> {
        self.enqueue(StdEnqueueable::AccessWidget{path: self.widget.direct_path.refc(),f},o,p)
    }
    /// Enqueue immutable access to this widget
    #[inline] 
    pub fn later_closure(&mut self, f: BoxAccessWidget<E>) {
        self.later_closure_at(f,StdOrder::PostCurrent,0)
    }
    #[inline] 
    pub fn later_closure_at<O>(&mut self, f: BoxAccessWidget<E>, o: O, p: i64) where for<'a> ECQueue<'a,E>: Queue<StdEnqueueable<E>,O> {
        self.enqueue(StdEnqueueable::AccessWidgetClosure{path: self.widget.direct_path.refc(),f},o,p)
    }
    #[inline]
    pub fn enqueue_invalidate(&mut self) {
        self.enqueue(StdEnqueueable::InvalidateWidget{path: self.widget.direct_path.refc()},StdOrder::PreRender,0)
    }
    /// Mark the current widget as validated
    /// 
    /// This should and should only be called from widget's render fn
    #[inline]
    pub fn enqueue_validate_render(&mut self) {
        self.enqueue(StdEnqueueable::ValidateWidgetRender{path: self.widget.direct_path.refc()},StdOrder::RenderValidation,0)
    }
    #[inline]
    pub fn enqueue_validate_size(&mut self, s: ESize<E>) {
        self.enqueue(StdEnqueueable::ValidateWidgetSize{path: self.widget.direct_path.refc(),size: s},StdOrder::RenderValidation,0)
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
    pub fn direct_path(&self) -> E::WidgetPath {
        self.widget.direct_path.refc()
    }

    #[inline]
    pub fn ident(&self) -> WidgetIdent<E> {
        self.widget.ident()
    }

    #[inline]
    pub fn render(&mut self, r: &mut ERenderer<'_,E>) {
        E::Context::<'_>::build_handler()._render(
            self.reference(),
            r,
            &mut |mut l,r| {
                l._render(r)
            }
        )
    }
    /// send event to this widget
    #[inline]
    pub fn event_direct(&mut self, e: &EventCompound<E>) -> EventResp {
        E::Context::<'_>::build_handler()._event_direct(
            self.reference(),
            e,
            &mut |mut l,e| {
                l._event_direct(e)
            }
        )
    }
    /// Send event to subpath
    /// 
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Widgets::send_event`](Widgets::send_event)
    #[inline]
    pub fn send_event(&mut self, e: &EventCompound<E>, child: E::WidgetPath) -> Result<EventResp,E::Error> {
        E::Context::<'_>::build_handler()._send_event(
            self.reference(),
            e,
            child,
            &mut |mut l,e,child| {
                l._send_event(e,child)
            }
        )
    }
    /// [Layout](StdGonstraints) constraints of this widget
    #[inline]
    pub fn size(&mut self, e: &EStyle<E>) -> ESize<E> {
        E::Context::<'_>::build_handler()._size(
            self.reference(),
            e,
            &mut |mut l,e| {
                l._size(e)
            }
        )
    }
    #[inline]
    #[deprecated="Non-root link is panic"]
    pub fn _event_root(&mut self, e: &EventCompound<E>) -> EventResp {
        assert!(self.path().is_empty());
        E::Context::<'_>::build_handler()._event_root(
            self.reference(),
            e,
            &mut |mut l,e| {
                //TODO everything wrong here with event root propagation and tail
                l.event_direct(e)
            }
        )
    }
    /// Bypasses [`Context`](Env::Context) and [Handler(s)](Context::Handler)
    #[inline]
    pub fn _render(&mut self, r: &mut ERenderer<'_,E>) {
        let w = self.ctx.link(self.widget.reference());
        (*self.widget.wref)._render(w,r)
    }
    /// Bypasses [`Context`](Env::Context) and [Handler(s)](Context::Handler)
    #[inline]
    pub fn _event_direct(&mut self, e: &EventCompound<E>) -> EventResp {
        let w = self.ctx.link(self.widget.reference());
        (*self.widget.wref)._event_direct(w,e)
    }
    #[inline]
    pub fn _send_event(&mut self, e: &EventCompound<E>, sub: E::WidgetPath) -> Result<EventResp,E::Error> {
        let e = EventCompound{
            filter: e.filter.clone().attach_path_prefix(sub.clone()),
            ..e.clone()
        };
        let _ = self.widget.resolve(sub,self.widget.root.fork(),self.ctx)?;
        let w = self.ctx.link(self.widget.reference());
        Ok( (*self.widget.wref)._event_direct(w,&e) )
    }
    /// Bypasses [`Context`](Env::Context) and [Handler(s)](Context::Handler)
    #[inline]
    pub fn _size(&mut self, e: &EStyle<E>) -> ESize<E> {
        let w = self.ctx.link(self.widget.reference());
        (*self.widget.wref)._size(w,e)
    }
    #[inline]
    pub fn _tabulate(&mut self, op: TabulateOrigin<E>, dir: TabulateDirection) -> Result<TabulateResponse<E>,E::Error> {
        let w = self.ctx.link(self.widget.reference());
        (*self.widget)._tabulate(w,op,dir)
    }

    #[deprecated="Not needed in OOF anymore"]
    pub fn trace_bounds(&mut self, root_bounds: &Bounds, e: &EStyle<E>, force: bool) -> Bounds {
        self.widget.root.trace_bounds(self.ctx,self.widget.path.refc(),root_bounds,e,force).unwrap()
    }

    #[inline]
    #[deprecated="Not needed in OOF anymore"]
    pub fn _trace_bounds(&mut self, sub: E::WidgetPath, root_bounds: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error> {
        let w = self.ctx.link(self.widget.reference());
        (*self.widget).trace_bounds(w,sub,root_bounds,e,force)
    }

    #[inline]
    pub fn is_hovered(&self) -> bool where for<'a> E::Context<'a>: CtxStdState<E> {
        self.ctx.state().is_hovered(&self.id())
    }
    #[inline]
    pub fn is_focused(&self) -> bool where for<'a> E::Context<'a>: CtxStdState<E> {
        self.ctx.state().is_focused(&self.id())
    }

    #[deprecated]
    #[allow(deprecated)]
    #[inline]
    pub fn child_paths(&mut self) -> Vec<E::WidgetPath> {
        self.widget.child_paths(self.widget.root.fork(),self.ctx)
    }

    /// Run closure for every child
    #[inline]
    pub fn for_childs<'s>(&'s mut self, mut f: impl FnMut(Link<E>)) -> Result<(),E::Error> where 'c: 's {
        for i in 0..self.widget.childs() {
            let l = self.for_child(i)?;
            f(l);
        }
        Ok(())
    }
    /// Get Link for specific child by index
    #[inline]
    pub fn for_child<'s>(&'s mut self, i: usize) -> Result<Link<'_,'cc,E>,E::Error> where 'c: 's { //TODO rename to child(i), use with_child_specific
        let path = self.widget.path.refc();

        let w = self.widget.child(i,self.widget.root.fork(),self.ctx)?;
        let mut r;

        let cpath = w.in_parent_path(path.refc()).into();
        r = Resolved {
            path: cpath.refc(),
            direct_path: cpath,
            wref: w,
            root: self.widget.root.fork(),
        };

        let l = Link{
            widget: r,
            ctx: self.ctx,
        };

        Ok(l)
    }
    #[inline]
    pub fn _with_link<'s>(ctx: &mut E::Context<'_>, w: Resolved<'s,E>, f: impl FnOnce(Link<E>)) where 'c: 's {
        let l = Link{
            widget: w.lt(),
            ctx,
        };
        f(l);
    }

    pub fn child_sizes(&mut self, e: &EStyle<E>) -> Result<Vec<ESize<E>>,E::Error> {
        let mut dest = Vec::with_capacity(self.widget.childs());
        self.for_childs(#[inline] |mut w| dest.push(w.size(e)) )?;
        Ok(dest)
    }
    pub fn child_bounds(&mut self, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> {
        let w = self.ctx.link(self.widget.reference());
        (*self.widget).child_bounds(w,b,e,force)
    }

    /// Link for Widget by Path, resolves Widget by Path
    #[inline]
    pub fn with_widget<'s>(&'s mut self, p: E::WidgetPath) -> Result<Link<'s,'cc,E>,E::Error> where 'c: 's {
        Ok(
            Link{
                widget: self.widget.root.widget(p,self.ctx)?,
                ctx: self.ctx
            }
        )
    }

    /// Current widget must be parent of rw
    #[inline]
    pub fn with_child_specific<'s>(&'s mut self, rw: WidgetRef<'s,E>) -> Result<Link<'s,'cc,E>,E::Error> where 'c: 's {
        let path = rw.in_parent_path(self.path());
        self.with_resolvable(rw,path)
    }

    #[inline]
    pub fn with_resolvable<'s>(&'s mut self, rw: WidgetRef<'s,E>, path: E::WidgetPath) -> Result<Link<'s,'cc,E>,E::Error> where 'c: 's {
        let mut r;

        r = Resolved {
            path: path.refc(),
            direct_path: path,
            wref: rw,
            root: self.widget.root.fork(),
        };

        let l = Link{
            widget: r,
            ctx: self.ctx,
        };

        Ok(l)
    }

    #[inline]
    pub fn with_root<'s>(&'s mut self) -> Result<Link<'s,'cc,E>,()> where 'c: 's {
        self.with_widget(WidgetPath::empty()).map_err(|_| ()) //TODO GuionError everywhere
    }

    pub fn resolve_sub<'s>(&'s mut self, sub: &E::WidgetPath) -> Result<Link<'s,'cc,E>,E::Error> where 'c: 's {
        let path = self.widget.path.refc().attached_subpath(sub);
        let rw = self.widget.wref.resolve(sub.refc(),self.widget.root.fork(),self.ctx)?;
        
        let mut r;

        r = Resolved {
            path: path.refc(),
            direct_path: path,
            wref: rw,
            root: self.widget.root.fork(),
        };
        
        Ok(
            Link{
                widget: r,
                ctx: self.ctx,
            }
        )
    }

    #[inline]
    pub fn reference<'s>(&'s mut self) -> Link<'s,'cc,E> where 'c: 's {
        Link{
            widget: self.widget.reference(),
            ctx: self.ctx
        }
    }

    // #[inline]
    // pub fn childs<'s>(&'s mut self) -> impl Iterator<Item=WidgetRef<'s,E>>+'s where 'c: 's {
    //     let w = &**self.widget; //TODO this looks like a fkn move and ref
    //     (0..w.childs())
    //         .map(#[inline] move |i| w.child(i,self.widget.root.fork(),self.ctx).unwrap() )
    // }


    // /// Iterate from current up to the root element
    // #[inline]
    // pub fn parents(&'c self) -> Parents<'c,E> {
    //     Parents{
    //         stor: self.widget.stor,
    //         next: Some(self.widget.path.refc()),
    //     }
    // }

    #[inline]
    pub fn enqueue<I,O>(&mut self, i: I, o: O, p: i64) where for<'a> ECQueue<'a,E>: Queue<I,O> {
        self.ctx.queue_mut().push(i,o,p)
    }
}

impl<'a,'cc: 'a,E> Deref for Link<'a,'cc,E> where E: Env {
    type Target = E::Context<'cc>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}
impl<'a,'cc: 'a,E> DerefMut for Link<'a,'cc,E> where E: Env {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}
