use std::marker::PhantomData;
use std::sync::Arc;

use crate::ProtectedReturn;
use crate::dispatchor::{ViewDispatch, ViewClosure};
use crate::env::Env;
use crate::error::ResolveResult;
use crate::widget::Widget;
use crate::widget::cache::{WidgetCache, WidgetCacheDyn, DynWidgetCache};
use crate::widget::dyn_tunnel::WidgetDyn;

pub mod view_widget;
pub mod apply;
pub mod message;
//pub mod test;

pub trait View<E> where E: Env {
    type Viewed<'v,'z,MutorFn>: Widget<E,Cache=Self::WidgetCache> + ?Sized + 'v where MutorFn: 'static, 'z: 'v, Self: 'z;
    type WidgetCache: WidgetCache<E>;
    type Mutable<'k>: ?Sized + 'k;

    fn view<'d,MutorFn,DispatchFn,R>(&self, dispatch: DispatchFn, mutor: MutorFn, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        MutorFn: for<'s,'c,'cc> Fn(
            E::RootMut<'s>,&'s (),
            &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Self::Mutable<'iss>>,&'iss (),&'c mut E::Context<'cc>)),
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static, //TODO does it also need Sync or only need to be Send?
        DispatchFn: ViewDispatch<'d,Self,MutorFn,R,E>, Self: 'd,
   ;
}

impl<T,E> View<E> for &'_ T where T: View<E> + ?Sized, E: Env {
    type Viewed<'v,'z,MutorFn> = T::Viewed<'v,'z,MutorFn> where MutorFn: 'static, 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;
    type Mutable<'k> = T::Mutable<'k>;

    #[inline]
    fn view<'d,MutorFn,DispatchFn,R>(&self, callback: DispatchFn, remut: MutorFn, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        MutorFn: for<'s,'c,'cc> Fn(
            E::RootMut<'s>,&'s (),
            &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Self::Mutable<'iss>>,&'iss (),&'c mut E::Context<'cc>)),
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static,
        DispatchFn: ViewDispatch<'d,Self,MutorFn,R,E>, Self: 'd,
    {
        let callback = ViewClosure::new(#[inline] move |widget,root,ctx|
            callback.call(widget, root, ctx)
        );
        (**self).view(callback,remut,root,ctx)
    }
}

pub trait ViewDyn<E> where E: Env {
    fn view_dyn(
        &self,
        dispatch: Box<dyn for<'w,'ww,'r,'c,'cc> FnOnce(&'w (dyn WidgetDyn<E>+'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> ProtectedReturn + '_>,
        remut: Arc<dyn for<'s,'c,'cc> Fn( //TODO Arc slow
            E::RootMut<'s>,&'s (),
            &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Timmy>,&'iss (),&'c mut E::Context<'cc>)),
            &'c mut E::Context<'cc>
        ) + Send + Sync + 'static>,
        root: E::RootRef<'_>, ctx: &mut E::Context<'_>
    ) -> ProtectedReturn;
}

impl<T,E> ViewDyn<E> for T where T: View<E>, E: Env {
    #[inline]
    fn view_dyn(
        &self,
        callback: Box<dyn for<'w,'ww,'r,'c,'cc> FnOnce(&'w (dyn WidgetDyn<E>+'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> ProtectedReturn + '_>,
        remut: Arc<dyn for<'s,'c,'cc> Fn(
            E::RootMut<'s>,&'s (),
            &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Timmy>,&'iss (),&'c mut E::Context<'cc>)),
            &'c mut E::Context<'cc>
        ) + Send + Sync + 'static>,
        root: E::RootRef<'_>, ctx: &mut E::Context<'_>
    ) -> ProtectedReturn {
        let callback = ViewClosure::new(#[inline] move |widget: &T::Viewed<'_,'_,_>,root,ctx|
            (callback)(widget.erase(), root, ctx)
        );
        View::view(
            self,
            callback,
            move |root,_,callback,ctx| {
                (remut)(root,&(),&mut move |resolved,&(),ctx| {
                    let resolved = resolved.expect("TODO");
                    (callback)(
                        Ok(resolved.into_everything::<Self,E>()),
                        &(),ctx
                    )
                },ctx)
            },
            root,
            ctx
        )
    }
}

impl<E> View<E> for dyn ViewDyn<E> + '_ where E: Env {
    type Viewed<'v,'z,MutorFn> = dyn WidgetDyn<E>+'v where MutorFn: 'static, 'z: 'v, Self: 'z;
    type WidgetCache = DynWidgetCache<E>;
    type Mutable<'k> = Timmy;

    #[inline]
    fn view<'d,MutorFn,DispatchFn,R>(&self, dispatch: DispatchFn, mutor: MutorFn, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        MutorFn: for<'s,'c,'cc> Fn(
            E::RootMut<'s>,&'s (),
            &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Self::Mutable<'iss>>,&'iss (),&'c mut E::Context<'cc>)),
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static,
        DispatchFn: ViewDispatch<'d,Self,MutorFn,R,E>, Self: 'd
    {
        let mut callback_return: Option<R> = None;
        self.view_dyn(
            Box::new(|widget,root,ctx| {
                callback_return = Some(dispatch.call(widget,root,ctx));
                ProtectedReturn(PhantomData)
            }),
            Arc::new(move |root,_,cb,ctx| {
                (mutor)(
                    root,&(),
                    &mut move |resolved: ResolveResult<&mut Timmy>,&(),ctx| {
                        (cb)(resolved,&(),ctx);
                    },
                    ctx
                )
            }),
            root,
            ctx
        );
        callback_return.unwrap()
    }
}

pub struct Timmy;

impl Timmy {
    fn into_everything<'v,'vv,V,E>(&mut self) -> &'v mut <V as View<E>>::Mutable<'vv> where V: View<E>, E: Env {
        todo!()
    }
}

pub trait ViewDyn2<E,M> where M: MuGator<E>, E: Env {
    fn view_dyn(
        &self,
        dispatch: Box<dyn for<'w,'ww,'r,'c,'cc> FnOnce(&'w (dyn WidgetDyn<E>+'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> ProtectedReturn + '_>,
        remut: Arc<dyn for<'s,'c,'cc> Fn(
            E::RootMut<'s>,&'s (),
            &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut M::Mutable<'iss>>,&'iss (),&'c mut E::Context<'cc>)),
            &'c mut E::Context<'cc>
        ) + Send + Sync + 'static>,
        root: E::RootRef<'_>, ctx: &mut E::Context<'_>
    ) -> ProtectedReturn;
}

pub trait MuGator<E>: 'static where E: Env {
    type Mutable<'k>: 'k;
}

impl<T,M,E> ViewDyn2<E,M> for T where for<'k> T: View<E,Mutable<'k>=M::Mutable<'k>>, M: MuGator<E>, E: Env {
    #[inline]
    fn view_dyn(
        &self,
        callback: Box<dyn for<'w,'ww,'r,'c,'cc> FnOnce(&'w (dyn WidgetDyn<E>+'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> ProtectedReturn + '_>,
        remut: Arc<dyn for<'s,'c,'cc> Fn(
            E::RootMut<'s>,&'s (),
            &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut M::Mutable<'iss>>,&'iss (),&'c mut E::Context<'cc>)),
            &'c mut E::Context<'cc>
        ) + Send + Sync + 'static>,
        root: E::RootRef<'_>, ctx: &mut E::Context<'_>
    ) -> ProtectedReturn {
        let callback = ViewClosure::new(#[inline] move |widget: &T::Viewed<'_,'_,_>,root,ctx|
            (callback)(widget.erase(), root, ctx)
        );
        View::view(
            self,
            callback,
            #[inline] move |root,_,cb,ctx| 
                (remut)(root,&(),cb,ctx),
            root,
            ctx
        )
    }
}

impl<M,E> View<E> for dyn ViewDyn2<E,M> + '_ where M: MuGator<E>, E: Env {
    type Viewed<'v,'z,MutFn> = dyn WidgetDyn<E>+'v where MutFn: 'static, 'z: 'v, Self: 'z;
    type WidgetCache = DynWidgetCache<E>;
    type Mutable<'k> = M::Mutable<'k>;

    #[inline]
    fn view<'d,MutorFn,DispatchFn,R>(&self, dispatch: DispatchFn, mutor: MutorFn, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        MutorFn: for<'s,'c,'cc> Fn(
            E::RootMut<'s>,&'s (),
            &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Self::Mutable<'iss>>,&'iss (),&'c mut E::Context<'cc>)),
            &'c mut E::Context<'cc>
        ) + Send + Sync + Clone + 'static,
        DispatchFn: ViewDispatch<'d,Self,MutorFn,R,E>, Self: 'd
    {
        let mut callback_return: Option<R> = None;
        self.view_dyn(
            Box::new(#[inline] |widget,root,ctx| {
                callback_return = Some(dispatch.call(widget,root,ctx));
                ProtectedReturn(PhantomData)
            }),
            Arc::new(#[inline] move |root,_,cb,ctx|
                (mutor)(root,&(),cb,ctx)
            ),
            root,
            ctx
        );
        callback_return.unwrap()
    }
}

// pub trait Timmy {
//     fn into_everything<'z,'v,V,E>(self) -> <V as View<'z,E>>::Mutable<'v> where V: View<'z,E>, E: Env;
// }
// impl<T> Timmy for T where T: ?Sized {
//     fn into_everything<'z,'v,V,E>(self) -> <V as View<'z,E>>::Mutable<'v> where V: View<'z,E>, E: Env {
//         todo!()
//     }
// }

// #[macro_export]
// macro_rules! impl_view {
//     (
//         $( < $($generics:path),* $(,)* > )?
//         for $ontype:ty :
//         <$life:lifetime> $mutfnroot:ty => $mutfndest:ty
//         $(where $($bounds:tt)+)?
//         {
//             $($impl:tt)*
//         }

//     ) => {
//         impl < E,MutFn, $( $($generics),* )? > $crate::view::View<E,MutFn>
//         for $ontype where
//             MutFn: for<$life,'ctx> Fn($mutfnroot,&$life (),&'ctx mut E::Context<'_>) -> $crate::error::ResolveResult<$mutfndest> + Clone + 'static,
//             E: $crate::env::Env,
//             $($($bounds)*)?

//         {
//             type Viewed = impl $crate::widget::Widget<E>;

//             $($impl)*
//         }
//     };
//     (
//         $( < $($generics:path),* $(,)* > )?
//         for $ontype:ty :
//         <$life:lifetime> $mutfndest:ty
//         $(where $($bounds:tt)+)?
//         {
//             $($impl:tt)*
//         }

//     ) => {
//         $crate::impl_view!(
//             $( < $($generics),* $(,)* > )?
//                 for $ontype :
//             <$life> <E as $crate::env::Env>::RootMut<$life> => $mutfndest
//             $(where $($bounds)+)?
//             {
//                 $($impl)*
//             }
//         );
//     };
// }

// #[macro_export]
// macro_rules! decl_dyn_view_type {
//     (
//         $dv:vis type $dest:ident  = 
//         <$life:lifetime> $mutfnroot:ty => $mutfndest:ty
//     ) => {
//         $dv type $dest<'view,E> = dyn $crate::view::View<
//             E,
//             std::sync::Arc<
//                 dyn for<$life,'ctx> Fn($mutfnroot,&$life (),&'ctx mut <E as $crate::env::Env>::Ctx<'_>) -> $crate::error::ResolveResult<$mutfndest> + 'static
//             >,
//             Viewed=Box<dyn $crate::widget::Widget<E>+'view>
//         >+'view;
//     };
//     (
//         $dv:vis type $dest:ident  = 
//         <$life:lifetime> $mutfndest:ty
//     ) => {
//         $crate::decl_dyn_view_type!(
//             $dv type $dest =
//             <$life> <E as $crate::env::Env>::RootMut<$life> => $mutfndest
//         );
//     };
// }

// decl_dyn_view_type!(
//     pub type ADyn = <'a> &'a mut test::A
// );
// decl_dyn_view_type!(
//     pub type BDyn = <'a> E::RootMut<'a> => &'a mut test::B
// );
