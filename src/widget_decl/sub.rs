use std::any::Any;
use std::marker::PhantomData;

use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::pathslice::{NewPathStack, PathSliceRef};
use crate::root::RootRef;
use crate::widget::Widget;
use crate::widget::dyn_tunnel::WidgetDyn;

use super::mutor_trait::MutorEnd;
use super::route::UpdateRoute;
use super::{DeclScope, WidgetDecl, WidgetDeclCallbackMode, DeclResult};

pub struct SubDecl<W,F,E> 
where
    F: Fn(
        DeclScope<'_,'_,W,E>,
    ) -> DeclResult,
    W: Widget<E> + 'static,
    E: Env,
{
    decl: F,
    _p: PhantomData<(fn() -> W,E)>,
}

impl<W,F,E> SubDecl<W,F,E> 
where
    F: Fn(
        DeclScope<'_,'_,W,E>,
    ) -> DeclResult,
    W: Widget<E> + 'static,
    E: Env,
{
    #[inline]
    pub fn new(decl: F) -> Self {
        Self {
            decl,
            _p: PhantomData,
        }
    }
}

impl<F,E> SubDecl<Box<dyn WidgetDyn<E> + 'static>,F,E> 
where
    F: Fn(
        DeclScope<'_,'_,Box<dyn WidgetDyn<E> + 'static>,E>,
    ) -> DeclResult,
    E: Env,
{
    #[inline]
    pub fn erased(decl: F) -> Self {
        Self {
            decl,
            _p: PhantomData,
        }
    }
}

impl<W,F,E> WidgetDecl<E> for SubDecl<W,F,E> 
where
    F: Fn(
        DeclScope<'_,'_,W,E>
    ) -> DeclResult,
    W: Widget<E> + 'static,
    E: Env,
{
    type Widget = W;
    
    #[inline]
    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        let op = DeclScope::new(
            root.fork(),
            ctx,
            path.fork(),
            UpdateRoute::none(),
            WidgetDeclCallbackMode::SendMutation(resolve, args),
        );

        (self.decl)(op);
    }

    #[inline]
    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget {
        let mut dest = None;
        
        let op = DeclScope::new(
            root.fork(),
            ctx,
            path.fork(),
            UpdateRoute::none(),
            WidgetDeclCallbackMode::Instantiate(&mut dest),
        );

        (self.decl)(op);

        dest.unwrap()
    }
    #[inline]
    fn instantiate_boxed(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        let mut dest = None;
        
        let op = DeclScope::new(
            root.fork(),
            ctx,
            path.fork(),
            UpdateRoute::none(),
            WidgetDeclCallbackMode::InstantiateBoxed(&mut dest),
        );

        (self.decl)(op);

        dest.unwrap()
    }
    #[inline]
    fn update(
        &self,
        w: &mut Self::Widget,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        let mut vali = Invalidation::new();

        let op = DeclScope::new(
            root.fork(),
            ctx,
            path.fork(),
            route,
            WidgetDeclCallbackMode::Update(w, &mut vali),
        );

        (self.decl)(op);

        vali
    }
    #[inline]
    fn update_restore(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) {
        let mut dest = None;
        let mut vali = Invalidation::new();
        
        let op = DeclScope::new(
            root.fork(),
            ctx,
            path.fork(),
            UpdateRoute::none(),
            WidgetDeclCallbackMode::UpdateRestore(prev, &mut dest, &mut vali),
        );

        (self.decl)(op);

        (dest.unwrap(), vali)
    }
    #[inline]
    fn update_restore_boxed(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation) {
        let mut dest = None;
        let mut vali = Invalidation::new();
        
        let op = DeclScope::new(
            root.fork(),
            ctx,
            path.fork(),
            UpdateRoute::none(),
            WidgetDeclCallbackMode::UpdateRestoreBoxed(prev, &mut dest, &mut vali),
        );

        (self.decl)(op);

        (dest.unwrap(), vali)
    }
    #[inline]
    fn update_dyn(
        &self,
        w: &mut Box<dyn WidgetDyn<E> + 'static>,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        let mut vali = Invalidation::new();

        let op = DeclScope::new(
            root.fork(),
            ctx,
            path.fork(),
            route,
            WidgetDeclCallbackMode::UpdateDyn(w, &mut vali),
        );

        (self.decl)(op);

        vali
    }
    #[inline]
    fn callback(self, v: DeclScope<'_,'_,Self::Widget,E>) -> super::DeclResult where Self: Sized {
        (self.decl)(v)
    }
    #[inline]
    fn call_on(&self, v: DeclScope<'_,'_,Self::Widget,E>) -> super::DeclResult {
        (self.decl)(v)
    }
}
