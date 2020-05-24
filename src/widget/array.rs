//! Trait over types holding an array of AsWidget types
use super::*;

pub trait WidgetArray<'w,E>: Sized + Statize<E> where E: Env {
    fn len(&self) -> usize;
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()> where 'w: 's;
    fn into_child(self, i: usize) -> Result<Resolvable<'w,E>,()>;
    fn childs<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's;
    fn into_childs(self) -> Vec<Resolvable<'w,E>>;
}
pub trait WidgetArrayMut<'w,E>: WidgetArray<'w,E> where E: Env {
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,E>,()> where 'w: 's;
    fn into_child_mut(self, i: usize) -> Result<ResolvableMut<'w,E>,()>;
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's;
    fn into_childs_mut(self) -> Vec<ResolvableMut<'w,E>>;
}

impl<'w,T,E> WidgetArray<'w,E> for Vec<T> where T: AsWidget<'w,E>+Statize<E>, T::Statur: Sized, E: Env {
    fn len(&self) -> usize {
        self.len()
    }
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()> where 'w: 's {
        Ok(self.get(i).ok_or(())?.as_ref())
    }
    fn into_child(mut self, i: usize) -> Result<Resolvable<'w,E>,()> {
        if self.len() > i {
            Ok(self.swap_remove(i).into_ref())
        }else{
            Err(())
        }
    }
    fn childs<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        self.iter()
            .map(|w| w.as_ref() )
            .collect::<Vec<_>>()
    }
    fn into_childs(self) -> Vec<Resolvable<'w,E>> {
        self.into_iter()
            .map(|w| w.into_ref() )
            .collect::<Vec<_>>()
    }
}
impl<'w,T,E> WidgetArrayMut<'w,E> for Vec<T> where T: AsWidgetMut<'w,E>+Statize<E>, T::Statur: Sized, E: Env {
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,E>,()> where 'w: 's {
        Ok(self.get_mut(i).ok_or(())?.as_mut())
    }
    fn into_child_mut(mut self, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        if self.len() > i {
            Ok(self.swap_remove(i).into_mut())
        }else{
            Err(())
        }
    }
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        self.iter_mut()
            .map(|w| w.as_mut() )
            .collect::<Vec<_>>()
    }
    fn into_childs_mut(self) -> Vec<ResolvableMut<'w,E>> {
        self.into_iter()
            .map(|w| w.into_mut() )
            .collect::<Vec<_>>()
    }
}

impl<'w,'l,T,E> WidgetArray<'w,E> for &'w [T] where T: AsWidget<'l,E>+Statize<E>, T::Statur: Sized, E: Env, 'l: 'w {
    fn len(&self) -> usize {
        (**self).len()
    }
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()> where 'w: 's {
        Ok(self.get(i).ok_or(())?.as_ref())
    }
    fn into_child(self, i: usize) -> Result<Resolvable<'w,E>,()> {
        Ok(self.get(i).ok_or(())?.as_ref())
    }
    fn childs<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        self.iter()
            .map(|w| w.as_ref() )
            .collect::<Vec<_>>()
    }
    fn into_childs(self) -> Vec<Resolvable<'w,E>> {
        self.into_iter()
            .map(|w: &'w T| w.as_ref() )
            .collect::<Vec<_>>()
    }
}

impl<'w,'l,T,E> WidgetArray<'w,E> for &'w mut [T] where T: AsWidget<'l,E>+Statize<E>, T::Statur: Sized, E: Env, 'l: 'w {
    fn len(&self) -> usize {
        (**self).len()
    }
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()> where 'w: 's {
        Ok(self.get(i).ok_or(())?.as_ref())
    }
    fn into_child(self, i: usize) -> Result<Resolvable<'w,E>,()> {
        Ok(self.get(i).ok_or(())?.as_ref())
    }
    fn childs<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        self.iter()
            .map(|w| w.as_ref() )
            .collect::<Vec<_>>()
    }
    fn into_childs(self) -> Vec<Resolvable<'w,E>> {
        self.into_iter()
            .map(|w: &'w mut T| w.as_ref() )
            .collect::<Vec<_>>()
    }
}
impl<'w,'l,T,E> WidgetArrayMut<'w,E> for &'w mut [T] where T: AsWidgetMut<'l,E>+Statize<E>, T::Statur: Sized, E: Env, 'l: 'w {
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,E>,()> where 'w: 's {
        Ok(self.get_mut(i).ok_or(())?.as_mut())
    }
    fn into_child_mut(self, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        Ok(self.get_mut(i).ok_or(())?.as_mut())
    }
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        self.iter_mut()
            .map(|w| w.as_mut() )
            .collect::<Vec<_>>()
    }
    fn into_childs_mut(self) -> Vec<ResolvableMut<'w,E>> {
        self.into_iter()
            .map(|w: &'w mut T| w.as_mut() )
            .collect::<Vec<_>>()
    }
}

macro_rules! impl_wpps_tuple {
    {$n:expr;$t:ident $($tt:ident)+;$l:ident $($ll:ident)+} => {
        impl_wpps_tuple!(($n-1);$($tt)+;$($ll)+);

        impl<'w,E,$t,$($tt),+> WidgetArray<'w,E> for ($t,$($tt),+) where
            E: Env,
            $t: AsWidget<'w,E>+Statize<E>, $t::Statur: Sized,
            $($tt: AsWidget<'w,E>+Statize<E>, $tt::Statur: Sized),+ 
        {
                fn len(&self) -> usize {
                    $n
                }
                fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()> where 'w: 's {
                    if self.len() > i { //TODO optimize (current method completely defeats the purpose of tuples)
                        Ok(self.childs().swap_remove(i))
                    }else{
                        Err(())
                    }
                }
                fn into_child(self, i: usize) -> Result<Resolvable<'w,E>,()> {
                    if self.len() > i { //TODO optimize (current method completely defeats the purpose of tuples)
                        Ok(self.into_childs().swap_remove(i))
                    }else{
                        Err(())
                    }
                }
                fn childs<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
                    let ($l,$($ll),*) = self;
                    vec![$l.as_ref(), $( $ll .as_ref() ),* ]
                }
                fn into_childs(self) -> Vec<Resolvable<'w,E>> {
                    let ($l,$($ll),*) = self;
                    vec![$l.into_ref(), $( $ll .into_ref() ),* ]
                }
        }

        impl<'w,E,$t,$($tt),+> WidgetArrayMut<'w,E> for ($t,$($tt),+) where
            E: Env,
            $t: AsWidgetMut<'w,E>+Statize<E>, $t::Statur: Sized,
            $($tt: AsWidgetMut<'w,E>+Statize<E>, $tt::Statur: Sized),+ 
        {
            fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,E>,()> where 'w: 's {
                if self.len() > i { //TODO optimize (current method completely defeats the purpose of tuples)
                    Ok(self.childs_mut().swap_remove(i))
                }else{
                    Err(())
                }
            }
            fn into_child_mut(self, i: usize) -> Result<ResolvableMut<'w,E>,()> {
                if self.len() > i { //TODO optimize (current method completely defeats the purpose of tuples)
                    Ok(self.into_childs_mut().swap_remove(i))
                }else{
                    Err(())
                }
            }
            fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
                let ($l,$($ll),*) = self;
                vec![$l.as_mut(), $( $ll .as_mut() ),* ]
            }
            fn into_childs_mut(self) -> Vec<ResolvableMut<'w,E>> {
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