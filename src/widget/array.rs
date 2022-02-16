//! Trait over types holding an array of AsWidget types
use super::*;

pub trait WidgetArray<E>: Sized where E: Env {
    fn len(&self) -> usize;
    fn child<'s>(&'s self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()>;
    fn into_child<'w>(self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'w,E>,()> where Self: 'w;
    fn childs<'s>(&'s self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>>;
    fn into_childs<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'w,E>> where Self: 'w;
}

impl<T,E> WidgetArray<E> for Vec<T> where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
    #[inline]
    fn child(&self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<E>,()> {
        self.get(i)
            .map(|w| w.as_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(mut self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'w,E>,()> where Self: 'w {
        if self.len() > i {
            Ok(self.swap_remove(i).into_widget(root,ctx))
        }else{
            Err(())
        }
    }
    #[inline]
    fn childs(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<E>> {
        self.iter()
            .map(|w| w.as_widget(root,ctx))
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w| w.into_widget(root,ctx) )
            .collect::<Vec<_>>()
    }
}

impl<T,E> WidgetArray<E> for &[T] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<E>,()> {
        self.get(i)
            .map(|w| w.as_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'w,E>,()> where Self: 'w {
        self.get(i)
            .map(|w| w.as_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn childs(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<E>> {
        self.iter()
            .map(|w| w.as_widget(root,ctx))
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'w,E>> where Self: 'w {
        self.iter()
            .map(|w| w.as_widget(root,ctx))
            .collect::<Vec<_>>()
    }
}

impl<T,E> WidgetArray<E> for &mut [T] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<E>,()> {
        self.get(i)
            .map(|w| w.as_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'w,E>,()> where Self: 'w {
        self.get(i)
            .map(|w| w.as_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn childs(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<E>> {
        self.iter()
            .map(|w| w.as_widget(root,ctx))
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'w,E>> where Self: 'w {
        self.iter()
            .map(|w| w.as_widget(root,ctx))
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
            fn child(&self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<E>,()> {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::as_widget(& $x ,root,ctx),
                    $($mm => AsWidget::as_widget(& $xx ,root,ctx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn into_child<'w>(self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'w,E>,()> where Self: 'w {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::into_widget($x ,root,ctx),
                    $($mm => AsWidget::into_widget($xx ,root,ctx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn childs(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_widget(root,ctx), $( $ll .as_widget(root,ctx) ),* ]
            }
            #[inline]
            fn into_childs<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.into_widget(root,ctx), $( $ll .into_widget(root,ctx) ),* ]
            }
        }

        impl<E,$t,$($tt),+> WidgetArray<E> for &($t,$($tt),+) where
            E: Env,
            $t: AsWidget<E>,
            $($tt: AsWidget<E>),+ 
        {
            #[inline]
            fn len(&self) -> usize {
                $n
            }
            #[inline]
            fn child(&self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<E>,()> {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::as_widget(& $x ,root,ctx),
                    $($mm => AsWidget::as_widget(& $xx ,root,ctx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn into_child<'w>(self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'w,E>,()> where Self: 'w {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::as_widget(& $x ,root,ctx),
                    $($mm => AsWidget::as_widget(& $xx ,root,ctx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn childs(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_widget(root,ctx), $( $ll .as_widget(root,ctx) ),* ]
            }
            #[inline]
            fn into_childs<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.as_widget(root,ctx), $( $ll .as_widget(root,ctx) ),* ]
            }
        }

        impl<E,$t,$($tt),+> WidgetArray<E> for &mut ($t,$($tt),+) where
            E: Env,
            $t: AsWidget<E>,
            $($tt: AsWidget<E>),+ 
        {
            #[inline]
            fn len(&self) -> usize {
                $n
            }
            #[inline]
            fn child(&self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<E>,()> {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::as_widget(& $x ,root,ctx),
                    $($mm => AsWidget::as_widget(& $xx ,root,ctx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn into_child<'w>(self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'w,E>,()> where Self: 'w {
                let $senf = self;
                Ok(match i {
                    $m => AsWidget::as_widget(& $x ,root,ctx),
                    $($mm => AsWidget::as_widget(& $xx ,root,ctx)),+ ,
                    _ => return Err(()),
                })
            }
            #[inline]
            fn childs(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<E>> {
                let ($l,$($ll),*) = self;
                vec![$l.as_widget(root,ctx), $( $ll .as_widget(root,ctx) ),* ]
            }
            #[inline]
            fn into_childs<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'w,E>> where Self: 'w {
                let ($l,$($ll),*) = self;
                vec![$l.as_widget(root,ctx), $( $ll .as_widget(root,ctx) ),* ]
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


impl<T,E,const N: usize> WidgetArray<E> for [T;N] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        N
    }
    #[inline]
    fn child(&self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<E>,()> {
        self.get(i)
            .map(|w| w.as_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'w,E>,()> where Self: 'w {
        std::array::IntoIter::new(self)
            .skip(i).next()
            .map(|w| w.into_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn childs(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<E>> {
        self.iter()
            .map(#[inline] |w| w.as_widget(root,ctx) )
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'w,E>> where Self: 'w {
        std::array::IntoIter::new(self)
            .map(#[inline] |w| w.into_widget(root,ctx) )
            .collect::<Vec<_>>()
    }
}

impl<T,E,const N: usize> WidgetArray<E> for &[T;N] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<E>,()> {
        self.get(i)
            .map(|w| w.as_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'w,E>,()> where Self: 'w {
        self.get(i)
            .map(|w| w.as_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn childs(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<E>> {
        self.iter()
            .map(#[inline] |w| w.as_widget(root,ctx))
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w| w.as_widget(root,ctx))
            .collect::<Vec<_>>()
    }
}
impl<T,E,const N: usize> WidgetArray<E> for &mut [T;N] where T: AsWidget<E>, E: Env {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn child(&self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<E>,()> {
        self.get(i)
            .map(|w| w.as_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn into_child<'w>(self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'w,E>,()> where Self: 'w {
        self.get(i)
            .map(|w| w.as_widget(root,ctx))
            .ok_or(())
    }
    #[inline]
    fn childs(&self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<E>> {
        self.iter()
            .map(#[inline] |w| w.as_widget(root,ctx))
            .collect::<Vec<_>>()
    }
    #[inline]
    fn into_childs<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'w,E>> where Self: 'w {
        self.into_iter()
            .map(#[inline] |w| w.as_widget(root,ctx))
            .collect::<Vec<_>>()
    }
}
