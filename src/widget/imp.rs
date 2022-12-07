use std::convert::Infallible;

use crate::env::Env;

use super::Widget;

impl<E> Widget<E> for Infallible where E: Env {
    type Cache = ();

    fn _render<P,Ph>(
        &self,
        _: &Ph,
        _: &P,
        _: &mut crate::aliases::ERenderer<'_,E>,
        _: bool,
        _: &mut Self::Cache,
        _: <E as crate::env::Env>::RootRef<'_>,
        _: &mut <E as crate::env::Env>::Context<'_>
    ) where Ph: crate::newpath::PathStack<E> + ?Sized, P: crate::queron::Queron<E> + ?Sized {
        unreachable!()
    }

    fn _event_direct<P,Ph,Evt>(
        &self,
        _: &Ph,
        _: &P,
        _: &Evt,
        _: Option<&(dyn crate::newpath::PathResolvusDyn<E>+'_)>,
        _: &mut Self::Cache,
        _: <E as crate::env::Env>::RootRef<'_>,
        _: &mut <E as crate::env::Env>::Context<'_>
    ) -> crate::EventResp where Ph: crate::newpath::PathStack<E> + ?Sized, P: crate::queron::Queron<E> + ?Sized, Evt: crate::event_new::Event<E> + ?Sized {
        unreachable!()
    }

    fn _size<P,Ph>(
        &self,
        _: &Ph,
        _: &P,
        _: &mut Self::Cache,
        _: <E as crate::env::Env>::RootRef<'_>,
        _: &mut <E as crate::env::Env>::Context<'_>
    ) -> crate::aliases::ESize<E> where Ph: crate::newpath::PathStack<E> + ?Sized, P: crate::queron::Queron<E> + ?Sized {
        unreachable!()
    }

    fn childs(&self) -> usize {
        unreachable!()
    }

    fn with_child<'s,F,R>(
        &'s self,
        _: usize,
        _: F,
        _: <E as crate::env::Env>::RootRef<'s>,
        _: &mut <E as crate::env::Env>::Context<'_>
    ) -> R
    where
        F: for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn super::dyn_tunnel::WidgetDyn<E>+'ww),()>,&'c mut <E as crate::env::Env>::Context<'cc>) -> R
    {
        unreachable!()
    }

    fn with_resolve_child<'s,F,R>(
        &'s self,
        _: &(dyn crate::newpath::PathResolvusDyn<E>+'_),
        _: F,
        _: <E as crate::env::Env>::RootRef<'s>,
        _: &mut <E as crate::env::Env>::Context<'_>
    ) -> R
    where
        F: for<'w,'c,'cc> FnMut(Result<super::WidgetWithResolveChildDyn<'w,E>,<E as crate::env::Env>::Error>,&'c mut <E as crate::env::Env>::Context<'cc>) -> R
    {
        unreachable!()
    }

    fn focusable(&self) -> bool {
        unreachable!()
    }

    fn _call_tabulate_on_child_idx<P,Ph>(&self, _: usize, _: &Ph, _: &P, _: crate::util::tabulate::TabulateOrigin<E>, _: crate::util::tabulate::TabulateDirection, _: <E as crate::env::Env>::RootRef<'_>, _: &mut <E as crate::env::Env>::Context<'_>) -> Result<crate::util::tabulate::TabulateResponse<E>,<E as crate::env::Env>::Error> where Ph: crate::newpath::PathStack<E> + ?Sized, P: crate::queron::Queron<E> + ?Sized {
        unreachable!()
    }

    fn respond_query<'a>(&'a self, _: crate::traitcast::WQueryResponder<'_,'a,E>) {
        unreachable!()
    }
}
