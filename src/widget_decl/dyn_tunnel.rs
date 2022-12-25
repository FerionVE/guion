use crate::env::Env;
use crate::newpath::{PathStackDyn, PathStack};
use crate::widget::dyn_tunnel::WidgetDyn;

use super::WidgetDecl;
use super::route::UpdateRoute;

pub trait WidgetDeclDyn<E> where E: Env {
    fn instantiate_dyn(&self, path: &(dyn PathStackDyn<E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static>;

    fn update_dyn_dyn(
        &self,
        w: &mut Box<dyn WidgetDyn<E> + 'static>,
        path: &(dyn PathStackDyn<E>+'_),
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    );

    fn update_restore_dyn(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &(dyn PathStackDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Box<dyn WidgetDyn<E> + 'static>;

    //fn call_on_dyn(&self, v: WidgetDeclCallbackDyn<'_,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult;
}

impl<T,E> WidgetDeclDyn<E> for T where T: WidgetDecl<E> + ?Sized, E: Env {
    #[inline]
    fn instantiate_dyn(&self, path: &(dyn PathStackDyn<E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        self.instantiate_boxed(path, root, ctx)
    }
    #[inline]
    fn update_dyn_dyn(
        &self,
        w: &mut Box<dyn WidgetDyn<E> + 'static>,
        path: &(dyn PathStackDyn<E>+'_),
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) {
        self.update_dyn(w, path, route, root, ctx)
    }
    #[inline]
    fn update_restore_dyn(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &(dyn PathStackDyn<E>+'_),
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Box<dyn WidgetDyn<E> + 'static> {
        self.update_restore_boxed(prev, path, root, ctx)
    }
}

impl<E> WidgetDecl<E> for dyn WidgetDeclDyn<E> + '_ where E: Env {
    type Widget = Box<dyn WidgetDyn<E> + 'static>;

    #[inline]
    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        self.instantiate_dyn(path._erase(), root, ctx)
    }
    #[inline]
    fn instantiate_boxed<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        self.instantiate_dyn(path._erase(), root, ctx)
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
        self.update_dyn(w, path, route, root, ctx)
    }
    #[inline]
    fn update_restore<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Widget where Ph: crate::newpath::PathStack<E> + ?Sized {
        self.update_restore_dyn(prev, path._erase(), root, ctx)
    }
    #[inline]
    fn update_restore_boxed<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        self.update_restore_dyn(prev, path._erase(), root, ctx)
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
        self.update_dyn_dyn(w, path._erase(), route, root, ctx)
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
