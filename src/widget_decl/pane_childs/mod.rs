use std::any::Any;
use std::marker::PhantomData;

use crate::aliases::ESize;
use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStack, PathFragment, PathResolvusDyn};
use crate::traitcast::WQuery;
use crate::util::bounds::Bounds;
use crate::widget::pane_childs::{PaneChilds, PaneChildsDyn};

use super::route::UpdateRoute;

pub mod fixed_idx;

pub trait PaneChildsDecl<E> where E: Env {
    type Retained: PaneChilds<E> + 'static;

    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized;

    #[inline]
    fn build<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Self: Sized, Ph: PathStack<E> + ?Sized {
        self.instantiate(path, root, ctx)
    }

    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Ph: PathStack<E> + ?Sized;

    fn update<Ph>(
        &self,
        w: &mut Self::Retained,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized;

    fn update_restore<Ph>(
        &self,
        prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Retained where Ph: PathStack<E> + ?Sized;
}

pub struct WQueryAsWidgetsDyn<CID>(pub PhantomData<CID>);

impl<E,CID> WQuery<E> for WQueryAsWidgetsDyn<CID> where E: Env, CID: PathFragment<E> + Clone + 'static {
    type Result<'a> = &'a mut dyn PaneChildsDyn<E,ChildID=CID>;
}

impl<E,T> PaneChildsDecl<E> for &T where T: PaneChildsDecl<E> + ?Sized, E: Env {
    type Retained = T::Retained;

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
    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Ph: PathStack<E> + ?Sized {
        (**self).instantiate(path, root, ctx)
    }

    #[inline]
    fn update<Ph>(
        &self,
        w: &mut Self::Retained,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        (**self).update(w, path, route, root, ctx)
    }

    #[inline]
    fn update_restore<Ph>(
        &self,
        prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Retained where Ph: PathStack<E> + ?Sized {
        (**self).update_restore(prev, path, root, ctx)
    }
}
