//! Trait over types holding an array of AsWidget types
use super::*;

pub trait WidgetArray<E>: Sized where E: Env {
    fn len(&self) -> usize;
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()>;
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<Resolvable<'s,E>,()>;
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w;
    fn childs<'s>(&'s self) -> Vec<Resolvable<'s,E>>;
    fn childs_mut<'s>(&'s mut self) -> Vec<Resolvable<'s,E>>;
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w;
}

impl<T,E> WidgetArray<E> for Vec<T> where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        Ok(self.get(i).ok_or(())?.as_widget())
    }
    #[inline]
    fn into_child<'w>(mut self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        if self.len() > i {
            Ok(self.swap_remove(i).into_widget())
        }else{
            Err(())
        }
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<E>> {
        self.iter()
            .map(#[inline] |w| w.as_widget() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w| w.into_widget() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<Resolvable<E>,()> {
        Ok(self.get_mut(i).ok_or(())?.as_widget_mut())
    }
    #[inline]
    fn childs_mut(&mut self) -> Vec<Resolvable<E>> {
        self.iter_mut()
            .map(#[inline] |w| w.as_widget_mut() )
            .collect::<Vec<_>>()
    }
}

impl<T,E> WidgetArray<E> for &[T] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        Ok(self.get(i).ok_or(())?.as_widget())
    }
    #[inline]
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        Ok(self.get(i).ok_or(())?.as_widget())
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<E>> {
        self.iter()
            .map(#[inline] |w| w.as_widget() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w: &'w T| w.as_widget() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<Resolvable<E>,()> {
        Ok(self.get_mut(i).ok_or(())?.as_widget_mut())
    }
    #[inline]
    fn childs_mut(&mut self) -> Vec<Resolvable<E>> {
        self.iter_mut()
            .map(#[inline] |w| w.as_widget_mut() )
            .collect::<Vec<_>>()
    }
}

impl<T,E> WidgetArray<E> for &mut [T] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        Ok(self.get(i).ok_or(())?.as_widget())
    }
    #[inline]
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        Ok(self.get(i).ok_or(())?.as_widget())
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<E>> {
        self.iter()
            .map(#[inline] |w| w.as_widget() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w: &'w mut T| w.as_widget() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<Resolvable<E>,()> {
        Ok(self.get_mut(i).ok_or(())?.as_widget_mut())
    }
    #[inline]
    fn childs_mut(&mut self) -> Vec<Resolvable<E>> {
        self.iter_mut()
            .map(#[inline] |w| w.as_widget_mut() )
            .collect::<Vec<_>>()
    }
}

macro_rules! impl_wpps_tuple {
    {$n:expr;$senf:ident;$t:ident $($tt:ident)+;$l:ident $($ll:ident)+;$m:pat => $x:expr,$($mm:pat => $xx:expr),+} => {
        impl_wpps_tuple!(($n-1);$senf;$($tt)+;$($ll)+;$($mm => $xx),+);

        impl<E,$t,$($tt),+> WidgetArray<E> for ($t,$($tt),+) where
            E: Env,
            $t: AsWidget<E>,
            $($tt: AsWidget<E>),+ 
        {
            #[inline]
            fn len(&self) -> usize {
                $n
            }
            #[inline]
            fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::as_widget(& $x),
                    $($mm => AsWidget::as_widget(& $xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn child_mut(&mut self, i: usize) -> Result<Resolvable<E>,()> {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::as_widget_mut(&mut $x),
                    $($mm => AsWidget::as_widget_mut(&mut $xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::into_widget($x),
                    $($mm => AsWidget::into_widget($xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn childs(&self) -> Vec<Resolvable<E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_widget(), $( $ll .as_widget() ),* ]
            }
            #[inline]
            fn childs_mut(&mut self) -> Vec<Resolvable<E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_widget_mut(), $( $ll .as_widget_mut() ),* ]
            }
            #[inline]
            fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.into_widget(), $( $ll .into_widget() ),* ]
            }
        }
    };
    {$n:expr;$senf:ident;$t:ident;$l:ident;$m:pat => $x:expr} => {}
}

impl_wpps_tuple!(
    32;senf;
    A B C D F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE AF AG;
    a b c d f g h i j k l m n o p q r s t u v w x y z aa ab ac ad ae af ag;
    31 => senf.31,30 => senf.30,29 => senf.29,28 => senf.28,
    27 => senf.27,26 => senf.26,25 => senf.25,24 => senf.24,
    23 => senf.23,22 => senf.22,21 => senf.21,20 => senf.20,
    19 => senf.19,18 => senf.18,17 => senf.17,16 => senf.16,
    15 => senf.15,14 => senf.14,13 => senf.13,12 => senf.12,
    11 => senf.11,10 => senf.10,09 => senf. 9,08 => senf. 8,
    07 => senf. 7,06 => senf. 6,05 => senf. 5,04 => senf. 4,
    03 => senf. 3,02 => senf. 2,01 => senf. 1,00 => senf. 0
);
