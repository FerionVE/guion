use std::marker::PhantomData;

use crate::dispatchor::{CallbackClosure, AsWidgetDispatch};
use crate::env::Env;
use crate::error::ResolveResult;
use crate::root::RootRef;
use crate::widget::as_widget::AsWidget;

use super::View;

pub struct ViewWidget<'z,Wid,WFn,MFn,E>(WFn,MFn,PhantomData<(&'z Wid,E)>) where
    WFn: Fn()->Wid + 'z,
    Wid: View<'z,E>,
    MFn: for<'s,'c,'cc> Fn(E::RootMut<'s>,&'s (),&'c mut E::Context<'cc>) -> ResolveResult<Wid::Mutable<'s>> + Clone + 'static,
    E: Env;

pub fn view_widget_adv<'z,Wid,WFn,MFn,E>(w: WFn, f: MFn) -> ViewWidget<'z,Wid,WFn,MFn,E> where
    WFn: Fn()->Wid + 'z,
    Wid: View<'z,E>,
    MFn: for<'s,'c,'cc> Fn(E::RootMut<'s>,&'s (),&'c mut E::Context<'cc>) -> ResolveResult<Wid::Mutable<'s>> + Clone + 'static,
    E: Env,
{
    ViewWidget(w,f,PhantomData)
}
// pub fn view_widget_dummy_adv<'z,Wid,WFn,MFn,E>(w: WFn, f: MFn) -> DummyWidget<ViewWidget<'z,Wid,WFn,MFn,E>> where
//     WFn: Fn()->Wid + 'z,
//     Wid: View<'z,E>,
//     MFn: for<'s,'c,'cc> Fn(E::RootMut<'s>,&'s (),&'c mut E::Context<'cc>) -> ResolveResult<Wid::Mutable<'s>> + Clone + 'static,
//     E: Env,
// {
//     DummyWidget(ViewWidget(w,f,PhantomData))
// }

impl<'z,Wid,WFn,MFn,E> AsWidget<'z,E> for ViewWidget<'z,Wid,WFn,MFn,E> where
    WFn: Fn()->Wid + 'z,
    Wid: View<'z,E>,
    MFn: for<'s,'c,'cc> Fn(E::RootMut<'s>,&'s (),&'c mut E::Context<'cc>) -> ResolveResult<Wid::Mutable<'s>> + Clone + 'static,
    E: Env,
{
    type Widget<'v> = Wid::Viewed<'v,MFn> where 'z: 'v;

    #[inline]
    fn with_widget<'w,F>(&'w self, dispatch: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetDispatch<'z,Self,E>
    {
        let s = (self.0)();
        let dis = CallbackClosure::for_view(move |widget,root,ctx| {
            dispatch.call(widget, root, ctx)
        });
        s.view(dis,self.1.clone(),root,ctx)
    }
}

// pub struct DummyWidget<T>(pub T);

// impl<'z,T,E> Widget<E> for DummyWidget<T> where T: AsWidget<'z,E>, E: Env {
//     type Inner = ();

//     #[inline]
//     fn inner<'s>(&'s self) -> Option<&'s Self::Inner> where Self: 's {
//         None
//     }
//     #[inline]
//     fn run<S>(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
//         let dis = CallbackClosure::for_as_widget(|widget: &T::Widget<'_>,root,ctx| {
//             widget.run::<()>(root,ctx)
//         });
//         self.0.with_widget(dis, root, ctx)
//     }
// }

// impl_as_widget_self!(E;('z,T,E) 'z DummyWidget<T> where T: AsWidget<'z,E>);

//TODO impl AsWidget

#[macro_export]
macro_rules! view_widget {
    (
        $viewgen:expr,
        $mutor:ident $(($($extra_out:expr),*))?  =>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::view_widget!(
            $viewgen,
            $mutor $(($($extra_out),*))?  =>? |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
        )
    };
    (
        $viewgen:expr,
        $($mutor:ident)?                        |=>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::view_widget!(
            $viewgen,
            $($mutor)?                   |=>? |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
        )
    };
    (
        $viewgen:expr,
        $mutor:ident $(($($extra_out:expr),*))?  =>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::view_widget!(
            $viewgen,
            $mutor $(($($extra_out),*))? ?=>? |$root,$ctx $(,$($extra_in),*)?| {match $root {
                $crate::error::ResolveResult::Ok($root) => {$mutexpr},
                $crate::error::ResolveResult::Err(v) => $crate::error::ResolveResult::Err(v),
            }}
        )
    };
    (
        $viewgen:expr,
        $mutor:ident $(($($extra_out:expr),*))? ?=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::view_widget!(
            $viewgen,
            $mutor                       |=>? |$root,$ctx $(,$($extra_in),*)?| {
                let $root = ($mutor)($root,&(),$ctx $(,$($extra_out),*)? );
                $mutexpr
            }
        )
    };
    (
        $viewgen:expr,
        $($mutor:ident)?                        |=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            $crate::view::view_widget::view_widget_adv(
                $viewgen,
                #[inline] move |$root,_,$ctx $(,$($extra_in),*)?| {$mutexpr},
            )
        }
    };
}

/*#[macro_export]
macro_rules! view_widget { // calling view direct with mutor! works but this doesn't (closure lifetime error)
    (
        $viewgen:expr,
        $($mutor:tt)*
    ) => {
        $crate::view::view_widget::view_widget_adv(
            $viewgen,
            $crate::mutor!( $($mutor)* ),
        )
    };
}*/

#[macro_export]
macro_rules! mutor {
    (
        $mutor:ident $(($($extra_out:expr),*))?  =>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $mutor $(($($extra_out),*))?  =>? |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
        )
    };
    (
        $mutor:ident $(($($extra_out:expr),*))?  =>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $mutor $(($($extra_out),*))? ?=>? |$root,$ctx $(,$($extra_in),*)?| {match $root {
                $crate::error::ResolveResult::Ok($root) => {$mutexpr},
                $crate::error::ResolveResult::Err(v) => $crate::error::ResolveResult::Err(v),
            }}
        )
    };
    (
        $mutor:ident $(($($extra_out:expr),*))? ?=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $mutor                       |=>? |$root,$ctx $(,$($extra_in),*)?| {
                let $root = ($mutor)($root,&(),$ctx $(,$($extra_out),*)? );
                $mutexpr
            }
        )
    };
    (
        $($mutor:ident)?                        |=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            #[inline] move |$root,_,$ctx $(,$($extra_in),*)?| {$mutexpr}
        }
    };

    (
        $($mutor:ident)?                        |=>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $($mutor)?                   |=>? |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
        )
    };

    (
        $mutor:ident $(($($extra_out:expr),*))?  =>| |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $mutor $(($($extra_out),*))? ?=>| |$root,$ctx $(,$($extra_in),*)?| {match $root {
                $crate::error::ResolveResult::Ok($root) => {$mutexpr;},
                $crate::error::ResolveResult::Err(_) => {/*TODO*/},
            }}
        )
    };
    (
        $mutor:ident $(($($extra_out:expr),*))? ?=>| |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor!(
            $mutor                       |=>| |$root,$ctx $(,$($extra_in),*)?| {
                let $root = ($mutor)($root,&(),$ctx $(,$($extra_out),*)? );
                $mutexpr;
            }
        )
    };
    (
        $($mutor:ident)?                        |=>| |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            #[inline] move |$root,_,$ctx $(,$($extra_in),*)?| {$mutexpr;}
        }
    };
}
