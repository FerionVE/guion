use std::marker::PhantomData;

use crate::dispatchor::AsWidgetDispatch;
use crate::env::Env;
use crate::error::ResolveResult;
use crate::widget::as_widget::AsWidget;
use crate::widget::cache::DynWidgetCache;
use crate::widget::dyn_tunnel::WidgetDyn;

use super::{View, box_view_cb};
use super::mut_target::MuTarget;
use super::mutor_trait::{MutorToBuilder, ForTargetCBIfBuilder, ForTargetCBBuilder, MutorToBuilderExt};

pub struct ViewWidget<ViewTy,ViewFn,MutorFn,E>(ViewFn,MutorFn,PhantomData<(fn()->ViewTy,E)>) where
    ViewFn: Fn()->ViewTy,
    ViewTy: View<E>,
    MutorFn: MutorToBuilder<(),ViewTy::Mutarget,E>,
    E: Env;

impl<ViewTy,ViewFn,MutorFn,E> AsWidget<E> for ViewWidget<ViewTy,ViewFn,MutorFn,E> where
    ViewFn: Fn()->ViewTy,
    ViewTy: View<E>,
    MutorFn: MutorToBuilder<(),ViewTy::Mutarget,E>,
    E: Env,
{
    type Widget<'v,'z2> = dyn WidgetDyn<E> + 'v where 'z2: 'v, Self: 'z2;
    type WidgetCache = DynWidgetCache<E>;

    #[inline]
    fn with_widget<'w,R>(&self, dispatch: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let s = (self.0)();
        let mut dis = box_view_cb(move |widget,root,ctx| {
            dispatch.call(widget, root, ctx)
            //todo!()
        });
        s.view(&mut dis,self.1.erase(),root,ctx)
        //todo!()
    }
}

pub fn view_widget_cb<RightView,LeftArgs,LeftMutor,LeftTarget,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
    RightView,
    RightViewFn,
    ForTargetCBBuilder<LeftMutor,LeftArgs,LeftTarget,(),RightView::Mutarget,MutorFn,E>,
    E
    >
where
    E: Env,
    RightViewFn: Fn()->RightView,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E>,
    LeftTarget: MuTarget<E> + ?Sized,
    RightView: View<E>,
    MutorFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut <RightView::Mutarget as MuTarget<E>>::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
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

// pub fn view_widget_ret<RightView,LeftArgs,LeftMutor,LeftTarget,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
//     RightView,
//     RightViewFn,
//     MutorForViewRet<LeftMutor,LeftArgs,LeftTarget,RightView::Mutarget,(),MutorFn,E>,
//     E
//     >
// where
//     E: Env,
//     RightViewFn: Fn()->RightView,
//     LeftArgs: Clone + Sized + Send + Sync + 'static,
//     LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//     LeftTarget: MuTarget<E> + ?Sized,
//     RightView: View<E>,
//     for<'a> <RightView::Mutarget as MuTarget<E>>::Mutable<'a>: Sized,
//     MutorFn: for<'s,'ss,'c,'cc> Fn(
//         ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
//         (),
//         &'c mut E::Context<'cc>
//     ) -> ResolveResult<<RightView::Mutarget as MuTarget<E>>::Mutable<'ss>> + Clone + Send + Sync + 'static
// {
//     ViewWidget(
//         v,
//         m.for_view_ret::<RightView::Mutarget,(),MutorFn>(a, f),
//         PhantomData
//     )
// }

// pub fn view_widget_ref<RightView,LeftArgs,LeftMutor,LeftTarget,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
//     RightView,
//     RightViewFn,
//     MutorForViewRef<LeftMutor,LeftArgs,LeftTarget,RightView::Mutarget,(),MutorFn,E>,
//     E
//     >
// where
//     E: Env,
//     RightViewFn: Fn()->RightView,
//     LeftArgs: Clone + Sized + Send + Sync + 'static,
//     LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//     LeftTarget: MuTarget<E> + ?Sized,
//     RightView: View<E>,
//     MutorFn: for<'s,'ss,'c,'cc> Fn(
//         ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
//         (),
//         &'c mut E::Context<'cc>
//     ) -> ResolveResult<&'s mut <RightView::Mutarget as MuTarget<E>>::Mutable<'ss>> + Clone + Send + Sync + 'static
// {
//     ViewWidget(
//         v,
//         m.for_view_ref::<RightView::Mutarget,(),MutorFn>(a, f),
//         PhantomData
//     )
// }

pub fn view_widget_cb_if<RightView,LeftArgs,LeftMutor,LeftTarget,MutorFn,RightViewFn,E>(view_fn: RightViewFn, left_mutor: LeftMutor, left_arg: LeftArgs, right_fn: MutorFn) -> ViewWidget<
    RightView,
    RightViewFn,
    ForTargetCBIfBuilder<LeftMutor,LeftArgs,LeftTarget,(),RightView::Mutarget,MutorFn,E>,
    E
    >
where
    E: Env,
    RightViewFn: Fn()->RightView,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E>,
    LeftTarget: MuTarget<E> + ?Sized,
    RightView: View<E>,
    MutorFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut <RightView::Mutarget as MuTarget<E>>::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        (),
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    ViewWidget(
        view_fn,
        left_mutor.for_view_cb_if::<RightView::Mutarget,(),MutorFn>(left_arg, right_fn),
        PhantomData
    )
}

// pub fn view_widget_cb_if_dyn<RightView,LeftArgs,LeftMutor,LeftTarget,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
//     RightView,
//     RightViewFn,
//     Box<dyn MutorToDyn<(),RightView::Mutarget,E>+'static>,
//     E
//     >
// where
//     E: Env,
//     RightViewFn: Fn()->RightView,
//     LeftArgs: Clone + Sized + Send + Sync + 'static,
//     LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//     LeftTarget: MuTarget<E> + ?Sized,
//     RightView: View<E>,
//     MutorFn: for<'s,'ss,'c,'cc> Fn(
//         &'s mut LeftTarget::Mutable<'ss>,
//         &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut <RightView::Mutarget as MuTarget<E>>::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
//         (),
//         &'c mut E::Context<'cc>
//     ) + Clone + Send + Sync + 'static
// {
//     ViewWidget(
//         v,
//         Box::new(m.for_view_cb_if::<RightView::Mutarget,(),MutorFn>(a,f)),
//         PhantomData
//     )
// }

// pub fn view_widget_ret_if<RightView,LeftArgs,LeftMutor,LeftTarget,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
//     RightView,
//     RightViewFn,
//     MutorForViewRetIf<LeftMutor,LeftArgs,LeftTarget,RightView::Mutarget,(),MutorFn,E>,
//     E
//     >
// where
//     E: Env,
//     RightViewFn: Fn()->RightView,
//     LeftArgs: Clone + Sized + Send + Sync + 'static,
//     LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//     LeftTarget: MuTarget<E> + ?Sized,
//     RightView: View<E>,
//     for<'a> <RightView::Mutarget as MuTarget<E>>::Mutable<'a>: Sized,
//     MutorFn: for<'s,'ss,'c,'cc> Fn(
//         &'s mut LeftTarget::Mutable<'ss>,
//         (),
//         &'c mut E::Context<'cc>
//     ) -> ResolveResult<<RightView::Mutarget as MuTarget<E>>::Mutable<'ss>> + Clone + Send + Sync + 'static
// {
//     ViewWidget(
//         v,
//         m.for_view_ret_if::<RightView::Mutarget,(),MutorFn>(a, f),
//         PhantomData
//     )
// }

// pub fn view_widget_ret_if_2<RightTarget,RightView,LeftArgs,LeftMutor,LeftTarget,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
//     RightView,
//     RightViewFn,
//     MutorForViewRetIf<LeftMutor,LeftArgs,LeftTarget,RightTarget,(),MutorFn,E>,
//     E
//     >
// where
//     E: Env,
//     RightViewFn: Fn()->RightView,
//     LeftArgs: Clone + Sized + Send + Sync + 'static,
//     LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//     LeftTarget: MuTarget<E> + ?Sized,
//     RightView: View<E,Mutarget=RightTarget>,
//     RightTarget: MuTarget<E> + ?Sized,
//     for<'a> RightTarget::Mutable<'a>: Sized,
//     MutorFn: for<'s,'ss,'c,'cc> Fn(
//         &'s mut LeftTarget::Mutable<'ss>,
//         (),
//         &'c mut E::Context<'cc>
//     ) -> ResolveResult<RightTarget::Mutable<'ss>> + Clone + Send + Sync + 'static
// {
//     ViewWidget(
//         v,
//         m.for_view_ret_if::<RightTarget,(),MutorFn>(a, f),
//         PhantomData
//     )
// }

// pub fn view_widget_ref_if<RightView,LeftArgs,LeftMutor,LeftTarget,MutorFn,RightViewFn,E>(v: RightViewFn, a: LeftArgs, m: LeftMutor, f: MutorFn) -> ViewWidget<
//     RightView,
//     RightViewFn,
//     MutorForViewRefIf<LeftMutor,LeftArgs,LeftTarget,RightView::Mutarget,(),MutorFn,E>,
//     E
//     >
// where
//     E: Env,
//     RightViewFn: Fn()->RightView,
//     LeftArgs: Clone + Sized + Send + Sync + 'static,
//     LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//     LeftTarget: MuTarget<E> + ?Sized,
//     RightView: View<E>,
//     MutorFn: for<'s,'ss,'c,'cc> Fn(
//         &'s mut LeftTarget::Mutable<'ss>,
//         (),
//         &'c mut E::Context<'cc>
//     ) -> ResolveResult<&'s mut <RightView::Mutarget as MuTarget<E>>::Mutable<'ss>> + Clone + Send + Sync + 'static
// {
//     ViewWidget(
//         v,
//         m.for_view_ref_if::<RightView::Mutarget,(),MutorFn>(a, f),
//         PhantomData
//     )
// }
