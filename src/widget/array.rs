//! Trait over types holding an array of AsWidget types
use super::*;

pub trait WidgetArray<E>: Sized + Statize<E> where E: Env {
    fn len(&self) -> usize;
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()>;
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w;
    fn childs<'s>(&'s self) -> Vec<Resolvable<'s,E>>;
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w;
}
pub trait WidgetArrayMut<E>: WidgetArray<E> where E: Env {
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,E>,()>;
    fn into_child_mut<'w>(self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w;
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>>;
    fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w;
}

impl<T,E> WidgetArray<E> for Vec<T> where T: AsWidget<E>+StatizeSized<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<'_,E>,()> {
        Ok(self.get(i).ok_or(())?.as_ref())
    }
    #[inline]
    fn into_child<'w>(mut self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        if self.len() > i {
            Ok(self.swap_remove(i).into_ref())
        }else{
            Err(())
        }
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<'_,E>> {
        self.iter()
            .map(#[inline] |w| w.as_ref() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w| w.into_ref() )
            .collect::<Vec<_>>()
    }
}
impl<T,E> WidgetArrayMut<E> for Vec<T> where T: AsWidgetMut<E>+StatizeSized<E>, E: Env {
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<'_,E>,()> {
        Ok(self.get_mut(i).ok_or(())?.as_mut())
    }
    #[inline]
    fn into_child_mut<'w>(mut self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        if self.len() > i {
            Ok(self.swap_remove(i).into_mut())
        }else{
            Err(())
        }
    }
    #[inline]
    fn childs_mut(&mut self) -> Vec<ResolvableMut<'_,E>> {
        self.iter_mut()
            .map(#[inline] |w| w.as_mut() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w| w.into_mut() )
            .collect::<Vec<_>>()
    }
}

impl<T,E> WidgetArray<E> for &[T] where T: AsWidget<E>+StatizeSized<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<'_,E>,()> {
        Ok(self.get(i).ok_or(())?.as_ref())
    }
    #[inline]
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        Ok(self.get(i).ok_or(())?.as_ref())
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<'_,E>> {
        self.iter()
            .map(#[inline] |w| w.as_ref() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w: &'w T| w.as_ref() )
            .collect::<Vec<_>>()
    }
}

impl<T,E> WidgetArray<E> for &mut [T] where T: AsWidget<E>+StatizeSized<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        Ok(self.get(i).ok_or(())?.as_ref())
    }
    #[inline]
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        Ok(self.get(i).ok_or(())?.as_ref())
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<E>> {
        self.iter()
            .map(#[inline] |w| w.as_ref() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w: &'w mut T| w.as_ref() )
            .collect::<Vec<_>>()
    }
}
impl<T,E> WidgetArrayMut<E> for &mut [T] where T: AsWidgetMut<E>+StatizeSized<E>, E: Env {
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<'_,E>,()> {
        Ok(self.get_mut(i).ok_or(())?.as_mut())
    }
    #[inline]
    fn into_child_mut<'w>(self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        Ok(self.get_mut(i).ok_or(())?.as_mut())
    }
    #[inline]
    fn childs_mut(&mut self) -> Vec<ResolvableMut<'_,E>> {
        self.iter_mut()
            .map(#[inline] |w| w.as_mut() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w: &'w mut T| w.as_mut() )
            .collect::<Vec<_>>()
    }
}

macro_rules! impl_wpps_tuple {
    {$n:expr;$t:ident $($tt:ident)+;$l:ident $($ll:ident)+} => {
        impl_wpps_tuple!(($n-1);$($tt)+;$($ll)+);

        impl<E,$t,$($tt),+> WidgetArray<E> for ($t,$($tt),+) where
            E: Env,
            $t: AsWidget<E>+StatizeSized<E>,
            $($tt: AsWidget<E>+StatizeSized<E>),+ 
        {
            #[inline]
            fn len(&self) -> usize {
                $n
            }
            #[inline]
            fn child(&self, i: usize) -> Result<Resolvable<'_,E>,()> {
                if self.len() > i { //TODO optimize (current method completely defeats the purpose of tuples)
                    Ok(self.childs().swap_remove(i))
                }else{
                    Err(())
                }
            }
            #[inline]
            fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
                if self.len() > i { //TODO optimize (current method completely defeats the purpose of tuples)
                    Ok(self.into_childs().swap_remove(i))
                }else{
                    Err(())
                }
            }
            #[inline]
            fn childs(&self) -> Vec<Resolvable<'_,E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_ref(), $( $ll .as_ref() ),* ]
            }
            #[inline]
            fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.into_ref(), $( $ll .into_ref() ),* ]
            }
        }

        impl<E,$t,$($tt),+> WidgetArrayMut<E> for ($t,$($tt),+) where
            E: Env,
            $t: AsWidgetMut<E>+StatizeSized<E>,
            $($tt: AsWidgetMut<E>+StatizeSized<E>),+ 
        {
            #[inline]
            fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<'_,E>,()> {
                if self.len() > i { //TODO optimize (current method completely defeats the purpose of tuples)
                    Ok(self.childs_mut().swap_remove(i))
                }else{
                    Err(())
                }
            }
            #[inline]
            fn into_child_mut<'w>(self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
                if self.len() > i { //TODO optimize (current method completely defeats the purpose of tuples)
                    Ok(self.into_childs_mut().swap_remove(i))
                }else{
                    Err(())
                }
            }
            #[inline]
            fn childs_mut(&mut self) -> Vec<ResolvableMut<'_,E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_mut(), $( $ll .as_mut() ),* ]
            }
            #[inline]
            fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.into_mut(), $( $ll .into_mut() ),* ]
            }
        }
    };
    {$n:expr;$t:ident;$l:ident} => {}
}

impl_wpps_tuple!(
    32;
    A B C D F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE AF AG;
    a b c d f g h i j k l m n o p q r s t u v w x y z aa ab ac ad ae af ag
);
