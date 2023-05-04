use std::any::Any;
use std::marker::PhantomData;

use crate::aliases::ESize;
use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStack, PathFragment, PathResolvusDyn};
use crate::pathslice::{NewPathStack, PathSliceRef};
use crate::traitcast::WQuery;
use crate::util::bounds::Bounds;
use crate::widget::pane_childs::{PaneChilds, PaneChildsDyn};

use super::route::UpdateRoute;

pub mod fixed_idx;

pub trait PaneChildsDecl<E> where E: Env {
    type Retained: PaneChilds<E> + 'static;

    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    );

    #[inline]
    fn build(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Self: Sized {
        self.instantiate(path, root, ctx)
    }

    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained;

    fn update(
        &self,
        w: &mut Self::Retained,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation;

    fn update_restore(
        &self,
        prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Retained,Invalidation);
}

pub struct WQueryAsWidgetsDyn<CID>(pub PhantomData<CID>);

impl<E,CID> WQuery<E> for WQueryAsWidgetsDyn<CID> where E: Env, CID: PathFragment<E> + Clone + 'static {
    type Result<'a> = &'a mut dyn PaneChildsDyn<E,ChildID=CID>;
}

impl<E,T> PaneChildsDecl<E> for &T where T: PaneChildsDecl<E> + ?Sized, E: Env {
    type Retained = T::Retained;

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
    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained {
        (**self).instantiate(path, root, ctx)
    }

    #[inline]
    fn update(
        &self,
        w: &mut Self::Retained,
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
        prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Retained,Invalidation) {
        (**self).update_restore(prev, path, root, ctx)
    }
}
