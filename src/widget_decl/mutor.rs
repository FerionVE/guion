use std::sync::Arc;

use crate::env::Env;

/// `dyn MutorFn` definiton.
/// 
/// # Example
/// 
/// ```ignore
/// fn my_view(
///     mutation_lens_builder: &(dyn Fn() -> Box<mutordyn!(Env; MyModel)> + Send + Sync),
/// ) {
///     ...
/// }
/// ```
#[macro_export]
macro_rules! mutordyn {
    ($e:ty; for<$($fl:lifetime),+ $(,)*> $t:ty) => {
        dyn for<'__m_lf_a,'__m_lf_b,'__m_lf_c> ::std::ops::Fn(
            &'__m_lf_b mut <$e as $crate::env::Env>::Context<'__m_lf_c>,
            $crate::mutorcb!($e; for<$($fl),+> $t)
        ) + ::std::marker::Send + ::std::marker::Sync + 'static
    };
    ($e:ty; $t:ty) => {
        dyn for<'__m_lf_a,'__m_lf_b,'__m_lf_c> ::std::ops::Fn(
            &'__m_lf_b mut <$e as $crate::env::Env>::Context<'__m_lf_c>,
            $crate::mutorcb!($e; $t)
        ) + ::std::marker::Send + ::std::marker::Sync + 'static
    };
}

/// `dyn MutorEndFn`definition
#[macro_export]
macro_rules! mutorenddyn {
    ($e:ty; $($t:ty),* $(,)*) => {
        dyn for<'__m_lf_a,'__m_lf_b> ::std::ops::Fn(
            &'__m_lf_a mut <$e as $crate::env::Env>::Context<'__m_lf_b>,
            $($t:ty),*
        ) + ::std::marker::Send + ::std::marker::Sync + 'static
    };
}

/// Force value into mutor type
/// 
/// Wrap the mutor closure (not the Box/Arc::new or the mutor builder) into this if won't do it
/// (and defining the return type on the builder closure isn't possible)
/// 
/// Try `mutor_force_ty_tait` if this doesn't work
#[macro_export]
macro_rules! mutor_force_ty {
    ($e:ty; for<$($fl:lifetime),+ $(,)*> $t:ty) => {
        #[inline(always)]
        fn __mutor_forcer<V>(v: V) -> V where V:
            for<'__m_lf_a,'__m_lf_b,'__m_lf_c> ::std::ops::Fn(
                &'__m_lf_b mut <$e as $crate::env::Env>::Context<'__m_lf_c>,
                $crate::mutorcb!($e; for<$($fl),+> $t)
            ) + ::std::marker::Send + ::std::marker::Sync + 'static
        {
            v
        }
        __mutor_forcer($v)
    };
    ($e:ty; $t:ty; $v:expr) => {{
        #[inline(always)]
        fn __mutor_forcer<V>(v: V) -> V where V:
            for<'__m_lf_a,'__m_lf_b,'__m_lf_c> ::std::ops::Fn(
                &'__m_lf_b mut <$e as $crate::env::Env>::Context<'__m_lf_c>,
                $crate::mutorcb!($e; $t)
            ) + ::std::marker::Send + ::std::marker::Sync + 'static
        {
            v
        }
        __mutor_forcer($v)
    }};
}

/// Force value into mutor type (requires TAIT feature)
#[macro_export]
macro_rules! mutor_force_ty_tait {
    ($e:ty; for<$($fl:lifetime),+ $(,)*> $t:ty) => {
        type __mutor_forcer_tait = impl
            for<'__m_lf_a,'__m_lf_b,'__m_lf_c> ::std::ops::Fn(
                &'__m_lf_b mut <$e as $crate::env::Env>::Context<'__m_lf_c>,
                $crate::mutorcb!($e; for<$($fl),+> $t)
            ) + ::std::marker::Send + ::std::marker::Sync + 'static
        ;
        let __mutor_forcing: __mutor_forcer_tait = $v;
        __mutor_forcing
    };
    ($e:ty; $t:ty; $v:expr) => {{
        type __mutor_forcer_tait = impl
            for<'__m_lf_a,'__m_lf_b,'__m_lf_c> ::std::ops::Fn(
                &'__m_lf_b mut <$e as $crate::env::Env>::Context<'__m_lf_c>,
                $crate::mutorcb!($e; $t)
            ) + ::std::marker::Send + ::std::marker::Sync + 'static
        ;
        let __mutor_forcing: __mutor_forcer_tait = $v;
        __mutor_forcing
    }};
}

#[macro_export]
macro_rules! mutorend_force_ty {
    ($e:ty; $($t:ty),* $(,)*; $v:expr) => {{
        #[inline(always)]
        fn __mutor_forcer<V>(v: V) -> V where V:
            for<'__m_lf_a,'__m_lf_b> ::std::ops::Fn(
                &'__m_lf_a mut <$e as $crate::env::Env>::Context<'__m_lf_b>,
                $($t:ty),*
            ) + ::std::marker::Send + ::std::marker::Sync + 'static
        {
            v
        }
        __mutor_forcer($v)
    }};
}

#[macro_export]
macro_rules! mutorend_force_ty_tait {
    ($e:ty; $($t:ty),* $(,)*; $v:expr) => {{
        type __mutor_forcer_tait = impl
            for<'__m_lf_a,'__m_lf_b> ::std::ops::Fn(
                &'__m_lf_a mut <$e as $crate::env::Env>::Context<'__m_lf_b>,
                $($t:ty),*
            ) + ::std::marker::Send + ::std::marker::Sync + 'static
        ;
        let __mutor_forcing: __mutor_forcer_tait = $v;
        __mutor_forcing
    }};
}

#[macro_export]
macro_rules! mutorimpl {
    ($e:ty; for<$($fl:lifetime),+ $(,)*> $t:ty) => {
        impl for<'__m_lf_a,'__m_lf_b,'__m_lf_c> ::std::ops::Fn(
            &'__m_lf_b mut <$e as $crate::env::Env>::Context<'__m_lf_c>,
            $crate::mutorcb!($e; for<$($fl),+> $t)
        ) + ::std::marker::Send + ::std::marker::Sync + 'static
    };
    ($e:ty; $t:ty) => {
        impl for<'__m_lf_a,'__m_lf_b,'__m_lf_c> ::std::ops::Fn(
            &'__m_lf_b mut <$e as $crate::env::Env>::Context<'__m_lf_c>,
            $crate::mutorcb!($e; $t)
        ) + ::std::marker::Send + ::std::marker::Sync + 'static
    };
}

#[macro_export]
macro_rules! mutor_ty {
    ($e:ty; for<$($fl:lifetime),+ $(,)*> $t:ty) => {
        for<'__m_lf_a,'__m_lf_b,'__m_lf_c> ::std::ops::Fn(
            &'__m_lf_b mut <$e as $crate::env::Env>::Context<'__m_lf_c>,
            $crate::mutorcb!($e; for<$($fl),+> $t)
        ) + ::std::marker::Send + ::std::marker::Sync + 'static
    };
    ($e:ty; $t:ty) => {
        for<'__m_lf_a,'__m_lf_b,'__m_lf_c> ::std::ops::Fn(
            &'__m_lf_b mut <$e as $crate::env::Env>::Context<'__m_lf_c>,
            $crate::mutorcb!($e; $t)
        ) + ::std::marker::Send + ::std::marker::Sync + 'static
    };
}

/// Mutor callback type
#[macro_export]
macro_rules! mutorcb {
    ($e:ty; for<$($fl:lifetime),+ $(,)*> $t:ty) => {
        &'_ mut (dyn for<'__m_lf_d,'__m_lf_e,'__m_lf_f,$($fl),+,> ::std::ops::FnMut(
            &'__m_lf_d mut <$e as $crate::env::Env>::Context<'__m_lf_e>,
            &'__m_lf_f mut $t,
        ))
    };
    ($e:ty; $t:ty) => {
        &'_ mut (dyn for<'__m_lf_d,'__m_lf_e,'__m_lf_f> ::std::ops::FnMut(
            &'__m_lf_d mut <$e as $crate::env::Env>::Context<'__m_lf_e>,
            &'__m_lf_f mut $t,
        ))
    };
}

#[macro_export]
macro_rules! end_mutor_builder_arc {
    ($e:ty; $mutr:ident; $mutfn:expr) => {
        || -> ::std::sync::Arc<mutorenddyn!(ExampleEnv;)> {
            let $mutr = ($mutr)();
            ::std::sync::Arc::new(move |__wmb_ctx| {
                $mutr(__wmb_ctx, &mut $mutfn)
            })
        };
    };
}

pub fn mutorbox_static<M,F,E>(f: F) -> Box<mutordyn!(E; M)> where E: Env, M: 'static, F: 
    for<'a,'b,'c> Fn(
        &'b mut E::Context<'c>,
        &'a mut (dyn for<'d,'e,'f> FnMut(
            &'d mut E::Context<'e>,
            &'f mut M,
        )),
    ) + Send + Sync + 'static
{
    Box::new(f)
}

pub fn mutorarc_static<M,F,E>(f: F) -> Arc<mutordyn!(E; M)> where E: Env, M: 'static, F: 
    for<'a,'b,'c> Fn(
        &'b mut E::Context<'c>,
        &'a mut (dyn for<'d,'e,'f> FnMut(
            &'d mut E::Context<'e>,
            &'f mut M,
        )),
    ) + Send + Sync + 'static
{
    Arc::new(f)
}

// #[macro_export]
// macro_rules! mutordong {
//     ($v:expr) => {
//         $v
//     };
// }
