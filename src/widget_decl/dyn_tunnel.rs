use std::any::Any;

use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStackDyn, PathStack, PathResolvusDyn};
use crate::pathslice::{NewPathStack, PathSliceRef};
use crate::widget::dyn_tunnel::WidgetDyn;

use super::WidgetDecl;
use super::route::UpdateRoute;

pub trait WidgetDeclDyn<E> where E: Env {
    fn send_mutation_dyn(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    );

    fn instantiate_dyn(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static>;

    fn update_dyn_dyn(
        &self,
        w: &mut Box<dyn WidgetDyn<E> + 'static>,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation;

    fn update_restore_dyn(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation);

    //fn call_on_dyn(&self, v: WidgetDeclCallbackDyn<'_,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult;
}

impl<T,E> WidgetDeclDyn<E> for T where T: WidgetDecl<E> + ?Sized, E: Env {
    #[inline]
    fn send_mutation_dyn(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        self.send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn instantiate_dyn(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        self.instantiate_boxed(path, root, ctx)
    }
    #[inline]
    fn update_dyn_dyn(
        &self,
        w: &mut Box<dyn WidgetDyn<E> + 'static>,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        self.update_dyn(w, path, route, root, ctx)
    }
    #[inline]
    fn update_restore_dyn(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation) {
        self.update_restore_boxed(prev, path, root, ctx)
    }
}

impl<E> WidgetDecl<E> for dyn WidgetDeclDyn<E> + '_ where E: Env {
    type Widget = Box<dyn WidgetDyn<E> + 'static>;

    #[inline]
    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        self.send_mutation_dyn(path, resolve, args, root, ctx)
    }

    #[inline]
    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget {
        self.instantiate_dyn(path, root, ctx)
    }
    #[inline]
    fn instantiate_boxed(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        self.instantiate_dyn(path, root, ctx)
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
        self.update_dyn(w, path, route, root, ctx)
    }
    #[inline]
    fn update_restore(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) {
        self.update_restore_dyn(prev, path, root, ctx)
    }
    #[inline]
    fn update_restore_boxed(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation) {
        self.update_restore_dyn(prev, path, root, ctx)
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
        self.update_dyn_dyn(w, path, route, root, ctx)
    }
    // #[inline]
    // fn callback(self, v: WidgetDeclCallback<'_,Self::Widget,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult where Self: Sized {
    //     match v.command {
    //         WidgetDeclCallbackMode::Instantiate(dest) =>
    //             *dest = Some(self.build(v.path, ctx)),
    //         WidgetDeclCallbackMode::InstantiateBoxed(dest) =>
    //             *dest = Some(self.build_boxed(v.path, ctx)),
    //         WidgetDeclCallbackMode::Update(widget) =>
    //             self.update(widget, v.path, v.resolve, ctx),
    //         // super::WidgetDeclCallbackMode::UpdateRestore(prev, dest) =>
    //         //     *dest = Some(self.update_restore(prev, v.path, v.resolve, ctx)),
    //         WidgetDeclCallbackMode::Phantom(_) =>
    //             unsafe { std::hint::unreachable_unchecked() },
    //     }
    //     WidgetDeclCallbackResult(std::marker::PhantomData)
    // }
    // #[inline]
    // fn call_on(&self, v: WidgetDeclCallback<'_,Self::Widget,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult {
    //     match v.command {
    //         WidgetDeclCallbackMode::Instantiate(dest) =>
    //             *dest = Some(self.build(v.path, ctx)),
    //         WidgetDeclCallbackMode::InstantiateBoxed(dest) =>
    //             *dest = Some(self.build_boxed(v.path, ctx)),
    //         WidgetDeclCallbackMode::Update(widget) =>
    //             self.update(widget, v.path, v.resolve, ctx),
    //         // super::WidgetDeclCallbackMode::UpdateRestore(prev, dest) =>
    //         //     *dest = Some(self.update_restore(prev, v.path, v.resolve, ctx)),
    //         WidgetDeclCallbackMode::Phantom(_) =>
    //             unsafe { std::hint::unreachable_unchecked() },
    //     }
    //     WidgetDeclCallbackResult(std::marker::PhantomData)
    // }
}
