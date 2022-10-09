use std::marker::PhantomData;

use crate::ctx::queue::BoxMutEvent;
use crate::dispatchor::{AsWidgetDispatch};
use crate::env::Env;
use crate::error::ResolveResult;
use crate::widget::as_widget::AsWidget;

use super::View;
use super::mut_target::{MuTarget, RootMutTarget};

pub trait MutorEndBuilder<Args,E>: Send + Sync where E: Env, Args: Sized + Send + Sync + 'static {
    type Built: MutorEnd<Args,E> + Sized + Send + Sync + 'static;

    //fn _erase_builder(&self) -> &MutorEndBuilderDyn<'_,Args,E>;

    fn build(&self) -> Self::Built;

    #[inline]
    fn build_boxed(&self) -> Box<dyn MutorEnd<Args,E>> {
        Box::new(self.build())
    }
}

pub type MutorEndBuilderDyn<'a,Args,E> = dyn MutorEndBuilder<Args,E,Built=Box<dyn MutorEnd<Args,E>>> + 'a;

impl<Args,E> MutorEndBuilder<Args,E> for () where E: Env, Args: Sized + Send + Sync + 'static {
    type Built = ();
    // #[inline]
    // fn _erase_builder(&self) -> &MutorEndBuilderDyn<'_,Args,E> {
    //     self
    // }
    #[inline]
    fn build(&self) -> Self::Built {
        ()
    }
}

pub trait MutorEnd<Args,E>: Send + Sync + 'static where E: Env, Args: Sized + Send + Sync + 'static {
    fn with_mutor_end<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        args: Args,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c;

    // #[inline]
    // fn _boxed_end(&self) -> Box<dyn MutorEnd<Args,E>+'static> {
    //     Box::new(self.clone())
    // }
}

impl<Args,E> MutorEnd<Args,E> for () where E: Env, Args: Sized + Send + Sync + 'static {
    #[inline]
    fn with_mutor_end<'s,'c,'cc>(
        &self,
        _: E::RootMut<'s>,
        _: Args,
        _: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {}
}

pub trait MutorEndBuilderExt<Args,E>: MutorEndBuilder<Args,E> + Send + Sync where E: Env, Args: Sized + Send + Sync + 'static {
    #[inline]
    fn erase<'a>(&'a self) -> &'a BoxingMutorEndBuilder<Args,Self,E> {
        unsafe{std::mem::transmute::<&'a Self,&'a BoxingMutorEndBuilder<Args,Self,E>>(self)}
    }
}
impl<Args,T,E> MutorEndBuilderExt<Args,E> for T where T: MutorEndBuilder<Args,E> + Send + Sync + ?Sized, E: Env, Args: Sized + Send + Sync + 'static {}

pub trait MutorToBuilder<Args,Target,E>: Send + Sync where E: Env, Args: Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    type Built: MutorTo<Args,Target,E> + Sized + Send + Sync + 'static;

    //fn _erase_builder(&self) -> &MutorToBuilderDyn<'_,Args,Target,E>;

    fn build(&self) -> Self::Built;

    #[inline]
    fn build_boxed(&self) -> Box<dyn MutorTo<Args,Target,E>> {
        Box::new(self.build())
    }
}

pub type MutorToBuilderDyn<'a,Args,Target,E> = dyn MutorToBuilder<Args,Target,E,Built=Box<dyn MutorTo<Args,Target,E>>> + 'a;

pub trait MutorTo<Args,Target,E>: Send + Sync + 'static where E: Env, Args: Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
        args: Args,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c;

    // #[inline]
    // fn convert_to_target<T>(&self) -> ConvertToTarget<Self,Target,T,Args,E> where for<'a> T: MuTarget<E,Mutable<'a>=Target::Mutable<'a>> {
    //     ConvertToTarget(self.clone(),PhantomData)
    // }

    // #[inline]
    // fn _boxed_to(&self) -> Box<dyn MutorToDyn<Args,Target,E>+'static> {
    //     Box::new(self.clone())
    // }

    // fn for_view_cb<NewTarget,RightArgs,RightFn>(&self, larg: Args, fun: RightFn) -> MutorForTarget<NewTarget,RightArgs,MutorForViewCB<Self,Args,Target,NewTarget,RightArgs,RightFn,E>,E>
    // where
    //     E: Env,
    //     RightArgs: Sized + Send + Sync + 'static,
    //     NewTarget: MuTarget<E> + ?Sized,
    //     RightFn: for<'s,'ss,'c,'cc> Fn(
    //         ResolveResult<&'s mut <Target as MuTarget<E>>::Mutable<'ss>>,&'ss (),
    //         &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
    //         RightArgs,
    //         &'c mut E::Context<'cc>
    //     ) + Send + Sync + 'static
    // {
    //     let left = self.clone();
    //     MutorForTarget::<NewTarget,RightArgs,_,E>::new(
    //         #[inline] move |root,_,callback,rarg: RightArgs,ctx| {
    //             left.with_mutor_cb(
    //                 root,
    //                 &mut |med,_,ctx| {
    //                     (fun)(med,&(),callback,rarg.clone(),ctx)
    //                 },
    //                 larg.clone(),ctx
    //             )
    //         }
    //     )
    // }

    // fn for_view_ret<NewTarget,RightArgs,RightFn>(&self, larg: Args, fun: RightFn) -> MutorForTarget<NewTarget,RightArgs,MutorForViewRet<Self,Args,Target,NewTarget,RightArgs,RightFn,E>,E>
    // where
    //     E: Env,
    //     RightArgs: Sized + Send + Sync + 'static,
    //     NewTarget: MuTarget<E> + ?Sized,
    //     for<'a> NewTarget::Mutable<'a>: Sized,
    //     RightFn: for<'s,'ss,'c,'cc> Fn(
    //         ResolveResult<&'s mut <Target as MuTarget<E>>::Mutable<'ss>>,&'ss (),
    //         RightArgs,
    //         &'c mut E::Context<'cc>
    //     ) -> ResolveResult<NewTarget::Mutable<'ss>> + Send + Sync + 'static
    // {
    //     let left = self.clone();
    //     MutorForTarget::<NewTarget,RightArgs,_,E>::new(
    //         #[inline] move |root,_,callback,rarg: RightArgs,ctx| {
    //             left.with_mutor_cb(
    //                 root,
    //                 &mut |med,_,ctx| {
    //                     let res = (fun)(med,&(),rarg.clone(),ctx);
    //                     match res {
    //                         Ok(mut v) => (callback)(Ok(&mut v),&(),ctx),
    //                         Err(e) => (callback)(Err(e),&(),ctx),
    //                     }
    //                 },
    //                 larg.clone(),ctx
    //             )
    //         }
    //     )
    // }

    // fn for_view_ref<NewTarget,RightArgs,RightFn>(&self, larg: Args, fun: RightFn) -> MutorForTarget<NewTarget,RightArgs,MutorForViewRef<Self,Args,Target,NewTarget,RightArgs,RightFn,E>,E>
    // where
    //     E: Env,
    //     RightArgs: Sized + Send + Sync + 'static,
    //     NewTarget: MuTarget<E> + ?Sized,
    //     RightFn: for<'s,'ss,'c,'cc> Fn(
    //         ResolveResult<&'s mut <Target as MuTarget<E>>::Mutable<'ss>>,&'ss (),
    //         RightArgs,
    //         &'c mut E::Context<'cc>
    //     ) -> ResolveResult<&'s mut NewTarget::Mutable<'ss>> + Send + Sync + 'static
    // {
    //     let left = self.clone();
    //     MutorForTarget::<NewTarget,RightArgs,_,E>::new(
    //         #[inline] move |root,_,callback,rarg: RightArgs,ctx| {
    //             left.with_mutor_cb(
    //                 root,
    //                 &mut |med,_,ctx| {
    //                     let res = (fun)(med,&(),rarg.clone(),ctx);
    //                     match res {
    //                         Ok(mut v) => (callback)(Ok(v),&(),ctx),
    //                         Err(e) => (callback)(Err(e),&(),ctx),
    //                     }
    //                 },
    //                 larg.clone(),ctx
    //             )
    //         }
    //     )
    // }

    // fn for_view_cb_if<NewTarget,RightArgs,RightFn>(&self, larg: Args, fun: RightFn) -> MutorForTarget<NewTarget,RightArgs,MutorForViewCBIf<Self,Args,Target,NewTarget,RightArgs,RightFn,E>,E>
    // where
    //     E: Env,
    //     RightArgs: Sized + Send + Sync + 'static,
    //     NewTarget: MuTarget<E> + ?Sized,
    //     RightFn: for<'s,'ss,'c,'cc> Fn(
    //         &'s mut <Target as MuTarget<E>>::Mutable<'ss>,&'ss (),
    //         &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
    //         RightArgs,
    //         &'c mut E::Context<'cc>
    //     ) + Send + Sync + 'static
    // {
    //     let left = self.clone();
    //     MutorForTarget::<NewTarget,RightArgs,_,E>::new(
    //         #[inline] move |root,_,callback,rarg: RightArgs,ctx| {
    //             left.with_mutor_cb(
    //                 root,
    //                 &mut |med,_,ctx| {
    //                     match med {
    //                         Ok(v) => (fun)(v,&(),callback,rarg.clone(),ctx),
    //                         Err(e) => (callback)(Err(e),&(),ctx),
    //                     }
                        
    //                 },
    //                 larg.clone(),ctx
    //             )
    //         }
    //     )
    // }

    // fn for_view_ret_if<NewTarget,RightArgs,RightFn>(&self, larg: Args, fun: RightFn) -> MutorForTarget<NewTarget,RightArgs,MutorForViewRetIf<Self,Args,Target,NewTarget,RightArgs,RightFn,E>,E>
    // where
    //     E: Env,
    //     RightArgs: Sized + Send + Sync + 'static,
    //     NewTarget: MuTarget<E> + ?Sized,
    //     for<'a> NewTarget::Mutable<'a>: Sized,
    //     RightFn: for<'s,'ss,'c,'cc> Fn(
    //         &'s mut <Target as MuTarget<E>>::Mutable<'ss>,&'ss (),
    //         RightArgs,
    //         &'c mut E::Context<'cc>
    //     ) -> ResolveResult<NewTarget::Mutable<'ss>> + Send + Sync + 'static
    // {
    //     let left = self.clone();
    //     MutorForTarget::<NewTarget,RightArgs,_,E>::new(
    //         #[inline] move |root,_,callback,rarg: RightArgs,ctx| {
    //             left.with_mutor_cb(
    //                 root,
    //                 &mut |med,_,ctx| {
    //                     let res = match med {
    //                         Ok(v) => (fun)(v,&(),rarg.clone(),ctx),
    //                         Err(e) => {(callback)(Err(e),&(),ctx);return},
    //                     };
    //                     match res {
    //                         Ok(mut v) => (callback)(Ok(&mut v),&(),ctx),
    //                         Err(e) => (callback)(Err(e),&(),ctx),
    //                     }
    //                 },
    //                 larg.clone(),ctx
    //             )
    //         }
    //     )
    // }

    // fn for_view_ref_if<NewTarget,RightArgs,RightFn>(&self, larg: Args, fun: RightFn) -> MutorForTarget<NewTarget,RightArgs,MutorForViewRefIf<Self,Args,Target,NewTarget,RightArgs,RightFn,E>,E>
    // where
    //     E: Env,
    //     RightArgs: Sized + Send + Sync + 'static,
    //     NewTarget: MuTarget<E> + ?Sized,
    //     RightFn: for<'s,'ss,'c,'cc> Fn(
    //         &'s mut <Target as MuTarget<E>>::Mutable<'ss>,&'ss (),
    //         RightArgs,
    //         &'c mut E::Context<'cc>
    //     ) -> ResolveResult<&'s mut NewTarget::Mutable<'ss>> + Send + Sync + 'static
    // {
    //     let left = self.clone();
    //     MutorForTarget::<NewTarget,RightArgs,_,E>::new(
    //         #[inline] move |root,_,callback,rarg: RightArgs,ctx| {
    //             left.with_mutor_cb(
    //                 root,
    //                 &mut |med,_,ctx| {
    //                     let res = match med {
    //                         Ok(v) => (fun)(v,&(),rarg.clone(),ctx),
    //                         Err(e) => {(callback)(Err(e),&(),ctx);return},
    //                     };
    //                     match res {
    //                         Ok(mut v) => (callback)(Ok(v),&(),ctx),
    //                         Err(e) => (callback)(Err(e),&(),ctx),
    //                     }
    //                 },
    //                 larg.clone(),ctx
    //             )
    //         }
    //     )
    // }

    // fn mutor_end<RightArgs,RightFn>(&self, larg: Args, fun: RightFn) -> MutorEnde<RightArgs,MutorEndor<Self,Args,Target,RightArgs,RightFn,E>,E>
    // where
    //     E: Env,
    //     RightArgs: Sized + Send + Sync + 'static,
    //     RightFn: for<'s,'ss,'c,'cc> Fn(
    //         ResolveResult<&'s mut <Target as MuTarget<E>>::Mutable<'ss>>,&'ss (),
    //         RightArgs,
    //         &'c mut E::Context<'cc>
    //     ) + Send + Sync + 'static
    // {
    //     let left = self.clone();
    //     MutorEnde::<RightArgs,_,E>::new(
    //         #[inline] move |root,_,rarg: RightArgs,ctx| {
    //             left.with_mutor_cb(
    //                 root,
    //                 &mut |med,_,ctx| {
    //                     (fun)(med,&(),rarg.clone(),ctx);
    //                 },
    //                 larg.clone(),ctx
    //             )
    //         }
    //     )
    // }

    // fn mutor_end_if<RightArgs,RightFn>(&self, larg: Args, fun: RightFn) -> MutorEnde<RightArgs,MutorEndorIf<Self,Args,Target,RightArgs,RightFn,E>,E>
    // where
    //     E: Env,
    //     RightArgs: Sized + Send + Sync + 'static,
    //     RightFn: for<'s,'ss,'c,'cc> Fn(
    //         &'s mut <Target as MuTarget<E>>::Mutable<'ss>,&'ss (),
    //         RightArgs,
    //         &'c mut E::Context<'cc>
    //     ) + Send + Sync + 'static
    // {
    //     let left = self.clone();
    //     MutorEnde::<RightArgs,_,E>::new(
    //         #[inline] move |root,_,rarg: RightArgs,ctx| {
    //             left.with_mutor_cb(
    //                 root,
    //                 &mut |med,_,ctx| {
    //                     match med {
    //                         Ok(v) => (fun)(v,&(),rarg.clone(),ctx),
    //                         Err(e) => {}, //TODO
    //                     }
    //                 },
    //                 larg.clone(),ctx
    //             )
    //         }
    //     )
    // }
}

pub trait MutorToBuilderExt<Args,Target,E>: MutorToBuilder<Args,Target,E> + Send + Sync where E: Env, Args: Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    #[inline]
    fn erase<'a>(&'a self) -> &'a BoxingMutorToBuilder<Args,Target,Self,E> {
        unsafe{std::mem::transmute::<&'a Self,&'a BoxingMutorToBuilder<Args,Target,Self,E>>(self)}
    }
}
impl<Args,Target,T,E> MutorToBuilderExt<Args,Target,E> for T
where
    T: MutorToBuilder<Args,Target,E> + Send + Sync + ?Sized,
    E: Env,
    Args: Sized + Send + Sync + 'static,
    Target: MuTarget<E> + ?Sized
{}

pub struct ForTargetCBBuilder
    <LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E>
    (LeftMutor,LeftArgs,RightFn,PhantomData<(&'static RightTarget,&'static LeftTarget,RightArgs,E)>)
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E>,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static ,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,&'ss (),
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static;

impl<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E> MutorToBuilder<RightArgs,RightTarget,E> for
ForTargetCBBuilder<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E>,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static ,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,&'ss (),
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = impl MutorTo<RightArgs,RightTarget,E>;
    // #[inline]
    // fn _erase_builder(&self) -> &MutorToBuilderDyn<'_,RightArgs,RightTarget,E> {
    //     self
    // }
    #[inline]
    fn build(&self) -> Self::Built {
        let left = self.0.build();
        let larg = self.1.clone();
        let fun = self.2.clone();

        MutorForTarget::new(#[inline] move |root,_,callback,rarg: RightArgs,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,_,ctx| {
                    (fun)(med,&(),callback,rarg.clone(),ctx)
                },
                larg.clone(),ctx
            )
        })
    }
}

// pub type MutorForViewCB<LeftMutor,LeftArgs,LeftTarget,NewTarget,RightArgs,RightFn,E>
//     where
//         E: Env,
//         LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//         LeftTarget: MuTarget<E> + ?Sized,
//         LeftArgs: Sized + Send + Sync + 'static ,
//         RightArgs: Sized + Send + Sync + 'static,
//         NewTarget: MuTarget<E> + ?Sized,
//         RightFn: for<'s,'ss,'c,'cc> Fn(
//             ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,&'ss (),
//             &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//             RightArgs,
//             &'c mut E::Context<'cc>
//         ) + Send + Sync + 'static
//  = impl for<'s,'c,'cc> Fn(
//     E::RootMut<'s>,&'s (),
//     &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//     RightArgs,
//     &'c mut E::Context<'cc>
// ) + Send + Sync + 'static;
 
// pub type MutorForViewRet<LeftMutor,LeftArgs,LeftTarget,NewTarget,RightArgs,RightFn,E>
//     where
//         E: Env,
//         LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//         LeftTarget: MuTarget<E> + ?Sized,
//         LeftArgs: Sized + Send + Sync + 'static ,
//         RightArgs: Sized + Send + Sync + 'static,
//         NewTarget: MuTarget<E> + ?Sized,
//         for<'a> NewTarget::Mutable<'a>: Sized,
//         RightFn: for<'s,'ss,'c,'cc> Fn(
//             ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,&'ss (),
//             RightArgs,
//             &'c mut E::Context<'cc>
//         ) -> ResolveResult<NewTarget::Mutable<'ss>> + Send + Sync + 'static
// = impl for<'s,'c,'cc> Fn(
//     E::RootMut<'s>,&'s (),
//     &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//     RightArgs,
//     &'c mut E::Context<'cc>
// ) + Send + Sync + 'static;

// pub type MutorForViewRef<LeftMutor,LeftArgs,LeftTarget,NewTarget,RightArgs,RightFn,E>
//     where
//         E: Env,
//         LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//         LeftTarget: MuTarget<E> + ?Sized,
//         LeftArgs: Sized + Send + Sync + 'static ,
//         RightArgs: Sized + Send + Sync + 'static,
//         NewTarget: MuTarget<E> + ?Sized,
//         RightFn: for<'s,'ss,'c,'cc> Fn(
//             ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,&'ss (),
//             RightArgs,
//             &'c mut E::Context<'cc>
//         ) -> ResolveResult<&'s mut NewTarget::Mutable<'ss>> + Send + Sync + 'static
// = impl for<'s,'c,'cc> Fn(
//     E::RootMut<'s>,&'s (),
//     &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//     RightArgs,
//     &'c mut E::Context<'cc>
// ) + Send + Sync + 'static;

// pub type MutorForViewCBIf<LeftMutor,LeftArgs,LeftTarget,NewTarget,RightArgs,RightFn,E>
//     where
//         E: Env,
//         LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//         LeftTarget: MuTarget<E> + ?Sized,
//         LeftArgs: Sized + Send + Sync + 'static ,
//         RightArgs: Sized + Send + Sync + 'static,
//         NewTarget: MuTarget<E> + ?Sized,
//         RightFn: for<'s,'ss,'c,'cc> Fn(
//             &'s mut LeftTarget::Mutable<'ss>,&'ss (),
//             &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//             RightArgs,
//             &'c mut E::Context<'cc>
//         ) + Send + Sync + 'static
//  = impl for<'s,'c,'cc> Fn(
//     E::RootMut<'s>,&'s (),
//     &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//     RightArgs,
//     &'c mut E::Context<'cc>
// ) + Send + Sync + 'static;
 
// pub type MutorForViewRetIf<LeftMutor,LeftArgs,LeftTarget,NewTarget,RightArgs,RightFn,E>
//     where
//         E: Env,
//         LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//         LeftTarget: MuTarget<E> + ?Sized,
//         LeftArgs: Sized + Send + Sync + 'static ,
//         RightArgs: Sized + Send + Sync + 'static,
//         NewTarget: MuTarget<E> + ?Sized,
//         for<'a> NewTarget::Mutable<'a>: Sized,
//         RightFn: for<'s,'ss,'c,'cc> Fn(
//             &'s mut LeftTarget::Mutable<'ss>,&'ss (),
//             RightArgs,
//             &'c mut E::Context<'cc>
//         ) -> ResolveResult<NewTarget::Mutable<'ss>> + Send + Sync + 'static
// = impl for<'s,'c,'cc> Fn(
//     E::RootMut<'s>,&'s (),
//     &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//     RightArgs,
//     &'c mut E::Context<'cc>
// ) + Send + Sync + 'static;

// pub type MutorForViewRefIf<LeftMutor,LeftArgs,LeftTarget,NewTarget,RightArgs,RightFn,E>
//     where
//         E: Env,
//         LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//         LeftTarget: MuTarget<E> + ?Sized,
//         LeftArgs: Sized + Send + Sync + 'static ,
//         RightArgs: Sized + Send + Sync + 'static,
//         NewTarget: MuTarget<E> + ?Sized,
//         RightFn: for<'s,'ss,'c,'cc> Fn(
//             &'s mut LeftTarget::Mutable<'ss>,&'ss (),
//             RightArgs,
//             &'c mut E::Context<'cc>
//         ) -> ResolveResult<&'s mut NewTarget::Mutable<'ss>> + Send + Sync + 'static
// = impl for<'s,'c,'cc> Fn(
//     E::RootMut<'s>,&'s (),
//     &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//     RightArgs,
//     &'c mut E::Context<'cc>
// ) + Send + Sync + 'static;

// pub type MutorEndor<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E>
//     where
//         E: Env,
//         LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//         LeftTarget: MuTarget<E> + ?Sized,
//         LeftArgs: Sized + Send + Sync + 'static,
//         RightArgs: Sized + Send + Sync + 'static,
//         RightFn: for<'s,'ss,'c,'cc> Fn(
//             ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,&'ss (),
//             RightArgs,
//             &'c mut E::Context<'cc>
//         ) + Send + Sync + 'static
// = impl for<'s,'c,'cc> Fn(
//     E::RootMut<'s>,&'s (),
//     RightArgs,
//     &'c mut E::Context<'cc>
// ) + Send + Sync + 'static;

// pub type MutorEndorIf<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E>
//     where
//         E: Env,
//         LeftMutor: MutorTo<LeftArgs,LeftTarget,E>,
//         LeftTarget: MuTarget<E> + ?Sized,
//         LeftArgs: Sized + Send + Sync + 'static,
//         RightArgs: Sized + Send + Sync + 'static,
//         RightFn: for<'s,'ss,'c,'cc> Fn(
//             &'s mut LeftTarget::Mutable<'ss>,&'ss (),
//             RightArgs,
//             &'c mut E::Context<'cc>
//         ) + Send + Sync + 'static
// = impl for<'s,'c,'cc> Fn(
//     E::RootMut<'s>,&'s (),
//     RightArgs,
//     &'c mut E::Context<'cc>
// ) + Send + Sync + 'static;

pub struct MutorForTarget<Targ,Args,MutorFn,E>(MutorFn,PhantomData<(&'static Targ,Args,E)>)
where
    Self: 'static,
    E: Env,
    Targ: MuTarget<E> + ?Sized,
    Args: Sized + Send + Sync + 'static,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,&'s (),
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
        Args,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static;

impl<Targ,Args,MutorFn,E> MutorForTarget<Targ,Args,MutorFn,E>
where
    Self: 'static,
    E: Env,
    Targ: MuTarget<E> + ?Sized,
    Args: Sized + Send + Sync + 'static,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,&'s (),
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
        Args,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static
{
    pub fn new(f: MutorFn) -> Self {
        Self(f,PhantomData)
    }
}

// impl<Target,Args,MutorFn,E> MutorTo<Args,Target,E> for MutorFn
// where
//     Self: 'static,
//     E: Env,
//     Target: MuTarget<E> + ?Sized,
//     Args: Sized + Send + Sync + 'static,
//     MutorFn: for<'s,'c,'cc> Fn(
//         E::RootMut<'s>,&'s (),
//         &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//         Args,
//         &'c mut E::Context<'cc>
//     ) + Send + Sync + 'static
// {
//     #[inline]
//     fn with_mutor_cb<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (self)(root,&(),callback,args,ctx)
//     }
// }

// impl<Targ,Args,MutorFn,E> Clone for MutorForTarget<Targ,Args,MutorFn,E>
// where
//     Self: 'static,
//     E: Env,
//     Targ: MuTarget<E> + ?Sized,
//     Args: Sized + Send + Sync + 'static,
//     MutorFn: for<'s,'c,'cc> Fn(
//         E::RootMut<'s>,&'s (),
//         &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//         Args,
//         &'c mut E::Context<'cc>
//     ) + Send + Sync + 'static
// {
//     fn clone(&self) -> Self {
//         Self(self.0.clone(),PhantomData)
//     }
// }

impl<Targ,Args,MutorFn,E> MutorTo<Args,Targ,E> for MutorForTarget<Targ,Args,MutorFn,E>
where
    Self: 'static,
    E: Env,
    Targ: MuTarget<E> + ?Sized,
    Args: Sized + Send + Sync + 'static,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,&'s (),
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
        Args,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static
{
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
        args: Args,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (self.0)(root,&(),callback,args,ctx)
    }

    // type MutorConvertTarget<T> = MutorForTarget<MutorFn,T,Args,E>
    // where for<'a> T: MuTarget<E,Mutable<'a>=<Self::Target as MuTarget<E>>::Mutable<'a>>;

    // fn convert_to_target<T>(&self) -> Self::MutorConvertTarget<T> where for<'a> T: MuTarget<E,Mutable<'a>=<Self::Target as MuTarget<E>>::Mutable<'a>> {
    //     MutorForTarget(self.0.clone(),PhantomData::<(T,Args,E)>)
    // }
}

// impl<Targ,Args,MutorFn,E> MutorEnd<Args,E> for MutorForTarget<Targ,Args,MutorFn,E>
// where
//     Self: 'static,
//     E: Env,
//     Targ: MuTarget<E> + ?Sized,
//     Args: Sized + Send + Sync + 'static,
//     MutorFn: for<'s,'c,'cc> Fn(
//         E::RootMut<'s>,&'s (),
//         &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//         Args,
//         &'c mut E::Context<'cc>
//     ) + Send + Sync + 'static
// {
//     fn with_mutor_end<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (self.0)(root,&(),&mut |_,_,_|{},args,ctx)
//     }
// }

// pub struct MutorEnde<Args,MutorFn,E>(MutorFn,PhantomData<(Args,E)>)
// where
//     Self: 'static,
//     E: Env,
//     Args: Sized,
//     MutorFn: for<'s,'c,'cc> Fn(
//         E::RootMut<'s>,&'s (),
//         Args,
//         &'c mut E::Context<'cc>
//     ) + Send + Sync + 'static;

// impl<MutorFn,Args,E> MutorEnde<Args,MutorFn,E>
// where
//     Self: 'static,
//     E: Env,
//     Args: Sized,
//     MutorFn: for<'s,'c,'cc> Fn(
//         E::RootMut<'s>,&'s (),
//         Args,
//         &'c mut E::Context<'cc>
//     ) + Send + Sync + 'static
// {
//     pub fn new(f: MutorFn) -> Self {
//         Self(f,PhantomData)
//     }
// }

// impl<Args,MutorFn,E> MutorEnd<Args,E> for MutorFn
// where
//     Self: 'static,
//     E: Env,
//     Args: Sized + Send + Sync + 'static,
//     MutorFn: for<'s,'c,'cc> Fn(
//         E::RootMut<'s>,&'s (),
//         Args,
//         &'c mut E::Context<'cc>
//     ) + Send + Sync + 'static
// {
//     #[inline]
//     fn with_mutor_end<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (self)(root,&(),args,ctx);
//     }
// }

// impl<MutorFn,Args,E> Clone for MutorEnde<Args,MutorFn,E>
// where
//     Self: 'static,
//     E: Env,
//     Args: Sized + 'static,
//     MutorFn: for<'s,'c,'cc> Fn(
//         E::RootMut<'s>,&'s (),
//         Args,
//         &'c mut E::Context<'cc>
//     ) + Send + Sync + 'static
// {
//     fn clone(&self) -> Self {
//         Self(self.0.clone(),PhantomData)
//     }
// }

// impl<MutorFn,Args,E> MutorEnd<Args,E> for MutorEnde<Args,MutorFn,E>
// where
//     Self: 'static,
//     E: Env,
//     Args: Sized + Send + Sync + 'static,
//     MutorFn: for<'s,'c,'cc> Fn(
//         E::RootMut<'s>,&'s (),
//         Args,
//         &'c mut E::Context<'cc>
//     ) + Send + Sync + 'static
// {

//     fn with_mutor_end<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (self.0)(root,&(),args,ctx);
//     }
// }

// pub struct ConvertToTarget<MutorTy,LeftTarget,RightTarget,Args,E>(MutorTy,PhantomData<(&'static LeftTarget,&'static RightTarget,Args,E)>)
// where
//     E: Env,
//     Args: Sized + Send + Sync + 'static,
//     MutorTy: MutorTo<Args,LeftTarget,E>,
//     LeftTarget: MuTarget<E> + ?Sized,
//     RightTarget: MuTarget<E> + ?Sized,
//     for<'a> RightTarget: MuTarget<E,Mutable<'a>=LeftTarget::Mutable<'a>>;

// impl<MutorTy,LeftTarget,RightTarget,Args,E> Clone for ConvertToTarget<MutorTy,LeftTarget,RightTarget,Args,E>
// where
//     E: Env,
//     Args: Sized + Send + Sync + 'static,
//     MutorTy: MutorTo<Args,LeftTarget,E>,
//     LeftTarget: MuTarget<E> + ?Sized,
//     RightTarget: MuTarget<E> + ?Sized,
//     for<'a> RightTarget: MuTarget<E,Mutable<'a>=LeftTarget::Mutable<'a>>
// {
//     fn clone(&self) -> Self {
//         Self(self.0.clone(),PhantomData)
//     }
// }

// impl<MutorTy,LeftTarget,RightTarget,Args,E> MutorEnd<Args,E> for ConvertToTarget<MutorTy,LeftTarget,RightTarget,Args,E>
// where
//     E: Env,
//     Args: Sized + Send + Sync + 'static,
//     MutorTy: MutorTo<Args,LeftTarget,E>,
//     LeftTarget: MuTarget<E> + ?Sized,
//     RightTarget: MuTarget<E> + ?Sized,
//     for<'a> RightTarget: MuTarget<E,Mutable<'a>=LeftTarget::Mutable<'a>>
// {
//     fn with_mutor_end<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         self.0.with_mutor_end(root, args, ctx)
//     }

//     #[inline]
//     fn _boxed_end(&self) -> Box<dyn MutorEndDyn<Args,E>+'static> {
//         MutorEnd::_boxed_end(&self.0)
//     }
// }

// impl<MutorTy,LeftTarget,RightTarget,Args,E> MutorTo<Args,RightTarget,E> for ConvertToTarget<MutorTy,LeftTarget,RightTarget,Args,E>
// where
//     E: Env,
//     Args: Sized + Send + Sync + 'static,
//     MutorTy: MutorTo<Args,LeftTarget,E>,
//     LeftTarget: MuTarget<E> + ?Sized,
//     RightTarget: MuTarget<E> + ?Sized,
//     for<'a> RightTarget: MuTarget<E,Mutable<'a>=LeftTarget::Mutable<'a>>
// {
//     fn with_mutor_cb<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut <RightTarget as MuTarget<E>>::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         self.0.with_mutor_cb(root, callback, args, ctx)
//     }

//     // #[inline]
//     // fn _boxed(&self) -> Box<dyn MutorToDyn<Args,E,Target=Self::Target>+'static> {
//     //     unsafe{
//     //         std::mem::transmute::<
//     //             Box<dyn MutorToDyn<Args,E,Target=MutorTy::Target>+'static>,
//     //             Box<dyn MutorToDyn<Args,E,Target=Self::Target>+'static>
//     //         >(MutorTo::_boxed(&self.0))
//     //     }
//     // }
// }

// pub trait MutorEndDyn<Args,E>: Send + Sync + 'static where E: Env, Args: Sized + Send + Sync + 'static {
//     fn with_mutor_end_dyn<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c;

//     fn _boxed_dyn(&self) -> Box<dyn MutorEndDyn<Args,E>+'static>;
// }

// pub trait MutorToDyn<Args,Target,E>: MutorEndDyn<Args,E> + 'static where E: Env, Args: Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
//     fn with_mutor_cb_dyn<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c;

//     fn _boxed_dyn(&self) -> Box<dyn MutorToDyn<Args,Target,E>+'static>;
// }

// impl<T,Args,E> MutorEndDyn<Args,E> for T where T: MutorEnd<Args,E> + ?Sized, E: Env, Args: Sized + Send + Sync + 'static {
//     fn with_mutor_end_dyn<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         self.with_mutor_end(root, args, ctx)
//     }

//     #[inline]
//     fn _boxed_dyn(&self) -> Box<dyn MutorEndDyn<Args,E>+'static> {
//         MutorEnd::_boxed_end(self)
//     }
// }

// impl<T,Args,Targ,E> MutorToDyn<Args,Targ,E> for T where T: MutorTo<Args,Targ,E> + ?Sized, E: Env, Args: Sized + Send + Sync + 'static, Targ: MuTarget<E> + ?Sized {
//     fn with_mutor_cb_dyn<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         self.with_mutor_cb(root, callback, args, ctx)
//     }

//     #[inline]
//     fn _boxed_dyn(&self) -> Box<dyn MutorToDyn<Args,Targ,E>+'static> {
//         MutorTo::_boxed_to(self)
//     }
// }

// impl<Args,E> Clone for Box<dyn MutorEndDyn<Args,E> + 'static> where E: Env, Args: Sized + Send + Sync + 'static {
//     #[inline]
//     fn clone(&self) -> Self {
//         MutorEndDyn::_boxed_dyn(&**self)
//     }
// }

// impl<Args,E> MutorEnd<Args,E> for Box<dyn MutorEndDyn<Args,E> + 'static> where E: Env, Args: Sized + Send + Sync + 'static {
//     fn with_mutor_end<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (**self).with_mutor_end_dyn(root, args, ctx)
//     }

//     #[inline]
//     fn _boxed_end(&self) -> Box<dyn MutorEndDyn<Args,E>+'static> {
//         MutorEndDyn::_boxed_dyn(&**self)
//     }
// }

// impl<Args,E,Targ> Clone for Box<dyn MutorToDyn<Args,Targ,E> + 'static> where E: Env, Targ: MuTarget<E> + ?Sized, Args: Sized + Send + Sync + 'static {
//     #[inline]
//     fn clone(&self) -> Self {
//         MutorToDyn::_boxed_dyn(&**self)
//     }
// }

// impl<Args,E,Targ> MutorEnd<Args,E> for Box<dyn MutorToDyn<Args,Targ,E> + 'static> where E: Env, Targ: MuTarget<E> + ?Sized, Args: Sized + Send + Sync + 'static {
//     fn with_mutor_end<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (**self).with_mutor_end_dyn(root, args, ctx)
//     }

//     #[inline]
//     fn _boxed_end(&self) -> Box<dyn MutorEndDyn<Args,E>+'static> {
//         MutorEndDyn::_boxed_dyn(&**self)
//     }
// }

// impl<Args,E,Targ> MutorTo<Args,Targ,E> for Box<dyn MutorToDyn<Args,Targ,E> + 'static> where E: Env, Targ: MuTarget<E> + ?Sized, Args: Sized + Send + Sync + 'static {
//     fn with_mutor_cb<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (**self).with_mutor_cb_dyn(root, callback, args, ctx)
//     }

//     #[inline]
//     fn _boxed_to(&self) -> Box<dyn MutorToDyn<Args,Targ,E>+'static> {
//         MutorToDyn::_boxed_dyn(&**self)
//     }
// }

// impl<Args,E> MutorEnd<Args,E> for std::sync::Arc<dyn MutorEndDyn<Args,E> + 'static> where E: Env, Args: Sized + Send + Sync + 'static {
//     fn with_mutor_end<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (**self).with_mutor_end_dyn(root, args, ctx)
//     }

//     #[inline]
//     fn _boxed_end(&self) -> Box<dyn MutorEndDyn<Args,E>+'static> {
//         MutorEndDyn::_boxed_dyn(&**self)
//     }
// }

// impl<Args,E,Targ> MutorEnd<Args,E> for std::sync::Arc<dyn MutorToDyn<Args,Targ,E> + 'static> where E: Env, Targ: MuTarget<E> + ?Sized, Args: Sized + Send + Sync + 'static {
//     fn with_mutor_end<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (**self).with_mutor_end_dyn(root, args, ctx)
//     }

//     #[inline]
//     fn _boxed_end(&self) -> Box<dyn MutorEndDyn<Args,E>+'static> {
//         MutorEndDyn::_boxed_dyn(&**self)
//     }
// }

// impl<Args,E,Targ> MutorTo<Args,Targ,E> for std::sync::Arc<dyn MutorToDyn<Args,Targ,E> + 'static> where E: Env, Targ: MuTarget<E> + ?Sized, Args: Sized + Send + Sync + 'static {
//     fn with_mutor_cb<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (**self).with_mutor_cb_dyn(root, callback, args, ctx)
//     }

//     #[inline]
//     fn _boxed_to(&self) -> Box<dyn MutorToDyn<Args,Targ,E>+'static> {
//         MutorToDyn::_boxed_dyn(&**self)
//     }
// }

// #[derive(Clone)]
// pub struct InitialMutor;

// impl<Args,E> MutorEnd<Args,E> for InitialMutor where E: Env, Args: Sized + Send + Sync + 'static {
//     fn with_mutor_end<'s,'c,'cc>(
//         &self,
//         root: E::RootMut<'s>,
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {}
// }

// impl<Args,E> MutorTo<Args,E> for InitialMutor where E: Env, Args: Sized + Send + Sync + 'static {
//     type Target = RootMutTarget<E>;

//     fn with_mutor_cb<'s,'c,'cc>(
//         &self,
//         mut root: E::RootMut<'s>,
//         callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut <Self::Target as MuTarget<E>>::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
//         args: Args,
//         ctx: &'c mut E::Context<'cc>,
//     ) where 'cc: 'c {
//         (callback)(Ok(&mut root),&(),ctx)
//     }
// }

#[repr(transparent)]
pub struct BoxingMutorEndBuilder<Args,T,E>(PhantomData<(Args,E)>,T)
where
    E: Env,
    Args: Sized + Send + Sync + 'static,
    T: MutorEndBuilder<Args,E> + ?Sized;

impl<Args,T,E> MutorEndBuilder<Args,E> for BoxingMutorEndBuilder<Args,T,E>
where
    E: Env,
    Args: Sized + Send + Sync + 'static,
    T: MutorEndBuilder<Args,E> + ?Sized
{
    type Built = Box<dyn MutorEnd<Args,E>>;

    #[inline]
    fn build(&self) -> Self::Built {
        self.1.build_boxed()
    }
    #[inline]
    fn build_boxed(&self) -> Box<dyn MutorEnd<Args,E>> {
        self.1.build_boxed()
    }
}

#[repr(transparent)]
pub struct BoxingMutorToBuilder<Args,Target,T,E>(PhantomData<(Args,&'static Target,E)>,T)
where
    E: Env,
    Args: Sized + Send + Sync + 'static,
    Target: MuTarget<E> + ?Sized,
    T: MutorToBuilder<Args,Target,E> + ?Sized;

impl<Args,Target,T,E> MutorToBuilder<Args,Target,E> for BoxingMutorToBuilder<Args,Target,T,E>
where
    E: Env,
    Args: Sized + Send + Sync + 'static,
    Target: MuTarget<E> + ?Sized,
    T: MutorToBuilder<Args,Target,E> + ?Sized
{
    type Built = Box<dyn MutorTo<Args,Target,E>>;
    
    #[inline]
    fn build(&self) -> Self::Built {
        self.1.build_boxed()
    }
    #[inline]
    fn build_boxed(&self) -> Box<dyn MutorTo<Args,Target,E>> {
        self.1.build_boxed()
    }
}

impl<Args,T,E> MutorEnd<Args,E> for Box<T> where T: MutorEnd<Args,E> + ?Sized, E: Env, Args: Sized + Send + Sync + 'static {
    #[inline]
    fn with_mutor_end<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        args: Args,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (**self).with_mutor_end(root, args, ctx)
    }
}

impl<Args,Target,T,E> MutorTo<Args,Target,E> for Box<T> where T: MutorTo<Args,Target,E> + ?Sized, E: Env, Args: Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: <E as Env>::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'iss (),&'ic mut E::Context<'icc>)),
        args: Args,
        ctx: &'c mut <E as Env>::Context<'cc>,
    ) where 'cc: 'c {
        (**self).with_mutor_cb(root, callback, args, ctx)
    }
}
