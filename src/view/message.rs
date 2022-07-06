use crate::env::Env;

/// Implement on [`View::Mutable`]
pub trait Messagable<E> where E: Env {
    fn message(&mut self, m: &dyn std::any::Any, ctx: &mut E::Context<'_>);
}

impl<E,T> Messagable<E> for &mut T where T: Messagable<E> + ?Sized, E: Env {
    #[inline]
    fn message(&mut self, m: &dyn std::any::Any, ctx: &mut E::Context<'_>) {
        (**self).message(m, ctx)
    }
}

#[macro_export]
macro_rules! messaged {
    (
        $e:ty;$mutor:ident $(($($extra_out:expr),*))? |$root:ident,$ctx:ident|| $id:expr
    ) => {
        {
            let $mutor = $mutor.clone();
            let __id = ($id);
            #[inline] move |$root, _: &'_ (), $ctx: &'_ mut _| {
                ($mutor)(
                    $root,&(),
                    &mut move |__resolved,_,$ctx| {
                        let mut __resolved = __resolved.expect("TODO");
                        let __msg = $id;
                        $crate::view::applion::Messagable::<$e>::message(__resolved,&__msg as &dyn ::std::any::Any,$ctx)
                    },
                    $ctx $(,$($extra_out),*)?
                )
            }
        }
    };
    (
        $e:ty;$mutor:ident $(($($extra_out:expr),*))? |$root:ident|| $id:expr
    ) => {
        {
            let $mutor = $mutor.clone();
            let __id = ($id);
            #[inline] move |$root, _: &'_ (), __ctx: &'_ mut _ | {
                ($mutor)(
                    $root,&(),
                    &mut move |__resolved,_,$ctx| {
                        let mut __resolved = __resolved.expect("TODO");
                        let __msg = $id;
                        $crate::view::applion::Messagable::<$e>::message(__resolved,&__msg as &dyn ::std::any::Any,__ctx)
                    },
                    __ctx $(,$($extra_out),*)?
                )
            }
        }
    };
    (
        $e:ty;$mutor:ident $(($($extra_out:expr),*))? |$root:ident,$ctx:ident|$($extra_in:ident $(: $extra_in_ty:ty)?),*| $id:expr
    ) => {
        {
            let $mutor = $mutor.clone();
            let __id = ($id);
            #[inline] move |$root, _: &'_ (), $ctx: &'_ mut _ ,$($extra_in $(: $extra_in_ty)?),*| {
                ($mutor)(
                    $root,&(),
                    &mut move |__resolved,_,$ctx| {
                        let mut __resolved = __resolved.expect("TODO");
                        let __msg = $id;
                        $crate::view::applion::Messagable::<$e>::message(__resolved,&__msg as &dyn ::std::any::Any,$ctx)
                    },
                    $ctx $(,$($extra_out),*)?
                )
            }
        }
    };
    (
        $e:ty;$mutor:ident $(($($extra_out:expr),*))? |$root:ident|$($extra_in:ident $(: $extra_in_ty:ty)?),*| $id:expr
    ) => {
        {
            let $mutor = $mutor.clone();
            let __id = ($id);
            #[inline] move |$root, _: &'_ (), __ctx: &'_ mut _ ,$($extra_in $(: $extra_in_ty)?),*| {
                ($mutor)(
                    $root,&(),
                    &mut move |__resolved,_,$ctx| {
                        let mut __resolved =__resolved.expect("TODO");
                        let __msg = $id;
                        $crate::view::applion::Messagable::<$e>::message(__resolved,&__msg as &dyn ::std::any::Any,__ctx)
                    },
                    __ctx $(,$($extra_out),*)?
                )
            }
        }
    };
    (
        $e:ty;$mutor:ident $(($($extra_out:expr),*))? |$($extra_in:ident $(: $extra_in_ty:ty)?),*| $id:expr
    ) => {
        {
            let $mutor = $mutor.clone();
            let __id = ($id);
            #[inline] move |__root, _: &'_ (), __ctx: &'_ mut _ ,$($extra_in $(: $extra_in_ty)?),*| {
                ($mutor)(
                    $root,&(),
                    &mut move |__resolved,_,$ctx| {
                        let mut __resolved = __resolved.expect("TODO");
                        let __msg = $id;
                        $crate::view::applion::Messagable::<$e>::message(__resolved,&__msg as &dyn ::std::any::Any,__ctx)
                    },
                    __ctx $(,$($extra_out),*)?
                )
            }
        }
    };
    (
        $e:ty;$mutor:ident $(($($extra_out:expr),*))? || $id:expr
    ) => {
        {
            let $mutor = $mutor.clone();
            let __id = ($id);
            #[inline] move |__root, _: &'_ (), __ctx: &'_ mut _| {
                ($mutor)(
                    $root,&(),
                    &mut move |__resolved,_,$ctx| {
                        let mut __resolved = __resolved.expect("TODO");
                        let __msg = $id;
                        $crate::view::applion::Messagable::<$e>::message(__resolved,&__msg as &dyn ::std::any::Any,__ctx)
                    },
                    __ctx $(,$($extra_out),*)?
                )
            }
        }
    };
}

pub(crate) use messaged;
