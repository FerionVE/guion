use crate::dispatchor::ViewDispatch;
use crate::env::Env;
use crate::error::ResolveResult;
use crate::widget::Widget;

pub mod view_widget;
//pub mod test;

pub trait View<'z,E> where E: Env, Self: 'z {
    type Viewed<'v,MutFn>: Widget<E> + ?Sized + 'v where MutFn: 'static, 'z: 'v;
    type Mutable<'k>: 'k;

    fn view<'d,MutorFn,DispatchFn>(&'d self, dispatch: DispatchFn, mutor: MutorFn, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        MutorFn: for<'s,'c,'cc> Fn(E::RootMut<'s>,&'s (),&'c mut E::Context<'cc>) -> ResolveResult<Self::Mutable<'s>> + Clone + 'static,
        DispatchFn: ViewDispatch<'z,Self,MutorFn,E>;
}

// #[macro_export]
// macro_rules! impl_view {
//     (
//         $e:ty;
//         ($($generics:tt)*)
//         for $ontype:ty :
//         <$life:lifetime> $mutfnroot:ty => $mutfndest:ty
//         $(where ($($bounds:tt)+))?
//         {
//             $($impl:item)*
//         }

//     ) => {
//         impl < $($generics)*, MutFn > $crate::view::View<$e,MutFn>
//         for $ontype where
//             MutFn: for<$life,'ctx> Fn($mutfnroot,&$life (),&'ctx mut <$e as $crate::env::Env>::Context<'_>) -> $crate::error::ResolveResult<$mutfndest> + Clone + Send + Sync + 'static,
//             $e: $crate::env::Env,
//             $($($bounds)*)?

//         {
//             type Viewed = impl $crate::widget::Widget<$e>;

//             $($impl)*
//         }
//     };
//     (
//         $e:ty;
//         ($($generics:tt)*)
//         for $ontype:ty :
//         <$life:lifetime> $mutfndest:ty
//         $(where ($($bounds:tt)+))?
//         {
//             $($impl:item)*
//         }

//     ) => {
//         $crate::impl_view!(
//             $e;
//             ($($generics)*)
//                 for $ontype :
//             <$life> <$e as $crate::env::Env>::RootMut<$life> => $mutfndest
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
//                 dyn for<$life,'ctx> Fn($mutfnroot,&$life (),&'ctx mut <E as $crate::env::Env>::Context<'_>) -> $crate::error::ResolveResult<$mutfndest> + Send + Sync + 'static
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
//     pub type BDyn = <'a> <E as Env>::RootMut<'a> => &'a mut test::B
// );
