#[macro_export]
macro_rules! compat_for_crossbeam_utils_0_8 {
    ($e:ty) => {
        impl<T> $crate::widgets::util::state::AtomState<$e,T> for ::crossbeam_utils::sync::ShardedLockReadGuard<'_,T> where T: Clone, $e: $crate::env::Env {
            #[inline]
            fn get_direct(&self) -> Result<T,()> {
                Ok((**self).clone())
            }
        }
        impl<T> $crate::widgets::util::state::AtomState<$e,T> for & ::crossbeam_utils::sync::ShardedLockReadGuard<'_,T> where T: Clone, $e: $crate::env::Env {
            #[inline]
            fn get_direct(&self) -> Result<T,()> {
                Ok((***self).clone())
            }
        }
        
        impl<T> $crate::widgets::util::state::AtomState<$e,T> for ::crossbeam_utils::sync::ShardedLockWriteGuard<'_,T> where T: Clone, $e: $crate::env::Env {
            #[inline]
            fn get_direct(&self) -> Result<T,()> {
                Ok((**self).clone())
            }
        }
        impl<T> $crate::widgets::util::state::AtomStateMut<$e,T> for ::crossbeam_utils::sync::ShardedLockWriteGuard<'_,T> where T: Clone, $e: $crate::env::Env {
            #[inline]
            fn set_direct(&mut self, v: T) -> Result<(),()> {
                **self = v;
                Ok(())
            }
        }
        
        impl<T> $crate::widgets::util::state::AtomState<$e,T> for & ::crossbeam_utils::sync::ShardedLockWriteGuard<'_,T> where T: Clone, $e: $crate::env::Env {
            #[inline]
            fn get_direct(&self) -> Result<T,()> {
                Ok((***self).clone())
            }
        }
        impl<T> $crate::widgets::util::state::AtomState<$e,T> for &mut ::crossbeam_utils::sync::ShardedLockWriteGuard<'_,T> where T: Clone, $e: $crate::env::Env {
            #[inline]
            fn get_direct(&self) -> Result<T,()> {
                Ok((***self).clone())
            }
        }
        impl<T> $crate::widgets::util::state::AtomStateMut<$e,T> for &mut ::crossbeam_utils::sync::ShardedLockWriteGuard<'_,T> where T: Clone, $e: $crate::env::Env {
            #[inline]
            fn set_direct(&mut self, v: T) -> Result<(),()> {
                ***self = v;
                Ok(())
            }
        }

        impl<A> $crate::text::stor::TextStor<$e> for ::crossbeam_utils::sync::ShardedLockReadGuard<'_,A> where A: $crate::text::stor::TextStor<$e> + ?Sized {
            #[inline]
            fn caption<'s>(&'s self) -> ::std::borrow::Cow<'s,str> {
                (**self).caption()
            }
            #[inline]
            fn chars(&self) -> usize {
                (**self).chars()
            }
            #[inline]
            fn len(&self) -> usize {
                (**self).len()
            }
        }
        
        impl<A> $crate::text::stor::TextStor<$e> for ::crossbeam_utils::sync::ShardedLockWriteGuard<'_,A> where A: $crate::text::stor::TextStor<$e> + ?Sized {
            #[inline]
            fn caption<'s>(&'s self) -> ::std::borrow::Cow<'s,str> {
                (**self).caption()
            }
            #[inline]
            fn chars(&self) -> usize {
                (**self).chars()
            }
            #[inline]
            fn len(&self) -> usize {
                (**self).len()
            }
        }
        impl<A> $crate::text::stor::TextStorMut<$e> for ::crossbeam_utils::sync::ShardedLockWriteGuard<'_,A> where A: $crate::text::stor::TextStorMut<$e> + ?Sized {
            #[inline]
            fn replace(&mut self, replace_range: ::std::ops::Range<usize>, insert: &str){
                (**self).replace(replace_range,insert)
            }
        }
    }
}
