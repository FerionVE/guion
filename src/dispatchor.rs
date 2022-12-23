use std::marker::PhantomData;

use crate::aliases::{ERenderer, ESize};
use crate::env::Env;
use crate::util::tabulate::{TabulateDirection, TabulateOrigin, TabulateResponse};
use crate::{event_new, EventResp};
use crate::newpath::{PathResolvusDyn, PathFragment, PathStack};
use crate::queron::Queron;
use crate::widget::Widget;
use crate::widget::as_widgets::AsWidgets;
use crate::widget::dyn_tunnel::WidgetDyn;

pub trait AsWidgetsDispatch<CID,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(self, result: Option<AsWidgetsResult<'w,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w, Self: Sized;

    fn call_none(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R where Self: Sized {
        self.call::<std::convert::Infallible>(None,root,ctx)
    }
}

pub trait AsWidgetsDispatchMut<CID,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(self, result: Option<AsWidgetsResultMut<'w,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w, Self: Sized;

    fn call_none(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R where Self: Sized {
        self.call::<std::convert::Infallible>(None,root,ctx)
    }
}

pub struct AsWidgetsResult<'w,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: isize,
    pub child_id: CID,
    pub widget: &'w W,
    _p: PhantomData<E>,
}

pub struct AsWidgetsResultMut<'w,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: isize,
    pub child_id: CID,
    pub widget: &'w mut W,
    _p: PhantomData<E>,
}

pub trait AsWidgetsCDispatch<CID,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(self, result: Option<AsWidgetsCResult<'w,'_,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w, Self: Sized;

    fn call_none(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R where Self: Sized {
        self.call::<std::convert::Infallible>(None,root,ctx)
    }
}

pub trait AsWidgetsCDispatchMut<CID,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(self, result: Option<AsWidgetsCResultMut<'w,'_,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w, Self: Sized;

    fn call_none(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R where Self: Sized {
        self.call::<std::convert::Infallible>(None,root,ctx)
    }
}

pub struct AsWidgetsCResult<'w,'c,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: isize,
    pub child_id: CID,
    pub widget: &'w W,
    pub cache: &'c mut W::Cache,
    _p: PhantomData<E>,
}

pub struct AsWidgetsCResultMut<'w,'c,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: isize,
    pub child_id: CID,
    pub widget: &'w mut W,
    pub cache: &'c mut W::Cache,
    _p: PhantomData<E>,
}

impl<'w,W,CID,E> AsWidgetsResult<'w,W,CID,E>
where
    W: Widget<E> + ?Sized,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub fn from_some(idx: isize, child_id: CID, widget: &'w W) -> Option<Self> {
        Some(Self { idx, child_id, widget, _p: PhantomData })
    }
}

impl<'w,W,CID,E> AsWidgetsResultMut<'w,W,CID,E>
where
    W: Widget<E> + ?Sized,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub fn from_some(idx: isize, child_id: CID, widget: &'w mut W) -> Option<Self> {
        Some(Self { idx, child_id, widget, _p: PhantomData })
    }
}

impl<'w,'c,W,CID,E> AsWidgetsCResult<'w,'c,W,CID,E>
where
    W: Widget<E> + ?Sized,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub fn from_some(idx: isize, child_id: CID, widget: &'w W, cache: &'c mut W::Cache) -> Option<Self> {
        Some(Self { idx, child_id, widget, cache, _p: PhantomData })
    }
}

impl<'w,'c,W,CID,E> AsWidgetsCResultMut<'w,'c,W,CID,E>
where
    W: Widget<E> + ?Sized,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub fn from_some(idx: isize, child_id: CID, widget: &'w mut W, cache: &'c mut W::Cache) -> Option<Self> {
        Some(Self { idx, child_id, widget, cache, _p: PhantomData })
    }
}


pub trait AsWidgetsResolveDispatch<CID,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(self, result: Option<AsWidgetsResolveResult<'w,'_,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w, Self: Sized;
    
    fn call_none(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R where Self: Sized {
        self.call::<std::convert::Infallible>(None,root,ctx)
    }
}

pub trait AsWidgetsResolveDispatchMut<CID,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(self, result: Option<AsWidgetsResolveResultMut<'w,'_,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w, Self: Sized;
    
    fn call_none(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R where Self: Sized {
        self.call::<std::convert::Infallible>(None,root,ctx)
    }
}

pub struct AsWidgetsResolveResult<'w,'p,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: isize,
    pub child_id: CID,
    pub resolvus: &'p (dyn PathResolvusDyn<E>+'p),
    pub widget: &'w W,
}

pub struct AsWidgetsResolveResultMut<'w,'p,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: isize,
    pub child_id: CID,
    pub resolvus: &'p (dyn PathResolvusDyn<E>+'p),
    pub widget: &'w mut W,
}

impl<'w,'p,W,CID,E> AsWidgetsResolveResult<'w,'p,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub fn from_some(idx: isize, child_id: CID, resolvus: &'p (dyn PathResolvusDyn<E>+'p), widget: &'w W) -> Option<Self> {
        Some(Self { idx, child_id, resolvus, widget })
    }
}

impl<'w,'p,W,CID,E> AsWidgetsResolveResultMut<'w,'p,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub fn from_some(idx: isize, child_id: CID, resolvus: &'p (dyn PathResolvusDyn<E>+'p), widget: &'w mut W) -> Option<Self> {
        Some(Self { idx, child_id, resolvus, widget })
    }
}

pub trait AsWidgetsResolveCDispatch<CID,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(self, result: Option<AsWidgetsResolveCResult<'w,'_,'_,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w, Self: Sized;
    
    fn call_none(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R where Self: Sized {
        self.call::<std::convert::Infallible>(None,root,ctx)
    }
}

pub trait AsWidgetsResolveCDispatchMut<CID,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(self, result: Option<AsWidgetsResolveCResultMut<'w,'_,'_,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w, Self: Sized;
    
    fn call_none(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R where Self: Sized {
        self.call::<std::convert::Infallible>(None,root,ctx)
    }
}

pub struct AsWidgetsResolveCResult<'w,'p,'c,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: isize,
    pub child_id: CID,
    pub resolvus: &'p (dyn PathResolvusDyn<E>+'p),
    pub widget: &'w W,
    pub cache: &'c mut W::Cache,
}

pub struct AsWidgetsResolveCResultMut<'w,'p,'c,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub idx: isize,
    pub child_id: CID,
    pub resolvus: &'p (dyn PathResolvusDyn<E>+'p),
    pub widget: &'w mut W,
    pub cache: &'c mut W::Cache,
}

impl<'w,'p,'c,W,CID,E> AsWidgetsResolveCResult<'w,'p,'c,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub fn from_some(idx: isize, child_id: CID, resolvus: &'p (dyn PathResolvusDyn<E>+'p), widget: &'w W, cache: &'c mut W::Cache) -> Option<Self> {
        Some(Self { idx, child_id, resolvus, widget, cache })
    }
}

impl<'w,'p,'c,W,CID,E> AsWidgetsResolveCResultMut<'w,'p,'c,W,CID,E>
where
    W: Widget<E> + ?Sized + 'w,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    pub fn from_some(idx: isize, child_id: CID, resolvus: &'p (dyn PathResolvusDyn<E>+'p), widget: &'w mut W, cache: &'c mut W::Cache) -> Option<Self> {
        Some(Self { idx, child_id, resolvus, widget, cache })
    }
}

pub trait AsWidgetsIndexedDispatch<CID,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(&mut self, idx: isize, child_id: CID, widget: &'w W, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w;
}

pub trait AsWidgetsIndexedDispatchMut<CID,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(&mut self, idx: isize, child_id: CID, widget: &'w mut W, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w;
}

pub trait AsWidgetsIndexedCDispatch<CID,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(&mut self, idx: isize, child_id: CID, widget: &'w W, cache: &mut W::Cache, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w;
}

pub trait AsWidgetsIndexedCDispatchMut<CID,E>
where
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    fn call<'w,W>(&mut self, idx: isize, child_id: CID, widget: &'w mut W, cache: &mut W::Cache, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w;
}



pub struct AsWidgetsOnWithChild<C,R,E>(pub C, pub PhantomData<(E,fn()->R)>)
where
    C: for<'www,'ww,'c,'cc> FnOnce(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R,
    E: Env;

impl<CID,C,R,E> AsWidgetsDispatch<CID,R,E> for AsWidgetsOnWithChild<C,R,E>
where
    CID: PathFragment<E> + Clone + 'static,
    C: for<'www,'ww,'c,'cc> FnOnce(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R,
    E: Env
{
    fn call<'w,W>(self, result: Option<AsWidgetsResult<'w,W,CID,E>>, _: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w, Self: Sized
    {
        if let Some(r) = result {
            (self.0)(Ok(r.widget.erase()),ctx)
        } else {
            (self.0)(Err(()),ctx)
        }
    }
}

// pub struct AsWidgetsOnWithResolveChild<C,R,E>(pub Option<C>, pub PhantomData<(E,fn()->R)>)
// where
//     C: for<'a,'c,'cc> FnOnce(Result<WidgetWithResolveChildDyn<'a,E>,E::Error>,&'c mut E::Context<'cc>) -> R,
//     E: Env;

// impl<CID,C,R,E> AsWidgetsResolveDispatch<CID,R,E> for AsWidgetsOnWithResolveChild<C,R,E>
// where
//     CID: PathFragment<E> + Clone + 'static,
//     C: for<'a,'c,'cc> FnOnce(Result<WidgetWithResolveChildDyn<'a,E>,E::Error>,&'c mut E::Context<'cc>) -> R,
//     E: Env
// {
//     fn call<'w,W>(&mut self, result: Option<AsWidgetsResolveResult<'w,'_,W,CID,E>>, _: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
//     where
//         W: Widget<E> +  ?Sized + 'w
//     {
//         let f = self.0.take().unwrap();

//         if let Some(r) = result {
//             let d = WidgetWithResolveChildDyn {
//                 idx: r.idx,
//                 sub_path: r.resolvus,
//                 widget: r.widget.erase(),
//             };

//             f(Ok(d),ctx)
//         } else {
//             f(Err(todo!()),ctx)
//         }
//     }
// }

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
    fn call<'w,W>(self, result: Option<AsWidgetsResult<'w,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<Result<TabulateResponse<E>,E::Error>>
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
    fn call<'w,W>(self, result: Option<AsWidgetsResolveResult<'w,'_,W,CID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Option<Result<TabulateResponse<E>,E::Error>>
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
    pub idx: isize,
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
    pub idx: isize,
    pub child_id: CID,
    pub resolvus: &'p (dyn PathResolvusDyn<E>+'p),
    pub invoke: W,
}

type SuperSizeFn<'w,Ph,P,E> = &'w mut (dyn FnMut(&Ph,&P,<E as Env>::RootRef<'_>,&mut <E as Env>::Context<'_>)->ESize<E> + 'w);

pub struct AsWidgetsAllSize<P,Ph,C,WW,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P))>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(isize,WW::ChildID,SuperSizeFn<Ph,P,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,E> AsWidgetsIndexedDispatch<WW::ChildID,E> for AsWidgetsAllSize<P,Ph,C,WW,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(isize,WW::ChildID,SuperSizeFn<Ph,P,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, idx: isize, child_id: WW::ChildID, widget: &'w W, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w
    {
        self.0(
            idx,
            child_id,
            &mut move |path,stack,root,ctx|
                widget.size(path,stack,root,ctx),
            root, ctx
        )
    }
}

pub struct AsWidgetsSize<P,Ph,C,WW,R,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P)->R)>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnOnce(Option<SuperAWR<SuperSizeFn<Ph,P,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,R,E> AsWidgetsDispatch<WW::ChildID,R,E> for AsWidgetsSize<P,Ph,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnOnce(Option<SuperAWR<SuperSizeFn<Ph,P,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(self, result: Option<AsWidgetsResult<'w,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWR {
                    idx: r.idx,
                    child_id: r.child_id,
                    invoke: &mut move |path,stack,root,ctx|
                        r.widget.size(path,stack,root,ctx),
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
    C: for<'r,'c,'cc> FnOnce(Option<SuperAWRR<'_,SuperSizeFn<Ph,P,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,R,E> AsWidgetsResolveDispatch<WW::ChildID,R,E> for AsWidgetsResolveSize<P,Ph,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnOnce(Option<SuperAWRR<'_,SuperSizeFn<Ph,P,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(self, result: Option<AsWidgetsResolveResult<'w,'_,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWRR {
                    idx: r.idx,
                    child_id: r.child_id,
                    resolvus: r.resolvus,
                    invoke: &mut move |path,stack,root,ctx|
                        r.widget.size(path,stack,root,ctx),
                }),
                root,ctx
            )
        } else {
            self.0(None,root,ctx)
        }
    }
}

type SuperRenderFn<'w,Ph,P,E> = &'w mut (dyn FnMut(&Ph,&P,&mut ERenderer<'_,E>,bool,<E as Env>::RootRef<'_>,&mut <E as Env>::Context<'_>) + 'w);

pub struct AsWidgetsAllRender<P,Ph,C,WW,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P))>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(isize,WW::ChildID,SuperRenderFn<Ph,P,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,E> AsWidgetsIndexedCDispatch<WW::ChildID,E> for AsWidgetsAllRender<P,Ph,C,WW,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(isize,WW::ChildID,SuperRenderFn<Ph,P,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, idx: isize, child_id: WW::ChildID, widget: &'w W, cache: &mut W::Cache, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w
    {
        self.0(
            idx,
            child_id,
            &mut move |path,stack,renderer,force,root,ctx|
                widget.render(path,stack,renderer,force,cache,root,ctx),
            root, ctx
        )
    }
}

pub struct AsWidgetsRender<P,Ph,C,WW,R,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P)->R)>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnOnce(Option<SuperAWR<SuperRenderFn<Ph,P,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,R,E> AsWidgetsCDispatch<WW::ChildID,R,E> for AsWidgetsRender<P,Ph,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnOnce(Option<SuperAWR<SuperRenderFn<Ph,P,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(self, result: Option<AsWidgetsCResult<'w,'_,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWR {
                    idx: r.idx,
                    child_id: r.child_id,
                    invoke: &mut move |path,stack,renderer,force,root,ctx|
                        r.widget.render(path,stack,renderer,force,r.cache,root,ctx),
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
    C: for<'r,'c,'cc> FnOnce(Option<SuperAWRR<'_,SuperRenderFn<Ph,P,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,C,WW,R,E> AsWidgetsResolveCDispatch<WW::ChildID,R,E> for AsWidgetsResolveRender<P,Ph,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    C: for<'r,'c,'cc> FnOnce(Option<SuperAWRR<'_,SuperRenderFn<Ph,P,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(self, result: Option<AsWidgetsResolveCResult<'w,'_,'_,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWRR {
                    idx: r.idx,
                    child_id: r.child_id,
                    resolvus: r.resolvus,
                    invoke: &mut move |path,stack,renderer,force,root,ctx|
                        r.widget.render(path,stack,renderer,force,r.cache,root,ctx),
                }),
                root,ctx
            )
        } else {
            self.0(None,root,ctx)
        }
    }
}

type SuperEventFn<'w,Ph,P,Evt,E> = &'w mut (dyn FnMut(&Ph,&P,&Evt,Option<&(dyn PathResolvusDyn<E>+'_)>,<E as Env>::RootRef<'_>,&mut <E as Env>::Context<'_>) -> EventResp + 'w);

pub struct AsWidgetsAllEvent<P,Ph,Evt,C,WW,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P,Evt))>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(isize,WW::ChildID,SuperEventFn<Ph,P,Evt,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,Evt,C,WW,E> AsWidgetsIndexedDispatch<WW::ChildID,E> for AsWidgetsAllEvent<P,Ph,Evt,C,WW,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(isize,WW::ChildID,SuperEventFn<Ph,P,Evt,E>,E::RootRef<'r>,&'c mut E::Context<'cc>),
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(&mut self, idx: isize, child_id: WW::ChildID, widget: &'w W, root: E::RootRef<'_>, ctx: &'_ mut E::Context<'_>)
    where
        W: Widget<E> + ?Sized + 'w
    {
        self.0(
            idx,
            child_id,
            &mut move |path,stack,event,route_to_widget,root,ctx|
                widget.event_direct(path,stack,event,route_to_widget,root,ctx),
            root, ctx
        )
    }
}

pub struct AsWidgetsEvent<P,Ph,Evt,C,WW,R,E>(pub C, pub PhantomData<(E,fn(WW::ChildID,Ph,P,Evt)->R)>)
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWR<SuperEventFn<Ph,P,Evt,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,Evt,C,WW,R,E> AsWidgetsDispatch<WW::ChildID,R,E> for AsWidgetsEvent<P,Ph,Evt,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnMut(Option<SuperAWR<SuperEventFn<Ph,P,Evt,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(mut self, result: Option<AsWidgetsResult<'w,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWR {
                    idx: r.idx,
                    child_id: r.child_id,
                    invoke: &mut move |path,stack,event,route_to_widget,root,ctx|
                        r.widget.event_direct(path,stack,event,route_to_widget,root,ctx),
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
    C: for<'r,'c,'cc> FnOnce(Option<SuperAWRR<'_,SuperEventFn<Ph,P,Evt,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env;

impl<P,Ph,Evt,C,WW,R,E> AsWidgetsResolveDispatch<WW::ChildID,R,E> for AsWidgetsResolveEvent<P,Ph,Evt,C,WW,R,E>
where
    Ph: PathStack<E> + ?Sized,
    P: Queron<E> + ?Sized,
    Evt: event_new::Event<E> + ?Sized,
    C: for<'r,'c,'cc> FnOnce(Option<SuperAWRR<'_,SuperEventFn<Ph,P,Evt,E>,WW::ChildID,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    WW: AsWidgets<E> + ?Sized,
    E: Env
{
    fn call<'w,W>(self, result: Option<AsWidgetsResolveResult<'w,'_,W,WW::ChildID,E>>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        W: Widget<E> +  ?Sized + 'w
    {
        if let Some(r) = result {
            self.0(
                Some(SuperAWRR {
                    idx: r.idx,
                    child_id: r.child_id,
                    resolvus: r.resolvus,
                    invoke: &mut move |path,stack,event,route_to_widget,root,ctx|
                        r.widget.event_direct(path,stack,event,route_to_widget,root,ctx),
                }),
                root,ctx
            )
        } else {
            self.0(None,root,ctx)
        }
    }
}
