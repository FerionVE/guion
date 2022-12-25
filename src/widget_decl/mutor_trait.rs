use std::marker::PhantomData;
use std::sync::Arc;

use crate::ctx::queue::{BoxMutEvent, ArcMutEvent};
use crate::env::Env;
use crate::error::ResolveResult;

use super::mut_target::MuTarget;

pub trait MutorEndBuilder<Args,E>: Send + Sync where E: Env, Args: Clone + Sized + Send + Sync + 'static {
    type Built: MutorEnd<Args,E> + Sized + Send + Sync + 'static;
    type Built2: MutorEnd<Args,E> + Sized + Send + Sync + 'static;

    fn erase<'a>(&'a self) -> &'a (dyn MutorEndBuilderDyn<Args,E>+'_);

    fn build(&self) -> Self::Built;

    fn build2(&self) -> Self::Built2;

    #[inline]
    fn build_boxed(&self) -> Box<dyn MutorEnd<Args,E>> {
        Box::new(self.build())
    }

    #[inline]
    fn build_arced(&self) -> Arc<dyn MutorEnd<Args,E>> {
        Arc::new(self.build())
    }

    #[inline]
    fn build_box_mut_event(&self, args: Args) -> Option<BoxMutEvent<E>> {
        let b = self.build();
        Some(Box::new(#[inline] move |root,_,ctx| b.with_mutor_end(root, args, ctx) ))
    }

    #[inline]
    fn build_arc_mut_event(&self, args: Args) -> Option<ArcMutEvent<E>> {
        let b = self.build2();
        Some(Arc::new(#[inline] move |root,_,ctx| b.with_mutor_end(root, args, ctx) ))
    }
}

//pub type MutorEndBuilderDyn<'a,Args,E> = dyn MutorEndBuilder<Args,E,Built=Box<dyn MutorEnd<Args,E>>> + 'a;

impl<Args,E> MutorEndBuilder<Args,E> for () where E: Env, Args: Clone + Sized + Send + Sync + 'static {
    type Built = ();
    type Built2 = ();

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorEndBuilderDyn<Args,E>+'_) {
        self
    }

    #[inline]
    fn build(&self) -> Self::Built {}
    #[inline]
    fn build2(&self) -> Self::Built2 {}

    #[inline]
    fn build_box_mut_event(&self, _: Args) -> Option<BoxMutEvent<E>> {
        None
    }
    #[inline]
    fn build_arc_mut_event(&self, _: Args) -> Option<ArcMutEvent<E>> {
        None
    }
}

pub trait MutorEnd<Args,E>: Send + Sync + 'static where E: Env, Args: Clone + Sized + Send + Sync + 'static {
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

impl<Args,E> MutorEnd<Args,E> for () where E: Env, Args: Clone + Sized + Send + Sync + 'static {
    #[inline]
    fn with_mutor_end<'s,'c,'cc>(
        &self,
        _: E::RootMut<'s>,
        _: Args,
        _: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {}
}

pub trait MutorEndBuilderExt<Args,E>: MutorEndBuilder<Args,E> + Send + Sync where E: Env, Args: Clone + Sized + Send + Sync + 'static {
    
}
impl<Args,T,E> MutorEndBuilderExt<Args,E> for T where T: MutorEndBuilder<Args,E> + Send + Sync + ?Sized, E: Env, Args: Clone + Sized + Send + Sync + 'static {}

pub trait MutorToBuilder<Args,Target,E>: Send + Sync where E: Env, Args: Clone + Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    type Built: MutorTo<Args,Target,E> + Sized + Send + Sync + 'static;
    type Built2: MutorTo<Args,Target,E> + Sized + Send + Sync + 'static;

    fn erase<'a>(&'a self) -> &'a (dyn MutorToBuilderDyn<Args,Target,E>+'_);

    #[inline]
    fn convert_to_target<NewTarget>(self) -> ConvertToTargetBuilder<Self,Target,NewTarget,Args,E> where Self: Sized, for<'b> NewTarget: MuTarget<E,Mutable<'b>=Target::Mutable<'b>> {
        ConvertToTargetBuilder(PhantomData,self)
    }

    fn build(&self) -> Self::Built;
    fn build2(&self) -> Self::Built2;

    #[inline]
    fn build_boxed(&self) -> Box<dyn MutorTo<Args,Target,E>> {
        Box::new(self.build())
    }

    #[inline]
    fn build_arced(&self) -> Arc<dyn MutorTo<Args,Target,E>> {
        Arc::new(self.build())
    }
}

//pub type MutorToBuilderDyn<'a,Args,Target,E> = dyn MutorToBuilder<Args,Target,E,Built=Box<dyn MutorTo<Args,Target,E>>> + 'a;

pub trait MutorTo<Args,Target,E>: Send + Sync + 'static where E: Env, Args: Clone + Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        args: Args,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c;
}

pub trait MutorToBuilderExt<Args,Target,E>: MutorToBuilder<Args,Target,E> + Send + Sync where E: Env, Args: Clone + Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    // #[inline]
    // fn erase<'a>(&'a self) -> BoxingMutorToBuilder<Args,Target,Self,E> {
    //     BoxingMutorToBuilder(PhantomData,self)
    // }

    // #[inline]
    // fn convert_to_target<'a,T>(&'a self) -> ConvertToTargetBuilder<'a,Self,Target,T,Args,E> where for<'b> T: MuTarget<E,Mutable<'b>=Target::Mutable<'b>> {
    //     ConvertToTargetBuilder(PhantomData,self)
    // }

    #[inline]
    fn for_view_cb<NewTarget,RightArgs,RightFn>(self, larg: Args, fun: RightFn) -> ForTargetCBBuilder<Self,Args,Target,RightArgs,NewTarget,RightFn,E>
    where
        Self: Sized,
        E: Env,
        RightArgs: Clone + Sized + Send + Sync + 'static,
        NewTarget: MuTarget<E> + ?Sized,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            ResolveResult<&'s mut Target::Mutable<'ss>>,
            &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
            RightArgs,
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        ForTargetCBBuilder(self,larg,fun,PhantomData)
    }

    #[inline]
    fn for_view_cb_if<NewTarget,RightArgs,RightFn>(self, larg: Args, fun: RightFn) -> ForTargetCBIfBuilder<Self,Args,Target,RightArgs,NewTarget,RightFn,E>
    where
        Self: Sized,
        E: Env,
        RightArgs: Clone + Sized + Send + Sync + 'static,
        NewTarget: MuTarget<E> + ?Sized,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            &'s mut Target::Mutable<'ss>,
            &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
            RightArgs,
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        ForTargetCBIfBuilder(self,larg,fun,PhantomData)
    }

    #[inline]
    fn mutor_end<RightArgs,RightFn>(self, larg: Args, fun: RightFn) -> EndorBuilder<Self,Args,Target,RightArgs,RightFn,E>
    where
        Self: Sized,
        E: Env,
        RightArgs: Clone + Sized + Send + Sync + 'static,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            ResolveResult<&'s mut Target::Mutable<'ss>>,
            RightArgs,
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        EndorBuilder(self,larg,fun,PhantomData)
    }

    #[inline]
    fn mutor_end_if<RightArgs,RightFn>(self, larg: Args, fun: RightFn) -> EndorIfBuilder<Self,Args,Target,RightArgs,RightFn,E>
    where
        Self: Sized,
        E: Env,
        RightArgs: Clone + Sized + Send + Sync + 'static,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            &'s mut Target::Mutable<'ss>,
            RightArgs,
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        EndorIfBuilder(self,larg,fun,PhantomData)
    }
}
impl<Args,Target,T,E> MutorToBuilderExt<Args,Target,E> for T
where
    T: MutorToBuilder<Args,Target,E> + Send + Sync + ?Sized,
    E: Env,
    Args: Clone + Sized + Send + Sync + 'static,
    Target: MuTarget<E> + ?Sized
{}

pub struct ForTargetCBBuilder
    <LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E>
    (LeftMutor,LeftArgs,RightFn,PhantomData<(&'static RightTarget,&'static LeftTarget,fn(RightArgs),E)>)
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static;

impl<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E> MutorToBuilder<RightArgs,RightTarget,E> for
ForTargetCBBuilder<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static ,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = impl MutorTo<RightArgs,RightTarget,E>;
    type Built2 = impl MutorTo<RightArgs,RightTarget,E>;

    #[inline]
    fn erase(&self) -> &(dyn MutorToBuilderDyn<RightArgs,RightTarget,E>+'_) {
        self
    }

    #[inline]
    fn build(&self) -> Self::Built {
        let left = self.0.build();
        let larg = self.1.clone();
        let fun = self.2.clone();

        MutorForTarget::new(#[inline] move |root,callback,rarg: RightArgs,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    (fun)(med,callback,rarg.clone(),ctx)
                },
                larg.clone(),ctx
            )
        })
    }

    #[inline]
    fn build2(&self) -> Self::Built2 {
        let left = self.0.build2();
        let larg = self.1.clone();
        let fun = self.2.clone();

        MutorForTarget::new(#[inline] move |root,callback,rarg: RightArgs,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    (fun)(med,callback,rarg.clone(),ctx)
                },
                larg.clone(),ctx
            )
        })
    }
}

impl<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E> Clone for
ForTargetCBBuilder<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Clone + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static ,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone(), PhantomData)
    }
}

pub struct ForTargetCBIfBuilder
    <LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E>
    (LeftMutor,LeftArgs,RightFn,PhantomData<(&'static RightTarget,&'static LeftTarget,fn(RightArgs),E)>)
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static;

impl<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E> MutorToBuilder<RightArgs,RightTarget,E> for
ForTargetCBIfBuilder<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = impl MutorTo<RightArgs,RightTarget,E>;
    type Built2 = impl MutorTo<RightArgs,RightTarget,E>;

    #[inline]
    fn erase(&self) -> &(dyn MutorToBuilderDyn<RightArgs,RightTarget,E>+'_) {
        self
    }

    #[inline]
    fn build(&self) -> Self::Built {
        let left = self.0.build();
        let larg = self.1.clone();
        let fun = self.2.clone();

        MutorForTarget::new(#[inline] move |root,callback,rarg: RightArgs,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    match med {
                        Ok(v) => (fun)(v,callback,rarg.clone(),ctx),
                        Err(e) => (callback)(Err(e),ctx),
                    }
                },
                larg.clone(),ctx
            )
        })
    }

    #[inline]
    fn build2(&self) -> Self::Built2 {
        let left = self.0.build2();
        let larg = self.1.clone();
        let fun = self.2.clone();

        MutorForTarget::new(#[inline] move |root,callback,rarg: RightArgs,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    match med {
                        Ok(v) => (fun)(v,callback,rarg.clone(),ctx),
                        Err(e) => (callback)(Err(e),ctx),
                    }
                },
                larg.clone(),ctx
            )
        })
    }
}

impl<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E> Clone for
ForTargetCBIfBuilder<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Clone + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone(), PhantomData)
    }
}

pub struct EndorBuilder
    <LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E>
    (LeftMutor,LeftArgs,RightFn,PhantomData<(&'static LeftTarget,fn(RightArgs),E)>)
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static ,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static;

impl<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E> MutorEndBuilder<RightArgs,E> for
EndorBuilder<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static ,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = impl MutorEnd<RightArgs,E>;
    type Built2 = impl MutorEnd<RightArgs,E>;

    #[inline]
    fn erase(&self) -> &(dyn MutorEndBuilderDyn<RightArgs,E>+'_) {
        self
    }

    #[inline]
    fn build(&self) -> Self::Built {
        let left = self.0.build();
        let larg = self.1.clone();
        let fun = self.2.clone();

        MutorEnde::new(#[inline] move |root,rarg: RightArgs,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    (fun)(med,rarg.clone(),ctx)
                },
                larg.clone(),ctx
            )
        })
    }

    #[inline]
    fn build2(&self) -> Self::Built2 {
        let left = self.0.build2();
        let larg = self.1.clone();
        let fun = self.2.clone();

        MutorEnde::new(#[inline] move |root,rarg: RightArgs,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    (fun)(med,rarg.clone(),ctx)
                },
                larg.clone(),ctx
            )
        })
    }
}

impl<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E> Clone for
EndorBuilder<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Clone + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static ,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone(), PhantomData)
    }
}

pub struct EndorIfBuilder
    <LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E>
    (LeftMutor,LeftArgs,RightFn,PhantomData<(&'static LeftTarget,fn(RightArgs),E)>)
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static ,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static;

impl<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E> MutorEndBuilder<RightArgs,E> for
EndorIfBuilder<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static ,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = impl MutorEnd<RightArgs,E>;
    type Built2 = impl MutorEnd<RightArgs,E>;

    #[inline]
    fn erase(&self) -> &(dyn MutorEndBuilderDyn<RightArgs,E>+'_) {
        self
    }

    #[inline]
    fn build(&self) -> Self::Built {
        let left = self.0.build();
        let larg = self.1.clone();
        let fun = self.2.clone();

        MutorEnde::new(#[inline] move |root,rarg: RightArgs,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    match med {
                        Ok(v) => (fun)(v,rarg.clone(),ctx),
                        Err(e) => {}, //TODO detect lost mutor debug mode
                    }
                },
                larg.clone(),ctx
            )
        })
    }

    #[inline]
    fn build2(&self) -> Self::Built2 {
        let left = self.0.build2();
        let larg = self.1.clone();
        let fun = self.2.clone();

        MutorEnde::new(#[inline] move |root,rarg: RightArgs,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    match med {
                        Ok(v) => (fun)(v,rarg.clone(),ctx),
                        Err(e) => {}, //TODO detect lost mutor debug mode
                    }
                },
                larg.clone(),ctx
            )
        })
    }
}

impl<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E> Clone for
EndorIfBuilder<LeftMutor,LeftArgs,LeftTarget,RightArgs,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Clone + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    LeftArgs: Clone + Sized + Send + Sync + 'static ,
    RightArgs: Clone + Sized + Send + Sync + 'static,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        RightArgs,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone(), PhantomData)
    }
}

pub struct MutorForTarget<Targ,Args,MutorFn,E>(MutorFn,PhantomData<(&'static Targ,fn(Args),E)>)
where
    Self: 'static,
    E: Env,
    Targ: MuTarget<E> + ?Sized,
    Args: Sized + Send + Sync + 'static,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
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
        E::RootMut<'s>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        Args,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static
{
    pub fn new(f: MutorFn) -> Self {
        Self(f,PhantomData)
    }
}

impl<Targ,Args,MutorFn,E> MutorToBuilder<Args,Targ,E> for MutorForTarget<Targ,Args,MutorFn,E>
where
    Self: 'static,
    E: Env,
    Targ: MuTarget<E> + ?Sized,
    Args: Clone + Sized + Send + Sync + 'static,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        Args,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = Self;
    type Built2 = Self;

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorToBuilderDyn<Args,Targ,E>+'_) {
        self
    }
    #[inline]
    fn build(&self) -> Self::Built {
        Self(self.0.clone(),PhantomData)
    }
    #[inline]
    fn build2(&self) -> Self::Built {
        Self(self.0.clone(),PhantomData)
    }
}

impl<Targ,Args,MutorFn,E> MutorTo<Args,Targ,E> for MutorForTarget<Targ,Args,MutorFn,E>
where
    Self: 'static,
    E: Env,
    Targ: MuTarget<E> + ?Sized,
    Args: Clone + Sized + Send + Sync + 'static,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        Args,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static
{
    #[inline]
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        args: Args,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (self.0)(root,callback,args,ctx)
    }
}

pub struct MutorEnde<Args,MutorFn,E>(MutorFn,PhantomData<(fn(Args),E)>)
where
    Self: 'static,
    E: Env,
    Args: Clone + Send + Sync + Sized,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        Args,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static;

impl<MutorFn,Args,E> MutorEnde<Args,MutorFn,E>
where
    Self: 'static,
    E: Env,
    Args: Clone + Send + Sync + Sized,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        Args,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static
{
    #[inline]
    pub fn new(f: MutorFn) -> Self {
        Self(f,PhantomData)
    }
}

impl<MutorFn,Args,E> MutorEndBuilder<Args,E> for MutorEnde<Args,MutorFn,E>
where
    Self: 'static,
    E: Env,
    Args: Clone + Sized + Send + Sync + 'static,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        Args,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = Self;
    type Built2 = Self;

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorEndBuilderDyn<Args,E>+'_) {
        self
    }
    #[inline]
    fn build(&self) -> Self::Built {
        Self(self.0.clone(),PhantomData)
    }
    #[inline]
    fn build2(&self) -> Self::Built {
        Self(self.0.clone(),PhantomData)
    }
}

impl<MutorFn,Args,E> MutorEnd<Args,E> for MutorEnde<Args,MutorFn,E>
where
    Self: 'static,
    E: Env,
    Args: Clone + Sized + Send + Sync + 'static,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        Args,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static
{
    #[inline]
    fn with_mutor_end<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        args: Args,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (self.0)(root,args,ctx);
    }
}

impl<Args,T,E> MutorEnd<Args,E> for Box<T> where T: MutorEnd<Args,E> + ?Sized, E: Env, Args: Clone + Sized + Send + Sync + 'static {
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

impl<Args,Target,T,E> MutorTo<Args,Target,E> for Box<T> where T: MutorTo<Args,Target,E> + ?Sized, E: Env, Args: Clone + Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        args: Args,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (**self).with_mutor_cb(root, callback, args, ctx)
    }
}

impl<Args,T,E> MutorEnd<Args,E> for Arc<T> where T: MutorEnd<Args,E> + ?Sized, E: Env, Args: Clone + Sized + Send + Sync + 'static {
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

impl<Args,Target,T,E> MutorTo<Args,Target,E> for Arc<T> where T: MutorTo<Args,Target,E> + ?Sized, E: Env, Args: Clone + Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        args: Args,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (**self).with_mutor_cb(root, callback, args, ctx)
    }
}

impl<Args,T,E> MutorEndBuilder<Args,E> for &T where T: MutorEndBuilder<Args,E> + ?Sized, E: Env, Args: Clone + Sized + Send + Sync + 'static {
    type Built = T::Built;
    type Built2 = T::Built2;

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorEndBuilderDyn<Args,E>+'_) {
        (**self).erase()
    }
    #[inline]
    fn build(&self) -> Self::Built {
        (**self).build()
    }
    #[inline]
    fn build2(&self) -> Self::Built2 {
        (**self).build2()
    }
    #[inline]
    fn build_boxed(&self) -> Box<dyn MutorEnd<Args,E>> {
        (**self).build_boxed()
    }
    #[inline]
    fn build_arced(&self) -> Arc<dyn MutorEnd<Args,E>> {
        (**self).build_arced()
    }
}

impl<Args,Target,T,E> MutorToBuilder<Args,Target,E> for &T where T: MutorToBuilder<Args,Target,E> + ?Sized, E: Env, Args: Clone + Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    type Built = T::Built;
    type Built2 = T::Built2;

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorToBuilderDyn<Args,Target,E>+'_) {
        (**self).erase()
    }
    #[inline]
    fn build(&self) -> Self::Built {
        (**self).build()
    }
    #[inline]
    fn build2(&self) -> Self::Built2 {
        (**self).build2()
    }
    #[inline]
    fn build_boxed(&self) -> Box<dyn MutorTo<Args,Target,E>> {
        (**self).build_boxed()
    }
    #[inline]
    fn build_arced(&self) -> Arc<dyn MutorTo<Args,Target,E>> {
        (**self).build_arced()
    }
}

#[repr(transparent)]
pub struct ConvertToTargetBuilder<T,Target,NewTarget,Args,E>(PhantomData<(fn(Args),&'static Target,&'static NewTarget,E)>,T)
where
    E: Env,
    Args: Clone + Sized + Send + Sync + 'static,
    Target: MuTarget<E> + ?Sized,
    T: MutorToBuilder<Args,Target,E> + Sized,
    T::Built: Sized,
    ConvertToTargetor<T::Built,Target,NewTarget,Args,E>: Sized,
    for<'a> NewTarget: MuTarget<E,Mutable<'a>=Target::Mutable<'a>>;

impl<Args,Target,NewTarget,T,E> MutorToBuilder<Args,NewTarget,E> for ConvertToTargetBuilder<T,Target,NewTarget,Args,E>
where
    E: Env,
    Args: Clone + Sized + Send + Sync + 'static,
    Target: MuTarget<E> + ?Sized,
    T: MutorToBuilder<Args,Target,E> + Sized,
    T::Built: Sized,
    ConvertToTargetor<T::Built,Target,NewTarget,Args,E>: Sized,
    for<'a> NewTarget: MuTarget<E,Mutable<'a>=Target::Mutable<'a>>
{
    type Built = ConvertToTargetor<T::Built,Target,NewTarget,Args,E>;
    type Built2 = ConvertToTargetor<T::Built2,Target,NewTarget,Args,E>;

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorToBuilderDyn<Args,NewTarget,E>+'_) {
        self
    }
    
    #[inline]
    fn build(&self) -> Self::Built {
        ConvertToTargetor(PhantomData,self.1.build())
    }
    #[inline]
    fn build2(&self) -> Self::Built2 {
        ConvertToTargetor(PhantomData,self.1.build2())
    }
}

impl<Args,Target,NewTarget,T,E> Clone for ConvertToTargetBuilder<T,Target,NewTarget,Args,E>
where
    E: Env,
    Args: Clone + Sized + Send + Sync + 'static,
    Target: MuTarget<E> + ?Sized,
    T: MutorToBuilder<Args,Target,E> + Clone + Sized,
    T::Built: Sized,
    ConvertToTargetor<T::Built,Target,NewTarget,Args,E>: Sized,
    for<'a> NewTarget: MuTarget<E,Mutable<'a>=Target::Mutable<'a>>
{
    #[inline]
    fn clone(&self) -> Self {
        Self(PhantomData,self.1.clone())
    }
}

#[repr(transparent)]
pub struct ConvertToTargetor<T,Target,NewTarget,Args,E>(PhantomData<(fn(Args),&'static Target,&'static NewTarget,E)>,T)
where
    E: Env,
    Args: Clone + Sized + Send + Sync + 'static,
    Target: MuTarget<E> + ?Sized,
    T: MutorTo<Args,Target,E> + Sized,
    for<'a> NewTarget: MuTarget<E,Mutable<'a>=Target::Mutable<'a>>;

impl<Args,Target,NewTarget,T,E> MutorTo<Args,NewTarget,E> for ConvertToTargetor<T,Target,NewTarget,Args,E>
where
    E: Env,
    Args: Clone + Sized + Send + Sync + 'static,
    Target: MuTarget<E> + ?Sized,
    T: MutorTo<Args,Target,E> + Sized,
    for<'a> NewTarget: MuTarget<E,Mutable<'a>=Target::Mutable<'a>>
{
    #[inline]
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        args: Args,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        self.1.with_mutor_cb(root, callback, args, ctx)
    }
}

pub trait MutorEndBuilderDyn<Args,E>: Send + Sync where E: Env, Args: Clone + Sized + Send + Sync + 'static {
    fn _build_dyn(&self) -> Box<dyn MutorEnd<Args,E>>;
    fn _build2_dyn(&self) -> Arc<dyn MutorEnd<Args,E>>;}

pub trait MutorToBuilderDyn<Args,Target,E>: Send + Sync where E: Env, Args: Clone + Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    fn _build_dyn(&self) -> Box<dyn MutorTo<Args,Target,E>>;
    fn _build2_dyn(&self) -> Arc<dyn MutorTo<Args,Target,E>>;
}

impl<T,Args,E> MutorEndBuilderDyn<Args,E> for T where T: MutorEndBuilder<Args,E> + ?Sized, E: Env, Args: Clone + Sized + Send + Sync + 'static {
    #[inline]
    fn _build_dyn(&self) -> Box<dyn MutorEnd<Args,E>> {
        (*self).build_boxed()
    }
    #[inline]
    fn _build2_dyn(&self) -> Arc<dyn MutorEnd<Args,E>> {
        (*self).build_arced()
    }
}

impl<T,Target,Args,E> MutorToBuilderDyn<Args,Target,E> for T
where
    E: Env,
    T: MutorToBuilder<Args,Target,E> + ?Sized,
    Args: Clone + Sized + Send + Sync + 'static,
    Target: MuTarget<E> + ?Sized,
{
    #[inline]
    fn _build_dyn(&self) -> Box<dyn MutorTo<Args,Target,E>> {
        (*self).build_boxed()
        //ConvertToTargetor(PhantomData,(**self).build())
    }
    #[inline]
    fn _build2_dyn(&self) -> Arc<dyn MutorTo<Args,Target,E>> {
        (*self).build_arced()
        //ConvertToTargetor(PhantomData,(**self).build())
    }
}

impl<Args,E> MutorEndBuilder<Args,E> for dyn MutorEndBuilderDyn<Args,E> + '_ where E: Env, Args: Clone + Sized + Send + Sync + 'static {
    type Built = Box<dyn MutorEnd<Args,E>>;
    type Built2 = Arc<dyn MutorEnd<Args,E>>;

    #[inline]
    fn build(&self) -> Self::Built {
        (*self)._build_dyn()
    }
    #[inline]
    fn build2(&self) -> Self::Built2 {
        (*self)._build2_dyn()
    }

    #[inline]
    fn erase(&self) -> &(dyn MutorEndBuilderDyn<Args,E>+'_) {
        self
    }
}

impl<Args,Target,E> MutorToBuilder<Args,Target,E> for dyn MutorToBuilderDyn<Args,Target,E> + '_ where E: Env, Args: Clone + Sized + Send + Sync + 'static, Target: MuTarget<E> + ?Sized {
    type Built = Box<dyn MutorTo<Args,Target,E>>;
    type Built2 = Arc<dyn MutorTo<Args,Target,E>>;

    #[inline]
    fn build(&self) -> Self::Built {
        (*self)._build_dyn()
    }
    #[inline]
    fn build2(&self) -> Self::Built2 {
        (*self)._build2_dyn()
    }

    #[inline]
    fn erase(&self) -> &(dyn MutorToBuilderDyn<Args,Target,E>+'_) {
        self
    }
}
