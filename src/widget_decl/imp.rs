use std::any::{TypeId, Any};
use std::convert::Infallible;

use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::pathslice::{NewPathStack, PathSliceRef};
use crate::widget::Widget;
use crate::widget::dyn_tunnel::WidgetDyn;

use super::route::UpdateRoute;
use super::{WidgetDecl, WidgetDeclCallback, WidgetDeclCallbackResult};

impl<T,E> WidgetDecl<E> for &T where T: WidgetDecl<E> + ?Sized, E: Env {
    type Widget = T::Widget;

    #[inline]
    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        (**self).send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized {
        (*self).instantiate(path, root, ctx)
    }
    #[inline]
    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget {
        (**self).instantiate(path, root, ctx)
    }
    #[inline]
    fn build_boxed(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized {
        (*self).instantiate_boxed(path, root, ctx)
    }
    #[inline]
    fn instantiate_boxed(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        (**self).instantiate_boxed(path, root, ctx)
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
        (**self).update(w, path, route, root, ctx)
    }
    #[inline]
    fn update_restore(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) {
        (**self).update_restore(prev, path, root, ctx)
    }
    #[inline]
    fn update_restore_boxed(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation) {
        (**self).update_restore_boxed(prev, path, root, ctx)
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
        (**self).update_dyn(w, path, route, root, ctx)
    }
}

impl<T,E> WidgetDecl<E> for Box<T> where T: WidgetDecl<E> + ?Sized, E: Env {
    type Widget = T::Widget;

    #[inline]
    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        (**self).send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized {
        (*self).instantiate(path, root, ctx) //TODO function self can't receive unsized, do we need box_box equilavent
    }
    #[inline]
    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget {
        (**self).instantiate(path, root, ctx)
    }
    #[inline]
    fn build_boxed(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized {
        (*self).instantiate_boxed(path, root, ctx)
    }
    #[inline]
    fn instantiate_boxed(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        (**self).instantiate_boxed(path, root, ctx)
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
        (**self).update(w, path, route, root, ctx)
    }
    #[inline]
    fn update_restore(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) {
        (**self).update_restore(prev, path, root, ctx)
    }
    #[inline]
    fn update_restore_boxed(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation) {
        (**self).update_restore_boxed(prev, path, root, ctx)
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
        (**self).update_dyn(w, path, route, root, ctx)
    }
    #[inline]
    fn callback(self, v: WidgetDeclCallback<'_,Self::Widget,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult where Self: Sized {
        (*self).call_on(v, ctx) //TODO function self can't receive unsized, do we need box_box equilavent
    }
    #[inline]
    fn call_on(&self, v: WidgetDeclCallback<'_,Self::Widget,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult {
        (**self).call_on(v, ctx)
    }
}

#[repr(transparent)]
pub struct Boxed<T>(pub T);

impl<T,E> WidgetDecl<E> for Boxed<T> where T: WidgetDecl<E>, E: Env {
    type Widget = Box<T::Widget>;

    #[inline]
    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        self.0.send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized {
        Box::new(self.0.build(path, root, ctx))
    }

    #[inline]
    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget {
        Box::new(self.0.instantiate(path, root, ctx))
    }

    #[inline]
    fn build_boxed(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized {
        self.0.build_boxed(path, root, ctx)
    }

    #[inline]
    fn instantiate_boxed(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        self.0.instantiate_boxed(path, root, ctx)
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
        self.0.update(w, path, route, root, ctx)
    }

    #[inline]
    fn update_restore(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) {
        let (widget,vali) = self.0.update_restore(prev, path, root, ctx);
        (Box::new(widget), vali)
    }

    #[inline]
    fn update_restore_boxed(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation) {
        self.0.update_restore_boxed(prev, path, root, ctx)
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
        if TypeId::of::<Self::Widget>() == TypeId::of::<Box<dyn WidgetDyn<E> + 'static>>() {
            // This actually isn't possible
            debug_assert!(false, "WidgetDecl::Widget can't be unsized");
            return self.0.update_dyn(w, path, route, root, ctx);
        }

        if let Some(v) = w.downcast_mut::<Self::Widget>() {
            self.update(v, path, route, root, ctx)
        // } else if let Some(v) = w_any.downcast_mut::<T::Widget>() {
        //     self.0.update(v, path, resolve, ctx)
        } else {
            self.0.update_dyn(w, path, route, root, ctx)
        }
    }
}

#[repr(transparent)]
pub struct Erased<T>(pub T);

impl<T,E> WidgetDecl<E> for Erased<T> where T: WidgetDecl<E>, E: Env {
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
        self.0.send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized {
        self.0.build_boxed(path, root, ctx)
    }
    #[inline]
    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget {
        self.0.instantiate_boxed(path, root, ctx)
    }
    #[inline]
    fn build_boxed(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized {
        self.0.build_boxed(path, root, ctx)
    }
    #[inline]
    fn instantiate_boxed(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        self.0.instantiate_boxed(path, root, ctx)
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
        self.0.update_restore_boxed(prev, path, root, ctx)
    }
    #[inline]
    fn update_restore_boxed(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation) {
        self.0.update_restore_boxed(prev, path, root, ctx)
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
        self.0.update_dyn(w, path, route, root, ctx)
    }
}

impl<E> WidgetDecl<E> for Infallible where E: Env {
    type Widget = Infallible;

    fn send_mutation(
        &self,
        _: &mut NewPathStack,
        _: PathSliceRef,
        _: &dyn Any,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>,
    ) {
        match *self {}
    }

    fn instantiate(&self, _: &mut NewPathStack, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Self::Widget {
        match *self {}
    }

    fn update(
        &self,
        _: &mut Self::Widget,
        _: &mut NewPathStack,
        _: UpdateRoute<'_,E>,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>,
    ) -> Invalidation {
        match *self {}
    }

    fn update_restore(
        &self,
        _: &mut dyn WidgetDyn<E>,
        _: &mut NewPathStack,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) {
        match *self {}
    }
}
