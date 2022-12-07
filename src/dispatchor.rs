use std::any::TypeId;
use std::marker::PhantomData;

use crate::aliases::{ERenderer, ESize};
use crate::env::Env;
use crate::util::tabulate::{TabulateDirection, TabulateOrigin, TabulateResponse};
use crate::{event_new, EventResp};
use crate::newpath::{PathResolvusDyn, PathFragment, PathStack};
use crate::queron::Queron;
use crate::widget::{Widget, WidgetWithResolveChildDyn};
use crate::widget::as_widget::AsWidget;
use crate::widget::as_widgets::AsWidgets;
use crate::widget::cache::{WidgetCache, DynWidgetCache};
use crate::widget::dyn_tunnel::WidgetDyn;

pub trait AsWidgetDispatch<'z,V,R,E>
where
    V: AsWidget<E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(&mut self, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww, V: 'z;
}

pub trait AsWidgetsDispatch<CID,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResult<'w,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w;

    fn call_none(&mut self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        self.call::<std::convert::Infallible>(None,root,ctx)
    }
}

pub struct AsWidgetsResult<'w,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: usize,
    pub child_id: CID,
    pub widget: &'w W,
    _p: PhantomData<E>,
}

// impl<'w,'y,W,CID,E> AsWidgetsResult<'w,&'y W,CID,E>
// where
//     W: Widget<E> + ?Sized,
//     'w: 'y,
//     CID: PathFragment<E> + Clone + 'static,
//     E: Env,
// {
//     pub fn deref(self) -> AsWidgetsResult<'y,W,CID,E> {
//         AsWidgetsResult {
//             idx: self.idx,
//             child_id: self.child_id,
//             widget: self.widget,
//         }
//     }
// }

impl<'w,W,CID,E> AsWidgetsResult<'w,W,CID,E>
where
    W: Widget<E> + ?Sized,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub fn from_some(idx: usize, child_id: CID, widget: &'w W) -> Option<Self> {
        Some(Self { idx, child_id, widget, _p: PhantomData })
    }

    // pub fn convert<W>(self) -> AsWidgetsResult<'z,'w,'ww,W,E> where W: 'z, for<'a,'b> W: AsWidgets<E,Widget<'a,'b>=V::Widget<'a,'b>,WidgetCache=V::WidgetCache,ChildID=V::ChildID> {
    //     AsWidgetsResult {
    //         idx: self.idx,
    //         child_id: self.child_id.clone(),
    //         widget: self.widget,
    //     }
    // }
}

pub trait AsWidgetsResolveDispatch<CID,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResolveResult<'w,'_,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w;
    
    fn call_none(&mut self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        self.call::<std::convert::Infallible>(None,root,ctx)
    }
}

pub struct AsWidgetsResolveResult<'w,'p,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: usize,
    pub child_id: CID,
    pub resolvus: &'p (dyn PathResolvusDyn<E>+'p),
    pub widget: &'w W,
}

impl<'w,'p,W,CID,E> AsWidgetsResolveResult<'w,'p,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub fn from_some(idx: usize, child_id: CID, resolvus: &'p (dyn PathResolvusDyn<E>+'p), widget: &'w W) -> Option<Self> {
        Some(Self { idx, child_id, resolvus, widget })
    }

    // pub fn convert<W>(self) -> AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,W,E> where W: 'z, W: 'p, for<'a,'b> W: AsWidgets<E,Widget<'a,'b>=V::Widget<'a,'b>,WidgetCache=V::WidgetCache,ChildID=V::ChildID> {
    //     AsWidgetsResolveResult {
    //         idx: self.idx,
    //         child_id: self.child_id.clone(),
    //         resolvus: self.resolvus,
    //         widget: self.widget,
    //     }
    // }
}

// impl<'w,'p,'y,W,CID,E> AsWidgetsResolveResult<'w,'p,&'y W,CID,E>
// where
//     W: Widget<E> + ?Sized + 'w,
//     'w: 'y,
//     CID: PathFragment<E> + Clone + 'static,
//     E: Env,
// {
//     pub fn deref(self) -> AsWidgetsResolveResult<'y,'p,W,CID,E> {
//         AsWidgetsResolveResult {
//             idx: self.idx,
//             child_id: self.child_id,
//             resolvus: self.resolvus,
//             widget: self.widget,
//         }
//     }
// }

pub trait AsWidgetsIndexedDispatch<CID,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(&mut self, idx: usize, child_id: CID, widget: &'w W, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w;
}

pub struct AsWidgetClosure<'z,C,V,R,E>(C,PhantomData<(fn(*const V),R,E,&'z ())>)
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R; //TODO find some way to do FnOnce thru reference

impl<'z,C,V,R,E> AsWidgetClosure<'z,C,V,R,E> 
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where R: 'a, V: 'a, C: 'a, 'z: 'a {
        Self(c,PhantomData)
    }
}

// pub struct AsWidgetsClosure<'z,C,V,R,E>(C,PhantomData<(fn(*const V),R,E,&'z ())>)
// where
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     for<'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResult<'z,'w,'ww,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R;

// impl<'z,C,V,R,E> AsWidgetsClosure<'z,C,V,R,E> 
// where
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     for<'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResult<'z,'w,'ww,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
// {
//     #[inline]
//     pub fn new<'a>(c: C) -> Self where R: 'a, V: 'a, C: 'a, 'z: 'a {
//         Self(c,PhantomData)
//     }
// }

// pub struct AsWidgetsAllClosure<'z,C,V,E>(C,PhantomData<(fn(*const V),E,&'z ())>)
// where
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>);

// impl<'z,C,V,E> AsWidgetsAllClosure<'z,C,V,E> 
// where
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>)
// {
//     #[inline]
//     pub fn new<'a>(c: C) -> Self where V: 'a, C: 'a, 'z: 'a {
//         Self(c,PhantomData)
//     }
// }

// pub struct AsWidgetsResolveClosure<'z,C,V,R,E>(C,PhantomData<(fn(*const V),R,E,&'z ())>)
// where
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     for<'p,'pp,'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R;

// impl<'z,C,V,R,E> AsWidgetsResolveClosure<'z,C,V,R,E> 
// where
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     for<'p,'pp,'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
// {
//     #[inline]
//     pub fn new<'a>(c: C) -> Self where V: 'a, C: 'a, 'z: 'a {
//         Self(c,PhantomData)
//     }
// }

impl<'z,C,V,R,E> AsWidgetDispatch<'z,V,R,E> for AsWidgetClosure<'z,C,V,R,E> 
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(widget,root,ctx)
    }
}

// impl<'z,C,V,R,E> AsWidgetsDispatch<'z,V,R,E> for AsWidgetsClosure<'z,C,V,R,E> 
// where
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     for<'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResult<'z,'w,'ww,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
// {
//     #[inline]
//     fn call<'w,'ww,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResult<'z,'w,'ww,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
//     where
//         'ww: 'w, 'z: 'ww
//     {
//         (self.0)(result,root,ctx)
//     }
// }

// impl<'z,C,V,R,E> AsWidgetsResolveDispatch<'z,V,R,E> for AsWidgetsResolveClosure<'z,C,V,R,E> 
// where
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     for<'p,'pp,'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
// {
//     #[inline]
//     fn call<'w,'ww,'p,'pp,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
//     where
//         'ww: 'w, 'z: 'ww, 'pp: 'p, 'z: 'pp, V: 'z
//     {
//         (self.0)(result,root,ctx)
//     }
// }

// impl<'z,C,V,E> AsWidgetsIndexedDispatch<'z,V,E> for AsWidgetsAllClosure<'z,C,V,E> 
// where
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>)
// {
//     #[inline]
//     fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, child_id: V::ChildID, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
//     where
//         'ww: 'w, 'z: 'ww
//     {
//         (self.0)(idx,child_id,widget,root,ctx)
//     }
// }

// pub struct AsWidgetsIndexedWrap<C>(pub C);

// impl<'z,C,V,E> AsWidgetsDispatch<'z,V,(),E> for AsWidgetsIndexedWrap<C>
// where
//     C: AsWidgetsIndexedDispatch<'z,V,E>,
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     //for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>)
// {
//     #[inline]
//     fn call<'w,'ww,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResult<'z,'w,'ww,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
//     where
//         'ww: 'w, 'z: 'ww
//     {
//         self.0.call(result,root,ctx)
//     }
// }

// pub struct AsWidgetToSDispatchErase<'z,C,V,W,R,E>(pub usize, pub W::Bound, pub W::ChildID, pub C, pub PhantomData<(*const V,Box<R>,Box<E>)>)
// where
//     C: AsWidgetsDispatch<'z,W,R,E>,
//     V: AsWidget<E> + ?Sized,
//     W: ?Sized,
//     for<'a> W: AsWidgets<'z,E,Widget<'a>=(dyn WidgetDyn<E> + 'a)> + 'z,
//     E: Env,
//     Self: 'z;

// impl<'z,C,V,W,R,E> AsWidgetDispatch<'z,V,R,E> for AsWidgetToSDispatchErase<'z,C,V,W,R,E>
// where
//     C: AsWidgetsDispatch<'z,W,R,E>,
//     V: AsWidget<E> + ?Sized,
//     W: ?Sized,
//     for<'a> W: AsWidgets<'z,E,Widget<'a>=(dyn WidgetDyn<E> + 'a)> + 'z,
//     E: Env,
//     Self: 'z
// {

// }

pub struct AsWidgetClosureErased<'z,C,V,R,E>(C,PhantomData<(fn(*const V),R,E,&'z ())>)
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w (dyn WidgetDyn<E> + 'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> R;

impl<'z,C,V,R,E> AsWidgetClosureErased<'z,C,V,R,E> 
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w (dyn WidgetDyn<E> + 'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where R: 'a, V: 'a, C: 'a, 'z: 'a {
        Self(c,PhantomData)
    }
}

impl<'z,C,V,R,E> AsWidgetDispatch<'z,V,R,E> for AsWidgetClosureErased<'z,C,V,R,E>
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w (dyn WidgetDyn<E> + 'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(widget.erase(),root,ctx)
    }
}

pub fn stupid_widget_cache_cast<A,B,E>(v: &mut A) -> &mut B
where
    A: WidgetCache<E>,
    B: WidgetCache<E>,
    E: Env
{
    if TypeId::of::<A>() == TypeId::of::<B>() {
        return unsafe { &mut *(v as *mut A as *mut B) }
    }
    if TypeId::of::<A>() == TypeId::of::<DynWidgetCache<E>>() {
        let wc = unsafe { &mut *(v as *mut A as *mut DynWidgetCache<E>) };
        return wc.downcast_mut_or_reset::<B>();
    }
    todo!()
}

pub struct AsWidgetsOnWithChild<C,R,E>(pub Option<C>, pub PhantomData<(E,fn()->R)>)
where
    C: for<'www,'ww,'c,'cc> FnOnce(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R,
    E: Env;

impl<CID,C,R,E> AsWidgetsDispatch<CID,R,E> for AsWidgetsOnWithChild<C,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    C: for<'www,'ww,'c,'cc> FnOnce(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R,
    E: Env
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResult<'w,W,CID,E>>, _: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        let f = self.0.take().unwrap();

        if let Some(r) = result {
            f(Ok(r.widget.erase()),ctx)
        } else {
            f(Err(()),ctx)
        }
    }
}

pub struct AsWidgetsOnWithResolveChild<C,R,E>(pub Option<C>, pub PhantomData<(E,fn()->R)>)
where
    C: for<'a,'c,'cc> FnOnce(Result<WidgetWithResolveChildDyn<'a,E>,E::Error>,&'c mut E::Context<'cc>) -> R,
    E: Env;

impl<CID,C,R,E> AsWidgetsResolveDispatch<CID,R,E> for AsWidgetsOnWithResolveChild<C,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    C: for<'a,'c,'cc> FnOnce(Result<WidgetWithResolveChildDyn<'a,E>,E::Error>,&'c mut E::Context<'cc>) -> R,
    E: Env
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResolveResult<'w,'_,W,CID,E>>, _: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        let f = self.0.take().unwrap();

        if let Some(r) = result {
            let d = WidgetWithResolveChildDyn {
                idx: r.idx,
                sub_path: r.resolvus,
                widget: r.widget.erase(),
            };

            f(Ok(d),ctx)
        } else {
            f(Err(todo!()),ctx)
        }
    }
}

pub struct AsWidgetsOnTabulate<'x,P,Ph,E>(pub &'x Ph, pub &'x P, pub TabulateOrigin<'x,E>, pub TabulateDirection, pub PhantomData<E>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    E: Env;

impl<'x,CID,P,Ph,E> AsWidgetsDispatch<CID,Option<Result<TabulateResponse<E>,E::Error>>,E> for AsWidgetsOnTabulate<'x,P,Ph,E>
where
    CID: PathFragment<E> + Clone + 'static,
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResult<'w,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<Result<TabulateResponse<E>,E::Error>>
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            Some(
                r.widget._tabulate(&r.child_id.push_on_stack(self.0), self.1, self.2.clone(), self.3, root, ctx)
            )
        } else {
            None
        }
    }
}

impl<'x,CID,P,Ph,E> AsWidgetsResolveDispatch<CID,Option<Result<TabulateResponse<E>,E::Error>>,E> for AsWidgetsOnTabulate<'x,P,Ph,E>
where
    CID: PathFragment<E> + Clone + 'static,
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResolveResult<'w,'_,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<Result<TabulateResponse<E>,E::Error>>
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            Some(
                r.widget._tabulate(&r.child_id.push_on_stack(self.0), self.1, self.2.clone(), self.3, root, ctx)
            )
        } else {
            None
        }
    }
}

pub struct SuperAWR<W,CID,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: usize,
    pub child_id: CID,
    pub invoke: W,
    _p: PhantomData<E>,
}

pub struct SuperAWRR<'p,W,CID,E>
where
    W: ?Sized,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: usize,
    pub child_id: CID,
    pub resolvus: &'p (dyn PathResolvusDyn<E>+'p),
    pub invoke: W,
}

type SuperSizeFn<'w,Ph,P,WC,E> = &'w mut (dyn FnMut(&Ph,&P,&mut WC,<E as Env>::RootRef<'_>,&mut <E as Env>::Context<'_>)->ESize<E> + 'w);

pub struct AsWidgetsAllSize<P,Ph,C,WW,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P))>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(usize,WW::ChildID,SuperSizeFn<Ph,P,WW::WidgetCache,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,E> AsWidgetsIndexedDispatch<WW::ChildID,E> for AsWidgetsAllSize<P,Ph,C,WW,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(usize,WW::ChildID,SuperSizeFn<Ph,P,WW::WidgetCache,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, idx: usize, child_id: WW::ChildID, widget: &'w W, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w
    {
        self.0(
            idx,
            child_id,
            &mut move |path,stack,cache,root,ctx|
                widget.size(path,stack,stupid_widget_cache_cast(cache),root,ctx),
            root, ctx
        )
    }
}

pub struct AsWidgetsSize<P,Ph,C,WW,R,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P)->R)>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWR<SuperSizeFn<Ph,P,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,R,E> AsWidgetsDispatch<WW::ChildID,R,E> for AsWidgetsSize<P,Ph,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWR<SuperSizeFn<Ph,P,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResult<'w,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWR {
                    idx: r.idx,
                    child_id: r.child_id,
                    invoke: &mut move |path,stack,cache,root,ctx|
                        r.widget.size(path,stack,stupid_widget_cache_cast(cache),root,ctx),
                    _p: PhantomData,
                }),
                root,ctx
            )
        } else {
            self.0(None,root,ctx)
        }
    }
}

pub struct AsWidgetsResolveSize<P,Ph,C,WW,R,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P)->R)>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWRR<'_,SuperSizeFn<Ph,P,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,R,E> AsWidgetsResolveDispatch<WW::ChildID,R,E> for AsWidgetsResolveSize<P,Ph,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWRR<'_,SuperSizeFn<Ph,P,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResolveResult<'w,'_,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWRR {
                    idx: r.idx,
                    child_id: r.child_id,
                    resolvus: r.resolvus,
                    invoke: &mut move |path,stack,cache,root,ctx|
                        r.widget.size(path,stack,stupid_widget_cache_cast(cache),root,ctx),
                }),
                root,ctx
            )
        } else {
            self.0(None,root,ctx)
        }
    }
}

type SuperRenderFn<'w,Ph,P,WC,E> = &'w mut (dyn FnMut(&Ph,&P,&mut ERenderer<'_,E>,bool,&mut WC,<E as Env>::RootRef<'_>,&mut <E as Env>::Context<'_>) + 'w);

pub struct AsWidgetsAllRender<P,Ph,C,WW,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P))>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(usize,WW::ChildID,SuperRenderFn<Ph,P,WW::WidgetCache,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,E> AsWidgetsIndexedDispatch<WW::ChildID,E> for AsWidgetsAllRender<P,Ph,C,WW,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(usize,WW::ChildID,SuperRenderFn<Ph,P,WW::WidgetCache,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, idx: usize, child_id: WW::ChildID, widget: &'w W, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w
    {
        self.0(
            idx,
            child_id,
            &mut move |path,stack,renderer,force,cache,root,ctx|
                widget.render(path,stack,renderer,force,stupid_widget_cache_cast(cache),root,ctx),
            root, ctx
        )
    }
}

pub struct AsWidgetsRender<P,Ph,C,WW,R,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P)->R)>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWR<SuperRenderFn<Ph,P,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,R,E> AsWidgetsDispatch<WW::ChildID,R,E> for AsWidgetsRender<P,Ph,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWR<SuperRenderFn<Ph,P,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResult<'w,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWR {
                    idx: r.idx,
                    child_id: r.child_id,
                    invoke: &mut move |path,stack,renderer,force,cache,root,ctx|
                        r.widget.render(path,stack,renderer,force,stupid_widget_cache_cast(cache),root,ctx),
                    _p: PhantomData
                }),
                root,ctx
            )
        } else {
            self.0(None,root,ctx)
        }
    }
}
pub struct AsWidgetsResolveRender<P,Ph,C,WW,R,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P)->R)>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWRR<'_,SuperRenderFn<Ph,P,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,R,E> AsWidgetsResolveDispatch<WW::ChildID,R,E> for AsWidgetsResolveRender<P,Ph,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWRR<'_,SuperRenderFn<Ph,P,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResolveResult<'w,'_,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWRR {
                    idx: r.idx,
                    child_id: r.child_id,
                    resolvus: r.resolvus,
                    invoke: &mut move |path,stack,renderer,force,cache,root,ctx|
                        r.widget.render(path,stack,renderer,force,stupid_widget_cache_cast(cache),root,ctx),
                }),
                root,ctx
            )
        } else {
            self.0(None,root,ctx)
        }
    }
}

type SuperEventFn<'w,Ph,P,Evt,WC,E> = &'w mut (dyn FnMut(&Ph,&P,&Evt,Option<&(dyn PathResolvusDyn<E>+'_)>,&mut WC,<E as Env>::RootRef<'_>,&mut <E as Env>::Context<'_>) -> EventResp + 'w);

pub struct AsWidgetsAllEvent<P,Ph,Evt,C,WW,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P,Evt))>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(usize,WW::ChildID,SuperEventFn<Ph,P,Evt,WW::WidgetCache,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,Evt,C,WW,E> AsWidgetsIndexedDispatch<WW::ChildID,E> for AsWidgetsAllEvent<P,Ph,Evt,C,WW,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(usize,WW::ChildID,SuperEventFn<Ph,P,Evt,WW::WidgetCache,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, idx: usize, child_id: WW::ChildID, widget: &'w W, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w
    {
        self.0(
            idx,
            child_id,
            &mut move |path,stack,event,route_to_widget,cache,root,ctx|
                widget.event_direct(path,stack,event,route_to_widget,stupid_widget_cache_cast(cache),root,ctx),
            root, ctx
        )
    }
}

pub struct AsWidgetsEvent<P,Ph,Evt,C,WW,R,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P,Evt)->R)>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWR<SuperEventFn<Ph,P,Evt,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,Evt,C,WW,R,E> AsWidgetsDispatch<WW::ChildID,R,E> for AsWidgetsEvent<P,Ph,Evt,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWR<SuperEventFn<Ph,P,Evt,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResult<'w,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWR {
                    idx: r.idx,
                    child_id: r.child_id,
                    invoke: &mut move |path,stack,event,route_to_widget,cache,root,ctx|
                        r.widget.event_direct(path,stack,event,route_to_widget,stupid_widget_cache_cast(cache),root,ctx),
                    _p: PhantomData
                }),
                root,ctx
            )
        } else {
            self.0(None,root,ctx)
        }
    }
}

pub struct AsWidgetsResolveEvent<P,Ph,Evt,C,WW,R,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P,Evt)->R)>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWRR<'_,SuperEventFn<Ph,P,Evt,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,Evt,C,WW,R,E> AsWidgetsResolveDispatch<WW::ChildID,R,E> for AsWidgetsResolveEvent<P,Ph,Evt,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWRR<'_,SuperEventFn<Ph,P,Evt,WW::WidgetCache,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, result: Option<AsWidgetsResolveResult<'w,'_,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWRR {
                    idx: r.idx,
                    child_id: r.child_id,
                    resolvus: r.resolvus,
                    invoke: &mut move |path,stack,event,route_to_widget,cache,root,ctx|
                        r.widget.event_direct(path,stack,event,route_to_widget,stupid_widget_cache_cast(cache),root,ctx),
                }),
                root,ctx
            )
        } else {
            self.0(None,root,ctx)
        }
    }
}
