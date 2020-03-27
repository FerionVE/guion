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
            Ok(self.swap_remove(i).consume_ref())
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
            .map(|w| w.consume_ref() )
            .collect::<Vec<_>>()
    }
}
impl<'w,T,E> WidgetArrayMut<'w,E> for Vec<T> where T: AsWidgetMut<'w,E>+Statize<E>, T::Statur: Sized, E: Env {
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,E>,()> where 'w: 's {
        Ok(self.get_mut(i).ok_or(())?.as_mut())
    }
    fn into_child_mut(mut self, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        if self.len() > i {
            Ok(self.swap_remove(i).consume_mut())
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
            .map(|w| w.consume_mut() )
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
