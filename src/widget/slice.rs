//! Trait over types holding an array of AsWidget types
use super::*;

pub trait WidgetSlice<E>: Sized where E: Env {
    fn len(&self) -> usize;
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()>;
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w;
    fn childs<'s>(&'s self) -> Vec<Resolvable<'s,E>>;
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w;
}
pub trait WidgetSliceMut<E>: WidgetSlice<E> where E: Env {
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,E>,()>;
    fn into_child_mut<'w>(self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w;
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>>;
    fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w;
}

impl<T,E> WidgetSlice<E> for Vec<T> where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        self.get(i)
            .map(AsWidget::as_ref)
            .ok_or(())
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
    fn childs(&self) -> Vec<Resolvable<E>> {
        self.iter()
            .map(AsWidget::as_ref)
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w| w.into_ref() )
            .collect::<Vec<_>>()
    }
}
impl<T,E> WidgetSliceMut<E> for Vec<T> where T: AsWidgetMut<E>, E: Env {
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
        self.get_mut(i)
            .map(AsWidgetMut::as_mut)
            .ok_or(())
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
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
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

impl<T,E> WidgetSlice<E> for &[T] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        self.get(i)
            .map(AsWidget::as_ref)
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        self.get(i)
            .map(AsWidget::as_ref)
            .ok_or(())
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<E>> {
        self.iter()
            .map(AsWidget::as_ref)
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(AsWidget::as_ref)
            .collect::<Vec<_>>()
    }
}

impl<T,E> WidgetSlice<E> for &mut [T] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        self.get(i)
            .map(AsWidget::as_ref)
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        self.get(i)
            .map(AsWidget::as_ref)
            .ok_or(())
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<E>> {
        self.iter()
            .map(AsWidget::as_ref)
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w| w.as_ref() )
            .collect::<Vec<_>>()
    }
}
impl<T,E> WidgetSliceMut<E> for &mut [T] where T: AsWidgetMut<E>, E: Env {
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
        self.get_mut(i)
            .map(AsWidgetMut::as_mut)
            .ok_or(())
    }
    #[inline]
    fn into_child_mut<'w>(self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        self.get_mut(i)
            .map(AsWidgetMut::as_mut)
            .ok_or(())
    }
    #[inline]
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        self.iter_mut()
            .map(AsWidgetMut::as_mut)
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
        self.into_iter()
            .map(AsWidgetMut::as_mut)
            .collect::<Vec<_>>()
    }
}

macro_rules! impl_wpps_tuple {
    {
        $n:expr;
        $senf:ident;

        $t:ident $($tt:ident)+;
        $l:ident $($ll:ident)+;
        
        $m:pat => $x:expr, $($mm:pat => $xx:expr),+
    } => {
        impl_wpps_tuple!(($n-1);$senf;$($tt)+;$($ll)+;$($mm => $xx),+);

        impl<E,$t,$($tt),+> WidgetSlice<E> for ($t,$($tt),+) where
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
                    $m => AsWidget::as_ref(& $x),
                    $($mm => AsWidget::as_ref(& $xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::into_ref($x),
                    $($mm => AsWidget::into_ref($xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn childs(&self) -> Vec<Resolvable<E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_ref(), $( $ll .as_ref() ),* ]
            }
            #[inline]
            fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.into_ref(), $( $ll .into_ref() ),* ]
            }
        }
        impl<E,$t,$($tt),+> WidgetSliceMut<E> for ($t,$($tt),+) where
            E: Env,
            $t: AsWidgetMut<E>,
            $($tt: AsWidgetMut<E>),+ 
        {
            #[inline]
            fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
                let $senf = self;
                Ok(match i {
                    $m => AsWidgetMut::as_mut(&mut $x),
                    $($mm => AsWidgetMut::as_mut(&mut $xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn into_child_mut<'w>(self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
                let $senf = self;
                Ok(match i {
                    $m => AsWidgetMut::into_mut($x),
                    $($mm => AsWidgetMut::into_mut($xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_mut(), $( $ll .as_mut() ),* ]
            }
            #[inline]
            fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.into_mut(), $( $ll .into_mut() ),* ]
            }
        }

        impl<E,$t,$($tt),+> WidgetSlice<E> for &($t,$($tt),+) where
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
                    $m => AsWidget::as_ref(& $x),
                    $($mm => AsWidget::as_ref(& $xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::as_ref(& $x),
                    $($mm => AsWidget::as_ref(& $xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn childs(&self) -> Vec<Resolvable<E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_ref(), $( $ll .as_ref() ),* ]
            }
            #[inline]
            fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.as_ref(), $( $ll .as_ref() ),* ]
            }
        }

        impl<E,$t,$($tt),+> WidgetSlice<E> for &mut ($t,$($tt),+) where
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
                    $m => AsWidget::as_ref(& $x),
                    $($mm => AsWidget::as_ref(& $xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::as_ref(& $x),
                    $($mm => AsWidget::as_ref(& $xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn childs(&self) -> Vec<Resolvable<E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_ref(), $( $ll .as_ref() ),* ]
            }
            #[inline]
            fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.as_ref(), $( $ll .as_ref() ),* ]
            }
        }
        impl<E,$t,$($tt),+> WidgetSliceMut<E> for &mut ($t,$($tt),+) where
            E: Env,
            $t: AsWidgetMut<E>,
            $($tt: AsWidgetMut<E>),+ 
        {
            #[inline]
            fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
                let $senf = self;
                Ok(match i {
                    $m => AsWidgetMut::as_mut(&mut $x),
                    $($mm => AsWidgetMut::as_mut(&mut $xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn into_child_mut<'w>(self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
                let $senf = self;
                Ok(match i {
                    $m => AsWidgetMut::as_mut(&mut $x),
                    $($mm => AsWidgetMut::as_mut(&mut $xx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_mut(), $( $ll .as_mut() ),* ]
            }
            #[inline]
            fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.as_mut(), $( $ll .as_mut() ),* ]
            }
        }
    };
    {
        $n:expr;
        $senf:ident;
        
        $t:ident;$l:ident;
        $m:pat => $x:expr
    } => {}
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


impl<T,E,const N: usize> WidgetSlice<E> for [T;N] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        N
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        self.get(i)
            .map(AsWidget::as_ref)
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        std::array::IntoIter::new(self)
            .skip(i).next()
            .map(AsWidget::into_ref)
            .ok_or(())
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<E>> {
        self.iter()
            .map(#[inline] |w| w.as_ref() )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        std::array::IntoIter::new(self)
            .map(#[inline] |w| w.into_ref() )
            .collect::<Vec<_>>()
    }
}
impl<T,E,const N: usize> WidgetSliceMut<E> for [T;N] where T: AsWidgetMut<E>, E: Env {
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
        self.get_mut(i)
            .map(AsWidgetMut::as_mut)
            .ok_or(())
    }
    #[inline]
    fn into_child_mut<'w>(self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        std::array::IntoIter::new(self)
            .skip(i).next()
            .map(AsWidgetMut::into_mut)
            .ok_or(())
    }
    #[inline]
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        self.iter_mut()
            .map(AsWidgetMut::as_mut)
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
        std::array::IntoIter::new(self)
            .map(AsWidgetMut::into_mut)
            .collect::<Vec<_>>()
    }
}
impl<T,E,const N: usize> WidgetSlice<E> for &[T;N] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        self.get(i)
            .map(AsWidget::as_ref)
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        self.get(i)
            .map(AsWidget::as_ref)
            .ok_or(())
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<E>> {
        self.iter()
            .map(AsWidget::as_ref)
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(AsWidget::as_ref)
            .collect::<Vec<_>>()
    }
}
impl<T,E,const N: usize> WidgetSlice<E> for &mut [T;N] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        self.get(i)
            .map(AsWidget::as_ref)
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(self, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        self.get(i)
            .map(AsWidget::as_ref)
            .ok_or(())
    }
    #[inline]
    fn childs(&self) -> Vec<Resolvable<E>> {
        self.iter()
            .map(AsWidget::as_ref)
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w| w.as_ref() )
            .collect::<Vec<_>>()
    }
}
impl<T,E,const N: usize> WidgetSliceMut<E> for &mut [T;N] where T: AsWidgetMut<E>, E: Env {
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
        self.get_mut(i)
            .map(AsWidgetMut::as_mut)
            .ok_or(())
    }
    #[inline]
    fn into_child_mut<'w>(self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        self.get_mut(i)
            .map(AsWidgetMut::as_mut)
            .ok_or(())
    }
    #[inline]
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        self.iter_mut()
            .map(AsWidgetMut::as_mut)
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
        self.into_iter()
            .map(AsWidgetMut::as_mut)
            .collect::<Vec<_>>()
    }
}
