use std::marker::PhantomData;

use crate::ProtectedReturn;
use crate::env::Env;
use crate::widget::cache::{WidgetCache, DynWidgetCache};
use crate::widget::dyn_tunnel::WidgetDyn;

use self::mut_target::MuTarget;
use self::mutor_trait::{MutorToBuilderDyn, MutorToBuilder};

pub mod view_widget;
pub mod apply;
pub mod message;
//pub mod test;
pub mod mutor_trait;
pub mod mut_target;

pub type DynViewDispatch<'a,R,E> = &'a mut (dyn for<'w,'ww,'r,'c,'cc> FnMut(&'w (dyn WidgetDyn<E>+'ww),<E as Env>::RootRef<'r>,&'c mut <E as Env>::Context<'cc>) -> R + 'a);

pub trait View<E> where E: Env {
    type WidgetCache: WidgetCache<E>;
    type Mutarget: MuTarget<E>;

    fn view<R>(&self, dispatch: DynViewDispatch<'_,R,E>, mutor: &(dyn MutorToBuilderDyn<(),Self::Mutarget,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R;
}

impl<T,E> View<E> for &'_ T where T: View<E> + ?Sized, E: Env {
    type WidgetCache = T::WidgetCache;
    type Mutarget = T::Mutarget;

    #[inline]
    fn view<R>(&self, callback: DynViewDispatch<'_,R,E>, mutor: &(dyn MutorToBuilderDyn<(),Self::Mutarget,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    {
        (**self).view(callback,mutor,root,ctx)
    }
}

pub fn box_view_cb<'a,F,R,E>(f: F) -> F
where
    E: Env,
    F: for<'w,'ww,'r,'c,'cc> FnMut(&'w (dyn WidgetDyn<E>+'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> R + 'a
{
    f
}

pub trait ViewDyn2<E,M> where M: MuTarget<E>, E: Env {
    fn view_dyn(
        &self,
        dispatch: &mut (dyn for<'w,'ww,'r,'c,'cc> FnMut(&'w (dyn WidgetDyn<E>+'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> ProtectedReturn + '_),
        remut: &(dyn MutorToBuilderDyn<(),M,E>+'_),
        root: E::RootRef<'_>, ctx: &mut E::Context<'_>
    ) -> ProtectedReturn;
}

impl<T,M,E> ViewDyn2<E,M> for T where T: View<E>, for<'k> M: MuTarget<E,Mutable<'k>=<T::Mutarget as MuTarget<E>>::Mutable<'k>>, E: Env {
    #[inline]
    fn view_dyn(
        &self,
        callback: &mut (dyn for<'w,'ww,'r,'c,'cc> FnMut(&'w (dyn WidgetDyn<E>+'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> ProtectedReturn + '_),
        mutor: &(dyn MutorToBuilderDyn<(),M,E>+'_),
        root: E::RootRef<'_>, ctx: &mut E::Context<'_>
    ) -> ProtectedReturn {
        View::view(
            self,
            callback,
            mutor.convert_to_target().erase(),
            root,
            ctx
        )
    }
}

impl<M,E> View<E> for dyn ViewDyn2<E,M> + '_ where M: MuTarget<E>, E: Env {
    type WidgetCache = DynWidgetCache<E>;
    type Mutarget = M;

    #[inline]
    fn view<R>(&self, dispatch: DynViewDispatch<'_,R,E>, mutor: &(dyn MutorToBuilderDyn<(),M,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    {
        let mut callback_return: Option<R> = None;
        self.view_dyn(
            &mut |widget,root,ctx| {
                callback_return = Some((dispatch)(widget,root,ctx));
                ProtectedReturn(PhantomData)
            },
            mutor,
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
