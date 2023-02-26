use std::any::Any;
use std::marker::PhantomData;

use crate::env::Env;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::widget::dyn_tunnel::WidgetDyn;

use super::WidgetDecl;
use super::mutor_trait::MutorEnd;
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
    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        self.inner.send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized, Ph: PathStack<E> + ?Sized {
        self.inner.build(path, root, ctx)
    }
    #[inline]
    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        self.inner.instantiate(path, root, ctx)
    }
    #[inline]
    fn build_boxed<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized, Ph: PathStack<E> + ?Sized {
        self.inner.build_boxed(path, root, ctx)
    }
    #[inline]
    fn instantiate_boxed<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        self.inner.instantiate_boxed(path, root, ctx)
    }
    #[inline]
    fn update<Ph>(
        &self,
        w: &mut Self::Widget,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: crate::newpath::PathStack<E> + ?Sized {
        if let Some(route) = route.through_zone::<Z>() {
            self.inner.update(w, path, route, root, ctx)
        }
    }
    #[inline]
    fn update_restore<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        self.inner.update_restore(prev, path, root, ctx)
    }
    #[inline]
    fn update_restore_boxed<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        self.inner.update_restore_boxed(prev, path, root, ctx)
    }
    #[inline]
    fn update_dyn<Ph>(
        &self,
        w: &mut Box<dyn WidgetDyn<E> + 'static>,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
        if let Some(route) = route.through_zone::<Z>() {
            self.inner.update_dyn(w, path, route, root, ctx)
        }
    }
}
