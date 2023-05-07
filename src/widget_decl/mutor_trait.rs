use std::marker::PhantomData;
use std::sync::Arc;

use crate::ctx::queue::{BoxMutEvent, ArcMutEvent};
use crate::env::Env;
use crate::error::ResolveResult;

use super::mut_target::MuTarget;

pub trait MutorEndBuilder<E>: Send + Sync where E: Env {
    type Built: MutorEnd<E> + Sized + Send + Sync + 'static;
    type Built2: MutorEnd<E> + Sized + Send + Sync + 'static;

    fn erase<'a>(&'a self) -> &'a (dyn MutorEndBuilderDyn<E>+'_);

    fn build(&self) -> Self::Built;

    fn build2(&self) -> Self::Built2;

    #[inline]
    fn build_boxed(&self) -> Box<dyn MutorEnd<E>> {
        Box::new(self.build())
    }

    #[inline]
    fn build_arced(&self) -> Arc<dyn MutorEnd<E>> {
        Arc::new(self.build())
    }

    #[inline]
    fn build_box_mut_event(&self) -> Option<BoxMutEvent<E>> {
        let b = self.build();
        Some(Box::new(#[inline] move |root,_,ctx| b.with_mutor_end(root, ctx) ))
    }

    #[inline]
    fn build_arc_mut_event(&self) -> Option<ArcMutEvent<E>> {
        let b = self.build2();
        Some(Arc::new(#[inline] move |root,_,ctx| b.with_mutor_end(root, ctx) ))
    }
}

//pub type MutorEndBuilderDyn<'a,E> = dyn MutorEndBuilder<E,Built=Box<dyn MutorEnd<E>>> + 'a;

impl<E> MutorEndBuilder<E> for () where E: Env {
    type Built = ();
    type Built2 = ();

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorEndBuilderDyn<E>+'_) {
        self
    }

    #[inline]
    fn build(&self) -> Self::Built {}
    #[inline]
    fn build2(&self) -> Self::Built2 {}

    #[inline]
    fn build_box_mut_event(&self) -> Option<BoxMutEvent<E>> {
        None
    }
    #[inline]
    fn build_arc_mut_event(&self) -> Option<ArcMutEvent<E>> {
        None
    }
}

pub trait MutorEnd<E>: Send + Sync + 'static where E: Env {
    fn with_mutor_end<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c;

    // #[inline]
    // fn _boxed_end(&self) -> Box<dyn MutorEnd<E>+'static> {
    //     Box::new(self.clone())
    // }
}

impl<E> MutorEnd<E> for () where E: Env {
    #[inline]
    fn with_mutor_end<'s,'c,'cc>(
        &self,
        _: E::RootMut<'s>,
        _: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {}
}

pub trait MutorEndBuilderExt<E>: MutorEndBuilder<E> + Send + Sync where E: Env {
    
}
impl<T,E> MutorEndBuilderExt<E> for T where T: MutorEndBuilder<E> + Send + Sync + ?Sized, E: Env {}

pub trait MutorToBuilder<Target,E>: Send + Sync where E: Env, Target: MuTarget<E> + ?Sized {
    type Built: MutorTo<Target,E> + Sized + Send + Sync + 'static;
    type Built2: MutorTo<Target,E> + Sized + Send + Sync + 'static;

    fn erase<'a>(&'a self) -> &'a (dyn MutorToBuilderDyn<Target,E>+'_);

    #[inline]
    fn convert_to_target<NewTarget>(self) -> ConvertToTargetBuilder<Self,Target,NewTarget,E> where Self: Sized, for<'b> NewTarget: MuTarget<E,Mutable<'b>=Target::Mutable<'b>> {
        ConvertToTargetBuilder(PhantomData,self)
    }

    fn build(&self) -> Self::Built;
    fn build2(&self) -> Self::Built2;

    #[inline]
    fn build_boxed(&self) -> Box<dyn MutorTo<Target,E>> {
        Box::new(self.build())
    }

    #[inline]
    fn build_arced(&self) -> Arc<dyn MutorTo<Target,E>> {
        Arc::new(self.build())
    }
}

//pub type MutorToBuilderDyn<'a,Target,E> = dyn MutorToBuilder<Target,E,Built=Box<dyn MutorTo<Target,E>>> + 'a;

pub trait MutorTo<Target,E>: Send + Sync + 'static where E: Env, Target: MuTarget<E> + ?Sized {
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c;
}

pub trait MutorToBuilderExt<Target,E>: MutorToBuilder<Target,E> + Send + Sync where E: Env, Target: MuTarget<E> + ?Sized {
    // #[inline]
    // fn erase<'a>(&'a self) -> BoxingMutorToBuilder<Target,Self,E> {
    //     BoxingMutorToBuilder(PhantomData,self)
    // }

    // #[inline]
    // fn convert_to_target<'a,T>(&'a self) -> ConvertToTargetBuilder<'a,Self,Target,T,E> where for<'b> T: MuTarget<E,Mutable<'b>=Target::Mutable<'b>> {
    //     ConvertToTargetBuilder(PhantomData,self)
    // }

    #[inline]
    fn for_view_cb<NewTarget,RightFn>(self, fun: RightFn) -> ForTargetCBBuilder<Self,Target,NewTarget,RightFn,E>
    where
        Self: Sized,
        E: Env,
        NewTarget: MuTarget<E> + ?Sized,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            ResolveResult<&'s mut Target::Mutable<'ss>>,
            &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        ForTargetCBBuilder(self,fun,PhantomData)
    }

    #[inline]
    fn for_view_cb_if<NewTarget,RightFn>(self, fun: RightFn) -> ForTargetCBIfBuilder<Self,Target,NewTarget,RightFn,E>
    where
        Self: Sized,
        E: Env,
        NewTarget: MuTarget<E> + ?Sized,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            &'s mut Target::Mutable<'ss>,
            &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        ForTargetCBIfBuilder(self,fun,PhantomData)
    }

    #[inline]
    fn mutor_end<RightFn>(self, fun: RightFn) -> EndorBuilder<Self,Target,RightFn,E>
    where
        Self: Sized,
        E: Env,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            ResolveResult<&'s mut Target::Mutable<'ss>>,
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        EndorBuilder(self,fun,PhantomData)
    }

    #[inline]
    fn mutor_end_if<RightFn>(self, fun: RightFn) -> EndorIfBuilder<Self,Target,RightFn,E>
    where
        Self: Sized,
        E: Env,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            &'s mut Target::Mutable<'ss>,
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        EndorIfBuilder(self,fun,PhantomData)
    }
}
impl<Target,T,E> MutorToBuilderExt<Target,E> for T
where
    T: MutorToBuilder<Target,E> + Send + Sync + ?Sized,
    E: Env,
Target: MuTarget<E> + ?Sized
{}

pub struct ForTargetCBBuilder
    <LeftMutor,LeftTarget,RightTarget,RightFn,E>
    (LeftMutor,RightFn,PhantomData<(&'static RightTarget,&'static LeftTarget,E)>)
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static;

impl<LeftMutor,LeftTarget,RightTarget,RightFn,E> MutorToBuilder<RightTarget,E> for
ForTargetCBBuilder<LeftMutor,LeftTarget,RightTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = impl MutorTo<RightTarget,E>;
    type Built2 = impl MutorTo<RightTarget,E>;

    #[inline]
    fn erase(&self) -> &(dyn MutorToBuilderDyn<RightTarget,E>+'_) {
        self
    }

    #[inline]
    fn build(&self) -> Self::Built {
        let left = self.0.build();
        let fun = self.1.clone();

        MutorForTarget::new(#[inline] move |root,callback,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    (fun)(med,callback,ctx)
                },
                ctx
            )
        })
    }

    #[inline]
    fn build2(&self) -> Self::Built2 {
        let left = self.0.build2();
        let fun = self.1.clone();

        MutorForTarget::new(#[inline] move |root,callback,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    (fun)(med,callback,ctx)
                },
                ctx
            )
        })
    }
}

impl<LeftMutor,LeftTarget,RightTarget,RightFn,E> Clone for
ForTargetCBBuilder<LeftMutor,LeftTarget,RightTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Clone + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), PhantomData)
    }
}

pub struct ForTargetCBIfBuilder
    <LeftMutor,LeftTarget,RightTarget,RightFn,E>
    (LeftMutor,RightFn,PhantomData<(&'static RightTarget,&'static LeftTarget,E)>)
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static;

impl<LeftMutor,LeftTarget,RightTarget,RightFn,E> MutorToBuilder<RightTarget,E> for
ForTargetCBIfBuilder<LeftMutor,LeftTarget,RightTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = impl MutorTo<RightTarget,E>;
    type Built2 = impl MutorTo<RightTarget,E>;

    #[inline]
    fn erase(&self) -> &(dyn MutorToBuilderDyn<RightTarget,E>+'_) {
        self
    }

    #[inline]
    fn build(&self) -> Self::Built {
        let left = self.0.build();
        let fun = self.1.clone();

        MutorForTarget::new(#[inline] move |root,callback,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    match med {
                        Ok(v) => (fun)(v,callback,ctx),
                        Err(e) => (callback)(Err(e),ctx),
                    }
                },
                ctx
            )
        })
    }

    #[inline]
    fn build2(&self) -> Self::Built2 {
        let left = self.0.build2();
        let fun = self.1.clone();

        MutorForTarget::new(#[inline] move |root,callback,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    match med {
                        Ok(v) => (fun)(v,callback,ctx),
                        Err(e) => (callback)(Err(e),ctx),
                    }
                },
                ctx
            )
        })
    }
}

impl<LeftMutor,LeftTarget,RightTarget,RightFn,E> Clone for
ForTargetCBIfBuilder<LeftMutor,LeftTarget,RightTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Clone + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut RightTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), PhantomData)
    }
}

pub struct EndorBuilder
    <LeftMutor,LeftTarget,RightFn,E>
    (LeftMutor,RightFn,PhantomData<(&'static LeftTarget,E)>)
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static;

impl<LeftMutor,LeftTarget,RightFn,E> MutorEndBuilder<E> for
EndorBuilder<LeftMutor,LeftTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = impl MutorEnd<E>;
    type Built2 = impl MutorEnd<E>;

    #[inline]
    fn erase(&self) -> &(dyn MutorEndBuilderDyn<E>+'_) {
        self
    }

    #[inline]
    fn build(&self) -> Self::Built {
        let left = self.0.build();
        let fun = self.1.clone();

        MutorEnde::new(#[inline] move |root,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    (fun)(med,ctx)
                },
                ctx
            )
        })
    }

    #[inline]
    fn build2(&self) -> Self::Built2 {
        let left = self.0.build2();
        let fun = self.1.clone();

        MutorEnde::new(#[inline] move |root,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    (fun)(med,ctx)
                },
                ctx
            )
        })
    }
}

impl<LeftMutor,LeftTarget,RightFn,E> Clone for
EndorBuilder<LeftMutor,LeftTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Clone + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        ResolveResult<&'s mut LeftTarget::Mutable<'ss>>,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), PhantomData)
    }
}

pub struct EndorIfBuilder
    <LeftMutor,LeftTarget,RightFn,E>
    (LeftMutor,RightFn,PhantomData<(&'static LeftTarget,E)>)
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static;

impl<LeftMutor,LeftTarget,RightFn,E> MutorEndBuilder<E> for
EndorIfBuilder<LeftMutor,LeftTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = impl MutorEnd<E>;
    type Built2 = impl MutorEnd<E>;

    #[inline]
    fn erase(&self) -> &(dyn MutorEndBuilderDyn<E>+'_) {
        self
    }

    #[inline]
    fn build(&self) -> Self::Built {
        let left = self.0.build();
        let fun = self.1.clone();

        MutorEnde::new(#[inline] move |root,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    match med {
                        Ok(v) => (fun)(v,ctx),
                        Err(e) => {}, //TODO detect lost mutor debug mode
                    }
                },
                ctx
            )
        })
    }

    #[inline]
    fn build2(&self) -> Self::Built2 {
        let left = self.0.build2();
        let fun = self.1.clone();

        MutorEnde::new(#[inline] move |root,ctx| {
            left.with_mutor_cb(
                root,
                &mut |med,ctx| {
                    match med {
                        Ok(v) => (fun)(v,ctx),
                        Err(e) => {}, //TODO detect lost mutor debug mode
                    }
                },
                ctx
            )
        })
    }
}

impl<LeftMutor,LeftTarget,RightFn,E> Clone for
EndorIfBuilder<LeftMutor,LeftTarget,RightFn,E>
where
    E: Env,
    LeftMutor: MutorToBuilder<LeftTarget,E> + Clone + Sized,
    LeftTarget: MuTarget<E> + ?Sized,
    RightFn: for<'s,'ss,'c,'cc> Fn(
        &'s mut LeftTarget::Mutable<'ss>,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), PhantomData)
    }
}

pub struct MutorForTarget<Targ,MutorFn,E>(MutorFn,PhantomData<(&'static Targ,E)>)
where
    Self: 'static,
    E: Env,
    Targ: MuTarget<E> + ?Sized,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static;

impl<Targ,MutorFn,E> MutorForTarget<Targ,MutorFn,E>
where
    Self: 'static,
    E: Env,
    Targ: MuTarget<E> + ?Sized,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static
{
    pub fn new(f: MutorFn) -> Self {
        Self(f,PhantomData)
    }
}

impl<Targ,MutorFn,E> MutorToBuilder<Targ,E> for MutorForTarget<Targ,MutorFn,E>
where
    Self: 'static,
    E: Env,
    Targ: MuTarget<E> + ?Sized,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = Self;
    type Built2 = Self;

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorToBuilderDyn<Targ,E>+'_) {
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

impl<Targ,MutorFn,E> MutorTo<Targ,E> for MutorForTarget<Targ,MutorFn,E>
where
    Self: 'static,
    E: Env,
    Targ: MuTarget<E> + ?Sized,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static
{
    #[inline]
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Targ::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (self.0)(root,callback,ctx)
    }
}

pub struct MutorEnde<MutorFn,E>(MutorFn,PhantomData<E>)
where
    Self: 'static,
    E: Env,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static;

impl<MutorFn,E> MutorEnde<MutorFn,E>
where
    Self: 'static,
    E: Env,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static
{
    #[inline]
    pub fn new(f: MutorFn) -> Self {
        Self(f,PhantomData)
    }
}

impl<MutorFn,E> MutorEndBuilder<E> for MutorEnde<MutorFn,E>
where
    Self: 'static,
    E: Env,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &'c mut E::Context<'cc>
    ) + Clone + Send + Sync + 'static
{
    type Built = Self;
    type Built2 = Self;

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorEndBuilderDyn<E>+'_) {
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

impl<MutorFn,E> MutorEnd<E> for MutorEnde<MutorFn,E>
where
    Self: 'static,
    E: Env,
    MutorFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,
        &'c mut E::Context<'cc>
    ) + Send + Sync + 'static
{
    #[inline]
    fn with_mutor_end<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (self.0)(root,ctx);
    }
}

impl<T,E> MutorEnd<E> for Box<T> where T: MutorEnd<E> + ?Sized, E: Env {
    #[inline]
    fn with_mutor_end<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (**self).with_mutor_end(root, ctx)
    }
}

impl<Target,T,E> MutorTo<Target,E> for Box<T> where T: MutorTo<Target,E> + ?Sized, E: Env, Target: MuTarget<E> + ?Sized {
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (**self).with_mutor_cb(root, callback, ctx)
    }
}

impl<T,E> MutorEnd<E> for Arc<T> where T: MutorEnd<E> + ?Sized, E: Env {
    #[inline]
    fn with_mutor_end<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (**self).with_mutor_end(root, ctx)
    }
}

impl<Target,T,E> MutorTo<Target,E> for Arc<T> where T: MutorTo<Target,E> + ?Sized, E: Env, Target: MuTarget<E> + ?Sized {
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut Target::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        (**self).with_mutor_cb(root, callback, ctx)
    }
}

impl<T,E> MutorEndBuilder<E> for &T where T: MutorEndBuilder<E> + ?Sized, E: Env {
    type Built = T::Built;
    type Built2 = T::Built2;

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorEndBuilderDyn<E>+'_) {
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
    fn build_boxed(&self) -> Box<dyn MutorEnd<E>> {
        (**self).build_boxed()
    }
    #[inline]
    fn build_arced(&self) -> Arc<dyn MutorEnd<E>> {
        (**self).build_arced()
    }
}

impl<Target,T,E> MutorToBuilder<Target,E> for &T where T: MutorToBuilder<Target,E> + ?Sized, E: Env, Target: MuTarget<E> + ?Sized {
    type Built = T::Built;
    type Built2 = T::Built2;

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorToBuilderDyn<Target,E>+'_) {
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
    fn build_boxed(&self) -> Box<dyn MutorTo<Target,E>> {
        (**self).build_boxed()
    }
    #[inline]
    fn build_arced(&self) -> Arc<dyn MutorTo<Target,E>> {
        (**self).build_arced()
    }
}

#[repr(transparent)]
pub struct ConvertToTargetBuilder<T,Target,NewTarget,E>(PhantomData<(&'static Target,&'static NewTarget,E)>,T)
where
    E: Env,
    Target: MuTarget<E> + ?Sized,
    T: MutorToBuilder<Target,E> + Sized,
    T::Built: Sized,
    ConvertToTargetor<T::Built,Target,NewTarget,E>: Sized,
    for<'a> NewTarget: MuTarget<E,Mutable<'a>=Target::Mutable<'a>>;

impl<Target,NewTarget,T,E> MutorToBuilder<NewTarget,E> for ConvertToTargetBuilder<T,Target,NewTarget,E>
where
    E: Env,
    Target: MuTarget<E> + ?Sized,
    T: MutorToBuilder<Target,E> + Sized,
    T::Built: Sized,
    ConvertToTargetor<T::Built,Target,NewTarget,E>: Sized,
    for<'a> NewTarget: MuTarget<E,Mutable<'a>=Target::Mutable<'a>>
{
    type Built = ConvertToTargetor<T::Built,Target,NewTarget,E>;
    type Built2 = ConvertToTargetor<T::Built2,Target,NewTarget,E>;

    #[inline]
    fn erase<'a>(&'a self) -> &'a (dyn MutorToBuilderDyn<NewTarget,E>+'_) {
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

impl<Target,NewTarget,T,E> Clone for ConvertToTargetBuilder<T,Target,NewTarget,E>
where
    E: Env,
    Target: MuTarget<E> + ?Sized,
    T: MutorToBuilder<Target,E> + Clone + Sized,
    T::Built: Sized,
    ConvertToTargetor<T::Built,Target,NewTarget,E>: Sized,
    for<'a> NewTarget: MuTarget<E,Mutable<'a>=Target::Mutable<'a>>
{
    #[inline]
    fn clone(&self) -> Self {
        Self(PhantomData,self.1.clone())
    }
}

#[repr(transparent)]
pub struct ConvertToTargetor<T,Target,NewTarget,E>(PhantomData<(&'static Target,&'static NewTarget,E)>,T)
where
    E: Env,
    Target: MuTarget<E> + ?Sized,
    T: MutorTo<Target,E> + Sized,
    for<'a> NewTarget: MuTarget<E,Mutable<'a>=Target::Mutable<'a>>;

impl<Target,NewTarget,T,E> MutorTo<NewTarget,E> for ConvertToTargetor<T,Target,NewTarget,E>
where
    E: Env,
    Target: MuTarget<E> + ?Sized,
    T: MutorTo<Target,E> + Sized,
    for<'a> NewTarget: MuTarget<E,Mutable<'a>=Target::Mutable<'a>>
{
    #[inline]
    fn with_mutor_cb<'s,'c,'cc>(
        &self,
        root: E::RootMut<'s>,
        callback: &mut (dyn for<'is,'iss,'ic,'icc> FnMut(ResolveResult<&'is mut NewTarget::Mutable<'iss>>,&'ic mut E::Context<'icc>)),
        ctx: &'c mut E::Context<'cc>,
    ) where 'cc: 'c {
        self.1.with_mutor_cb(root, callback, ctx)
    }
}

pub trait MutorEndBuilderDyn<E>: Send + Sync where E: Env {
    fn _build_dyn(&self) -> Box<dyn MutorEnd<E>>;
    fn _build2_dyn(&self) -> Arc<dyn MutorEnd<E>>;}

pub trait MutorToBuilderDyn<Target,E>: Send + Sync where E: Env, Target: MuTarget<E> + ?Sized {
    fn _build_dyn(&self) -> Box<dyn MutorTo<Target,E>>;
    fn _build2_dyn(&self) -> Arc<dyn MutorTo<Target,E>>;
}

impl<T,E> MutorEndBuilderDyn<E> for T where T: MutorEndBuilder<E> + ?Sized, E: Env {
    #[inline]
    fn _build_dyn(&self) -> Box<dyn MutorEnd<E>> {
        (*self).build_boxed()
    }
    #[inline]
    fn _build2_dyn(&self) -> Arc<dyn MutorEnd<E>> {
        (*self).build_arced()
    }
}

impl<T,Target,E> MutorToBuilderDyn<Target,E> for T
where
    E: Env,
    T: MutorToBuilder<Target,E> + ?Sized,
    Target: MuTarget<E> + ?Sized,
{
    #[inline]
    fn _build_dyn(&self) -> Box<dyn MutorTo<Target,E>> {
        (*self).build_boxed()
        //ConvertToTargetor(PhantomData,(**self).build())
    }
    #[inline]
    fn _build2_dyn(&self) -> Arc<dyn MutorTo<Target,E>> {
        (*self).build_arced()
        //ConvertToTargetor(PhantomData,(**self).build())
    }
}

impl<E> MutorEndBuilder<E> for dyn MutorEndBuilderDyn<E> + '_ where E: Env {
    type Built = Box<dyn MutorEnd<E>>;
    type Built2 = Arc<dyn MutorEnd<E>>;

    #[inline]
    fn build(&self) -> Self::Built {
        (*self)._build_dyn()
    }
    #[inline]
    fn build2(&self) -> Self::Built2 {
        (*self)._build2_dyn()
    }

    #[inline]
    fn erase(&self) -> &(dyn MutorEndBuilderDyn<E>+'_) {
        self
    }
}

impl<Target,E> MutorToBuilder<Target,E> for dyn MutorToBuilderDyn<Target,E> + '_ where E: Env, Target: MuTarget<E> + ?Sized {
    type Built = Box<dyn MutorTo<Target,E>>;
    type Built2 = Arc<dyn MutorTo<Target,E>>;

    #[inline]
    fn build(&self) -> Self::Built {
        (*self)._build_dyn()
    }
    #[inline]
    fn build2(&self) -> Self::Built2 {
        (*self)._build2_dyn()
    }

    #[inline]
    fn erase(&self) -> &(dyn MutorToBuilderDyn<Target,E>+'_) {
        self
    }
}
