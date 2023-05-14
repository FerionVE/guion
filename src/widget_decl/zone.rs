use std::any::Any;
use std::marker::PhantomData;

use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::pathslice::{NewPathStack, PathSliceRef};
use crate::widget::dyn_tunnel::WidgetDyn;

use super::WidgetDecl;
use super::route::UpdateRoute;

pub struct Zone<Z,T,E> where Z: 'static, T: WidgetDecl<E>, E: Env {
    inner: T,
    _p: PhantomData<(Z,E)>,
}

impl<Z,T,E> Zone<Z,T,E> where Z: 'static, T: WidgetDecl<E>, E: Env {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _p: PhantomData,
        }
    }
}

impl<Z,T,E> WidgetDecl<E> for Zone<Z,T,E> where Z: 'static, T: WidgetDecl<E>, E: Env {
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
        self.inner.send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized {
        self.inner.build(path, root, ctx)
    }
    #[inline]
    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget {
        self.inner.instantiate(path, root, ctx)
    }
    #[inline]
    fn build_boxed(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized {
        self.inner.build_boxed(path, root, ctx)
    }
    #[inline]
    fn instantiate_boxed(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        self.inner.instantiate_boxed(path, root, ctx)
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
        if let Some(route) = route.through_zone::<Z>() {
            self.inner.update(w, path, route, root, ctx)
        } else {
            Invalidation::new()
        }
    }
    #[inline]
    fn update_restore(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) {
        self.inner.update_restore(prev, path, root, ctx)
    }
    #[inline]
    fn update_restore_boxed(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation) {
        self.inner.update_restore_boxed(prev, path, root, ctx)
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
        if let Some(route) = route.through_zone::<Z>() {
            self.inner.update_dyn(w, path, route, root, ctx)
        } else {
            Invalidation::new()
        }
    }
}
