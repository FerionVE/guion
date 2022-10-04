use std::marker::PhantomData;

use crate::dispatchor::{AsWidgetDispatch, ViewClosure, AsWidgetClosure};
use crate::env::Env;
use crate::error::ResolveResult;
use crate::root::RootRef;
use crate::widget::Widget;
use crate::widget::as_widget::AsWidget;

use super::View;
use super::mut_target::MuTarget;
use super::mutor_trait::*;

pub struct ViewWidget<ViewTy,ViewFn,MutorFn,E>(ViewFn,MutorFn,PhantomData<(ViewTy,E)>) where
    ViewFn: Fn()->ViewTy,
    ViewTy: View<E>,
    MutorFn: MutorTo<(),E,Target=ViewTy::Mutarget>,
    E: Env;

impl<ViewTy,ViewFn,MutorFn,E> AsWidget<E> for ViewWidget<ViewTy,ViewFn,MutorFn,E> where
    ViewFn: Fn()->ViewTy,
    ViewTy: View<E>,
    MutorFn: MutorTo<(),E,Target=ViewTy::Mutarget>,
    E: Env,
{
    type Widget<'v,'z2> = ViewTy::Viewed<'v,'z2,MutorFn> where 'z2: 'v, Self: 'z2;
    type WidgetCache = ViewTy::WidgetCache;

    #[inline]
    fn with_widget<'w,F,R>(&self, dispatch: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetDispatch<'w,Self,R,E>, Self: 'w
    {
        let s = (self.0)();
        let dis = ViewClosure::<'_,_,ViewTy,_,_,_>::new(move |widget,root,ctx| {
            dispatch.call(widget, root, ctx)
            //todo!()
        });
        s.view(dis,self.1.clone(),root,ctx)
        //todo!()
    }
}

pub fn view_widget_cb<RightView,LeftArgs,LeftMutor,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
    RightView,
    RightViewFn,
    MutorForViewCB<LeftMutor,LeftArgs,RightView::Mutarget,(),MutorFn,E>,
    E
    >
where
    E: Env,
    RightViewFn: Fn()->RightView,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    LeftMutor: MutorTo<LeftArgs,E>,
    RightView: View<E>,
    MutorFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut <LeftMutor::Target as MuTarget<E>>::Mutable<'ss>>,&'ss (),
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut <RightView::Mutarget as MuTarget<E>>::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
        (),
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    ViewWidget(
        v,
        m.for_view_cb::<RightView::Mutarget,(),MutorFn>(a, f),
        PhantomData
    )
}

pub fn view_widget_ret<RightView,LeftArgs,LeftMutor,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
    RightView,
    RightViewFn,
    MutorForViewRet<LeftMutor,LeftArgs,RightView::Mutarget,(),MutorFn,E>,
    E
    >
where
    E: Env,
    RightViewFn: Fn()->RightView,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    LeftMutor: MutorTo<LeftArgs,E>,
    RightView: View<E>,
    for<'a> <RightView::Mutarget as MuTarget<E>>::Mutable<'a>: Sized,
    MutorFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut <LeftMutor::Target as MuTarget<E>>::Mutable<'ss>>,&'ss (),
        (),
        &'c mut E::Context<'cc>
    ) -> ResolveResult<<RightView::Mutarget as MuTarget<E>>::Mutable<'ss>> + Clone + Send + Sync + 'static
{
    ViewWidget(
        v,
        m.for_view_ret::<RightView::Mutarget,(),MutorFn>(a, f),
        PhantomData
    )
}

pub fn view_widget_ref<RightView,LeftArgs,LeftMutor,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
    RightView,
    RightViewFn,
    MutorForViewRef<LeftMutor,LeftArgs,RightView::Mutarget,(),MutorFn,E>,
    E
    >
where
    E: Env,
    RightViewFn: Fn()->RightView,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    LeftMutor: MutorTo<LeftArgs,E>,
    RightView: View<E>,
    MutorFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut <LeftMutor::Target as MuTarget<E>>::Mutable<'ss>>,&'ss (),
        (),
        &'c mut E::Context<'cc>
    ) -> ResolveResult<&'s mut <RightView::Mutarget as MuTarget<E>>::Mutable<'ss>> + Clone + Send + Sync + 'static
{
    ViewWidget(
        v,
        m.for_view_ref::<RightView::Mutarget,(),MutorFn>(a, f),
        PhantomData
    )
}

pub fn view_widget_cb_if<RightView,LeftArgs,LeftMutor,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
    RightView,
    RightViewFn,
    MutorForViewCBIf<LeftMutor,LeftArgs,RightView::Mutarget,(),MutorFn,E>,
    E
    >
where
    E: Env,
    RightViewFn: Fn()->RightView,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    LeftMutor: MutorTo<LeftArgs,E>,
    RightView: View<E>,
    MutorFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut <LeftMutor::Target as MuTarget<E>>::Mutable<'ss>,&'ss (),
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut <RightView::Mutarget as MuTarget<E>>::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
        (),
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    ViewWidget(
        v,
        m.for_view_cb_if::<RightView::Mutarget,(),MutorFn>(a, f),
        PhantomData
    )
}

pub fn view_widget_ret_if<RightView,LeftArgs,LeftMutor,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
    RightView,
    RightViewFn,
    MutorForViewRetIf<LeftMutor,LeftArgs,RightView::Mutarget,(),MutorFn,E>,
    E
    >
where
    E: Env,
    RightViewFn: Fn()->RightView,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    LeftMutor: MutorTo<LeftArgs,E>,
    RightView: View<E>,
    for<'a> <RightView::Mutarget as MuTarget<E>>::Mutable<'a>: Sized,
    MutorFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut <LeftMutor::Target as MuTarget<E>>::Mutable<'ss>,&'ss (),
        (),
        &'c mut E::Context<'cc>
    ) -> ResolveResult<<RightView::Mutarget as MuTarget<E>>::Mutable<'ss>> + Clone + Send + Sync + 'static
{
    ViewWidget(
        v,
        m.for_view_ret_if::<RightView::Mutarget,(),MutorFn>(a, f),
        PhantomData
    )
}

pub fn view_widget_ret_if_2<RightTarget,RightView,LeftArgs,LeftMutor,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
    RightView,
    RightViewFn,
    MutorForViewRetIf<LeftMutor,LeftArgs,RightTarget,(),MutorFn,E>,
    E
    >
where
    E: Env,
    RightViewFn: Fn()->RightView,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    LeftMutor: MutorTo<LeftArgs,E>,
    RightView: View<E,Mutarget=RightTarget>,
    RightTarget: MuTarget<E>,
    for<'a> RightTarget::Mutable<'a>: Sized,
    MutorFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut <LeftMutor::Target as MuTarget<E>>::Mutable<'ss>,&'ss (),
        (),
        &'c mut E::Context<'cc>
    ) -> ResolveResult<RightTarget::Mutable<'ss>> + Clone + Send + Sync + 'static
{
    ViewWidget(
        v,
        m.for_view_ret_if::<RightTarget,(),MutorFn>(a, f),
        PhantomData
    )
}

pub fn view_widget_ref_if<RightView,LeftArgs,LeftMutor,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
    RightView,
    RightViewFn,
    MutorForViewRefIf<LeftMutor,LeftArgs,RightView::Mutarget,(),MutorFn,E>,
    E
    >
where
    E: Env,
    RightViewFn: Fn()->RightView,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    LeftMutor: MutorTo<LeftArgs,E>,
    RightView: View<E>,
    MutorFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut <LeftMutor::Target as MuTarget<E>>::Mutable<'ss>,&'ss (),
        (),
        &'c mut E::Context<'cc>
    ) -> ResolveResult<&'s mut <RightView::Mutarget as MuTarget<E>>::Mutable<'ss>> + Clone + Send + Sync + 'static
{
    ViewWidget(
        v,
        m.for_view_ref_if::<RightView::Mutarget,(),MutorFn>(a, f),
        PhantomData
    )
}
