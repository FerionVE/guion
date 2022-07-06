use std::marker::PhantomData;

use crate::dispatchor::{AsWidgetDispatch, ViewClosure, AsWidgetClosure};
use crate::env::Env;
use crate::error::ResolveResult;
use crate::root::RootRef;
use crate::widget::Widget;
use crate::widget::as_widget::AsWidget;

use super::View;

pub struct ViewWidget<'z,Wid,WFn,MFn,E>(WFn,MFn,PhantomData<(&'z Wid,E)>) where
    WFn: Fn()->Wid + 'z,
    Wid: View<'z,E>,
    MFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,&'s (),
        &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Wid::Mutable<'iss>>,&'iss (),&'c mut E::Context<'cc>)),
        &'c mut E::Context<'cc>
    ) + Clone + 'static,
    E: Env;

pub fn view_widget_adv<'z,Wid,WFn,MFn,E>(w: WFn, f: MFn) -> ViewWidget<'z,Wid,WFn,MFn,E> where
    WFn: Fn()->Wid + 'z,
    Wid: View<'z,E>,
    MFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,&'s (),
        &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Wid::Mutable<'iss>>,&'iss (),&'c mut E::Context<'cc>)),
        &'c mut E::Context<'cc>
    ) + Clone + 'static,
    E: Env,
{
    ViewWidget(w,f,PhantomData)
}
// pub fn view_widget_dummy_adv<'z,Wid,WFn,MFn,E>(w: WFn, f: MFn) -> DummyWidget<ViewWidget<'z,Wid,WFn,MFn,E>> where
//     WFn: Fn()->Wid + 'z,
//     Wid: View<'z,E>,
//     MFn: for<'s,'c,'cc> Fn(
//         E::RootMut<'s>,&'s (),
//         &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Wid::Mutable<'iss>>,&'iss (),&'c mut E::Context<'cc>)),
//         &'c mut E::Context<'cc>
//     ) + Clone + 'static,
//     E: Env,
// {
//     DummyWidget(ViewWidget(w,f,PhantomData))
// }

impl<'z,Wid,WFn,MFn,E> AsWidget<'z,E> for ViewWidget<'z,Wid,WFn,MFn,E> where
    WFn: Fn()->Wid + 'z,
    Wid: View<'z,E>,
    MFn: for<'s,'c,'cc> Fn(
        E::RootMut<'s>,&'s (),
        &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Wid::Mutable<'iss>>,&'iss (),&'c mut E::Context<'cc>)),
        &'c mut E::Context<'cc>
    ) + Clone + 'static,
    E: Env,
{
    type Widget<'v> = Wid::Viewed<'v,MFn> where 'z: 'v;

    #[inline]
    fn with_widget<'w,F>(&'w self, dispatch: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetDispatch<'z,Self,E>
    {
        let s = (self.0)();
        let dis = ViewClosure::new(move |widget,root,ctx| {
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
//         let dis = AsWidgetClosure::new(|widget: &T::Widget<'_>,root,ctx| {
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
        $mutor:ident $(($($extra_out:expr),*))?  =>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;$($derefor:tt)*
    ) => {
        $crate::view_widget!(
            $viewgen,
            $mutor $(($($extra_out),*))?  =>?  |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
            ;$($derefor)*
        )
    };
    (
        $viewgen:expr,
        $($mutor:ident)?                        |=>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;$($derefor:tt)*
    ) => {
        $crate::view_widget!(
            $viewgen,
            $($mutor)?                   |=>? (($derefor)*) |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
            ;$($derefor)*
        )
    };

    (
        $viewgen:expr,
        $mutor:ident $(($($extra_out:expr),*))?  =>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;$($derefor:tt)*
    ) => {
        $crate::view_widget!(
            $viewgen,
            $mutor $(($($extra_out),*))? ?=>? |$root,$ctx $(,$($extra_in),*)?| {match $root {
                $crate::error::ResolveResult::Ok($root) => {$mutexpr},
                $crate::error::ResolveResult::Err(v) => $crate::error::ResolveResult::Err(v),
            }}
            ;$($derefor)*
        )
    };

    (
        $viewgen:expr,
        $mutor:ident $(($($extra_out:expr),*))? ?=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;$($derefor:tt)*
    ) => {
        {
            let $mutor = $mutor.clone();
            $crate::view::view_widget::view_widget_adv(
                $viewgen,
                #[inline] move |$root,_,__callback,$ctx $(,$($extra_in),*)?| {
                    ($mutor)(
                        $root,&(),
                        &mut move |$root,_,$ctx| {
                            let __val = $mutexpr;
                            match __val {
                                ::std::result::Result::Ok(mut __val) =>
                                    (__callback)(::std::result::Result::Ok($($derefor)* __val),&(),$ctx),
                                ::std::result::Result::Err(e) =>
                                    (__callback)(::std::result::Result::Err(e),&(),$ctx),
                            };
                        },
                        $ctx,
                    )
                }
            )
        }
    };
    (
        $viewgen:expr,
        $($mutor:ident)?                        |=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;$($derefor:tt)*
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            $crate::view::view_widget::view_widget_adv(
                $viewgen,
                #[inline] move |$root,_,$callback,$ctx $(,$($extra_in),*)?| {
                    let __val = $mutexpr;
                    match __val {
                        ::std::result::Result::Ok(mut __val) =>
                            (__callback)(::std::result::Result::Ok($($derefor)* __val),&(),$ctx),
                        ::std::result::Result::Err(e) =>
                            (__callback)(::std::result::Result::Err(e),&(),$ctx),
                    };
                },
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
        $mutor:ident $(($($extra_out:expr),*))?  =>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;$($derefor:tt)*
    ) => {
        $crate::mutor!(
            $mutor $(($($extra_out),*))?  =>? |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
            ;$($derefor)*
        )
    };
    (
        $($mutor:ident)?                        |=>  |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;$($derefor:tt)*
    ) => {
        $crate::mutor!(
            $($mutor)?                   |=>? |$root,$ctx $(,$($extra_in),*)?| $crate::error::ResolveResult::Ok( $mutexpr )
            ;$($derefor)*
        )
    };
    (
        $mutor:ident $(($($extra_out:expr),*))?  =>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;$($derefor:tt)*
    ) => {
        $crate::mutor!(
            $mutor $(($($extra_out),*))? ?=>? |$root,$ctx $(,$($extra_in),*)?| {match $root {
                $crate::error::ResolveResult::Ok($root) => {$mutexpr},
                $crate::error::ResolveResult::Err(v) => $crate::error::ResolveResult::Err(v),
            }}
            ;$($derefor)*
        )
    };
    (
        $mutor:ident $(($($extra_out:expr),*))? ?=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;$($derefor:tt)*
    ) => {
        {
            let $mutor = $mutor.clone();
            #[inline] move |$root,_,__callback,$ctx $(,$($extra_in),*)?| {
                ($mutor)(
                    $root,&(),
                    &mut move |$root,_,$ctx| {
                        let __val = $mutexpr;
                        match __val {
                            ::std::result::Result::Ok(mut __val) =>
                                (__callback)(::std::result::Result::Ok($($derefor)* __val),&(),$ctx),
                            ::std::result::Result::Err(e) =>
                                (__callback)(::std::result::Result::Err(e),&(),$ctx),
                        };
                    },
                    $ctx,
                )
            }
        }
    };
    (
        $($mutor:ident)?                        |=>? |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;$($derefor:tt)*
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            #[inline] move |$root,_,$callback,$ctx $(,$($extra_in),*)?| {
                let __val = $mutexpr;
                match __val {
                    ::std::result::Result::Ok(mut __val) =>
                        (__callback)(::std::result::Result::Ok($($derefor)* __val),&(),$ctx),
                    ::std::result::Result::Err(e) =>
                        (__callback)(::std::result::Result::Err(e),&(),$ctx),
                };
            },
        }
    };

    (
        $mutor:ident $(($($extra_out:expr),*))?  =>| |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;
    ) => {
        $crate::mutor!(
            $mutor $(($($extra_out),*))? ?=>| |$root,$ctx $(,$($extra_in),*)?| {match $root {
                $crate::error::ResolveResult::Ok($root) => {$mutexpr;},
                $crate::error::ResolveResult::Err(_) => {/*TODO*/},
            }};
        )
    };
    (
        $mutor:ident $(($($extra_out:expr),*))? ?=>| |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;
    ) => {
        $crate::mutor!(
            $mutor                       |=>| |__root,$ctx $(,$($extra_in),*)?| {
                ($mutor)(
                    __root,&(),
                    &mut move |$root,_,$ctx| {
                        $mutexpr
                    },
                    $ctx,
                )
            };
        )
    };
    (
        $($mutor:ident)?                        |=>| |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr ;
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            #[inline] move |$root,_,$ctx $(,$($extra_in),*)?| {$mutexpr;}
        }
    };
}
