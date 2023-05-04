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
use super::{WidgetDeclCallback, WidgetDecl, WidgetDeclCallbackMode, WidgetDeclCallbackResult};

pub struct SubDecl<W,F,E> 
where
    F: Fn(
        WidgetDeclCallback<'_,W,E>,
        &mut E::Context<'_>,
    ) -> WidgetDeclCallbackResult,
    W: Widget<E> + 'static,
    E: Env,
{
    decl: F,
    _p: PhantomData<(fn() -> W,E)>,
}

impl<W,F,E> SubDecl<W,F,E> 
where
    F: Fn(
        WidgetDeclCallback<'_,W,E>,
        &mut E::Context<'_>,
    ) -> WidgetDeclCallbackResult,
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
        WidgetDeclCallback<'_,Box<dyn WidgetDyn<E> + 'static>,E>,
        &mut E::Context<'_>,
    ) -> WidgetDeclCallbackResult,
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
        WidgetDeclCallback<'_,W,E>,
        &mut E::Context<'_>,
    ) -> WidgetDeclCallbackResult,
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
        let op = WidgetDeclCallback::new(
            root.fork(),
            path.fork(),
            UpdateRoute::none(),
            WidgetDeclCallbackMode::SendMutation(resolve, args),
        );

        (self.decl)(op, ctx);
    }

    #[inline]
    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget {
        let mut dest = None;
        
        let op = WidgetDeclCallback::new(
            root.fork(),
            path.fork(),
            UpdateRoute::none(),
            WidgetDeclCallbackMode::Instantiate(&mut dest),
        );

        (self.decl)(op, ctx);

        dest.unwrap()
    }
    #[inline]
    fn instantiate_boxed(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        let mut dest = None;
        
        let op = WidgetDeclCallback::new(
            root.fork(),
            path.fork(),
            UpdateRoute::none(),
            WidgetDeclCallbackMode::InstantiateBoxed(&mut dest),
        );

        (self.decl)(op, ctx);

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

        let op = WidgetDeclCallback::new(
            root.fork(),
            path.fork(),
            route,
            WidgetDeclCallbackMode::Update(w, &mut vali),
        );

        (self.decl)(op, ctx);

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
        
        let op = WidgetDeclCallback::new(
            root.fork(),
            path.fork(),
            UpdateRoute::none(),
            WidgetDeclCallbackMode::UpdateRestore(prev, &mut dest, &mut vali),
        );

        (self.decl)(op, ctx);

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
        
        let op = WidgetDeclCallback::new(
            root.fork(),
            path.fork(),
            UpdateRoute::none(),
            WidgetDeclCallbackMode::UpdateRestoreBoxed(prev, &mut dest, &mut vali),
        );

        (self.decl)(op, ctx);

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

        let op = WidgetDeclCallback::new(
            root.fork(),
            path.fork(),
            route,
            WidgetDeclCallbackMode::UpdateDyn(w, &mut vali),
        );

        (self.decl)(op, ctx);

        vali
    }
    #[inline]
    fn callback(self, v: WidgetDeclCallback<'_,Self::Widget,E>, ctx: &mut E::Context<'_>) -> super::WidgetDeclCallbackResult where Self: Sized {
        (self.decl)(v, ctx)
    }
    #[inline]
    fn call_on(&self, v: WidgetDeclCallback<'_,Self::Widget,E>, ctx: &mut E::Context<'_>) -> super::WidgetDeclCallbackResult {
        (self.decl)(v, ctx)
    }
}
