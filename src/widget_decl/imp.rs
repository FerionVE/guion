use std::any::{TypeId, Any};

use crate::env::Env;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::widget::Widget;
use crate::widget::dyn_tunnel::WidgetDyn;

use super::route::UpdateRoute;
use super::{WidgetDecl, WidgetDeclCallback, WidgetDeclCallbackResult};

impl<T,E> WidgetDecl<E> for &T where T: WidgetDecl<E> + ?Sized, E: Env {
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
        (**self).send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized, Ph: PathStack<E> + ?Sized {
        (*self).instantiate(path, root, ctx)
    }
    #[inline]
    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        (**self).instantiate(path, root, ctx)
    }
    #[inline]
    fn build_boxed<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized, Ph: PathStack<E> + ?Sized {
        (*self).instantiate_boxed(path, root, ctx)
    }
    #[inline]
    fn instantiate_boxed<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        (**self).instantiate_boxed(path, root, ctx)
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
        (**self).update(w, path, route, root, ctx)
    }
    #[inline]
    fn update_restore<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        (**self).update_restore(prev, path, root, ctx)
    }
    #[inline]
    fn update_restore_boxed<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        (**self).update_restore_boxed(prev, path, root, ctx)
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
        (**self).update_dyn(w, path, route, root, ctx)
    }
}

impl<T,E> WidgetDecl<E> for Box<T> where T: WidgetDecl<E> + ?Sized, E: Env {
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
        (**self).send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized, Ph: PathStack<E> + ?Sized {
        (*self).instantiate(path, root, ctx) //TODO function self can't receive unsized, do we need box_box equilavent
    }
    #[inline]
    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        (**self).instantiate(path, root, ctx)
    }
    #[inline]
    fn build_boxed<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized, Ph: PathStack<E> + ?Sized {
        (*self).instantiate_boxed(path, root, ctx)
    }
    #[inline]
    fn instantiate_boxed<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        (**self).instantiate_boxed(path, root, ctx)
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
        (**self).update(w, path, route, root, ctx)
    }
    #[inline]
    fn update_restore<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        (**self).update_restore(prev, path, root, ctx)
    }
    #[inline]
    fn update_restore_boxed<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        (**self).update_restore_boxed(prev, path, root, ctx)
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
    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        self.0.send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized, Ph: PathStack<E> + ?Sized {
        Box::new(self.0.build(path, root, ctx))
    }

    #[inline]
    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        Box::new(self.0.instantiate(path, root, ctx))
    }

    #[inline]
    fn build_boxed<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized, Ph: PathStack<E> + ?Sized {
        self.0.build_boxed(path, root, ctx)
    }

    #[inline]
    fn instantiate_boxed<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        self.0.instantiate_boxed(path, root, ctx)
    }

    #[inline]
    fn update<Ph>(
        &self,
        w: &mut Self::Widget,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
        self.0.update(w, path, route, root, ctx)
    }

    #[inline]
    fn update_restore<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        Box::new(self.0.update_restore(prev, path, root, ctx))
    }

    #[inline]
    fn update_restore_boxed<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        self.0.update_restore_boxed(prev, path, root, ctx)
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
    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        self.0.send_mutation(path, resolve, args, root, ctx)
    }

    #[inline]
    fn build<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized, Ph: PathStack<E> + ?Sized {
        self.0.build_boxed(path, root, ctx)
    }
    #[inline]
    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        self.0.instantiate_boxed(path, root, ctx)
    }
    #[inline]
    fn build_boxed<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized, Ph: PathStack<E> + ?Sized {
        self.0.build_boxed(path, root, ctx)
    }
    #[inline]
    fn instantiate_boxed<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        self.0.instantiate_boxed(path, root, ctx)
    }
    #[inline]
    fn update<Ph>(
        &self,
        w: &mut Self::Widget,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
        self.update_dyn(w, path, route, root, ctx)
    }
    #[inline]
    fn update_restore<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        self.0.update_restore_boxed(prev, path, root, ctx)
    }
    #[inline]
    fn update_restore_boxed<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        self.0.update_restore_boxed(prev, path, root, ctx)
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
        self.0.update_dyn(w, path, route, root, ctx)
    }
}
