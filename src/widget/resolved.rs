//! Widget reference including it's path and a reference to the root
use super::*;

/// A reference to a resolved Widget
pub struct Resolved<'a,E> where E: Env {
    pub wref: WidgetRef<'a,E>,
    pub path: E::WidgetPath,
    pub stor: &'a E::Storage,
}
/// A mutable reference to a resolved Widget
pub struct ResolvedMut<'a,E> where E: Env {
    pub wref: WidgetRefMut<'a,E>,
    pub path: E::WidgetPath,
}

impl<'a,E> Resolved<'a,E> where E: Env {
    #[inline]
    pub fn widget(&self) -> &WidgetRef<'a,E> {
        &self.wref
    }

    #[inline]
    pub fn render(&self, c: &mut E::Context, r: &mut RenderLink<E>) -> bool {
        c.render(self.clone(),r)
    }
    #[inline]
    pub fn event(&self, c: &mut E::Context, e: (EEvent<E>,&Bounds,u64)) {
        c.event(self.clone(),e)
    }
    #[inline]
    pub fn size(&self, c: &mut E::Context) -> ESize<E> {
        c.size(self.clone())
    }

    #[inline]
    pub fn _render(&self, c: &mut E::Context, r: &mut RenderLink<E>) -> bool {
        self.widget()._render(c.link(self.clone()),r)
    }
    #[inline]
    pub fn _event(&self, c: &mut E::Context, e: (EEvent<E>,&Bounds,u64)) {
        self.widget()._event(c.link(self.clone()),e)
    }
    #[inline]
    pub fn _size(&self, c: &mut E::Context) -> ESize<E> {
        self.widget()._size(c.link(self.clone()))
    }
    #[inline]
    pub fn link(&self, c: &'a mut E::Context) -> Link<'a,E> {
        c.link(self.clone())
    }

    #[inline]
    pub fn trace_bounds(&mut self, c: &mut E::Context, root_bounds: &Bounds, force: bool) -> Bounds {
        self.stor.trace_bounds(c,self.path.refc(),root_bounds,force).unwrap()
    }

    #[inline]
    pub fn reference<'s>(&'s self) -> Resolved<'s,E> where 'a: 's {
        Resolved{
            wref: Box::new(&*self.wref),
            path: self.path.clone(),
            stor: &self.stor,
        }
    }

    /*#[inline]
    pub fn childs(&self) -> Vec<Resolved<E>> {
        (**self)._childs(self.path)
    }
    #[inline]
    pub fn childs_mut(&self) -> Vec<Resolved<E>> {
        (**self)._childs_mut(self.path)
    }*/
    #[deprecated]
    #[allow(deprecated)]
    #[inline]
    pub fn child_paths(&self) -> Vec<E::WidgetPath> {
        self.widget().child_paths(self.path.refc())
    }

    pub fn with_env<F: Env<WidgetPath=E::WidgetPath,Storage=E::Storage>>(self) -> Resolved<'a,F> where E::WidgetPath: WidgetPath<F,SubPath=EWPSub<E>>, EWPSub<E>: SubPath<F>, E::Storage: Widgets<F> {
        let stor = self.stor.with_env::<F>();
        let path = rc_path_with_env::<E,F>(self.path.refc());
        stor.widget(path).unwrap()
    }
}

impl<'a,E> ResolvedMut<'a,E> where E: Env {
    #[inline]
    pub fn widget<'s>(&'s mut self) -> &'s mut (dyn WidgetMut<'s,E>+'s) where 'a: 's {
        (&mut (*self.wref)).short_lt()
    }
}

impl<'a,E> Clone for Resolved<'a,E> where E: Env {
    fn clone(&self) -> Self {
        self.stor.widget(self.path.refc()).unwrap()
    }
}
