use std::marker::PhantomData;

use crate::env::Env;
use crate::newpath::{PathStack, PathResolvusDyn, PathFragment};
use crate::traitcast::WQuery;
use crate::widget::as_widgets::{AsWidgets, AsWidgetsDyn};

pub mod fixed_idx;

pub trait DeclList<E> where E: Env {
    type Retained: AsWidgets<E> + 'static;

    #[inline]
    fn build<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Self: Sized, Ph: PathStack<E> + ?Sized {
        self.instantiate(path, root, ctx)
    }

    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Ph: PathStack<E> + ?Sized;

    fn update<Ph>(
        &self,
        w: &mut Self::Retained,
        path: &Ph,
        resolve: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized;

    fn update_restore<Ph>(
        &self,
        prev: &mut dyn AsWidgetsDyn<E,ChildID=<Self::Retained as AsWidgetsDyn<E>>::ChildID>,
        path: &Ph,
        //resolve: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Retained where Ph: PathStack<E> + ?Sized;
}

pub struct WQueryAsWidgetsDyn<CID>(pub PhantomData<CID>);

impl<E,CID> WQuery<E> for WQueryAsWidgetsDyn<CID> where E: Env, CID: PathFragment<E> + Clone + 'static {
    type Result<'a> = &'a mut dyn AsWidgetsDyn<E,ChildID=CID>;
}

impl<E,T> DeclList<E> for &T where T: DeclList<E> + ?Sized, E: Env {
    type Retained = T::Retained;

    #[inline]
    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Ph: PathStack<E> + ?Sized {
        (**self).instantiate(path, root, ctx)
    }

    #[inline]
    fn update<Ph>(
        &self,
        w: &mut Self::Retained,
        path: &Ph,
        resolve: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
        (**self).update(w, path, resolve, root, ctx)
    }

    #[inline]
    fn update_restore<Ph>(
        &self,
        prev: &mut dyn AsWidgetsDyn<E,ChildID=<Self::Retained as AsWidgetsDyn<E>>::ChildID>,
        path: &Ph,
        //resolve: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Retained where Ph: PathStack<E> + ?Sized {
        (**self).update_restore(prev, path, root, ctx)
    }
}
