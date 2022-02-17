use super::*;

impl<'w,E,T> Widget<E> for Pane<'w,E,T> where
    E: Env,
    T: WidgetArray<E>,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut ERenderer<'_,E>) {
        self._render_impl(l,r)
    }
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp {
        self._event_direct_impl(l,e)
    }
    fn _size(&self, l: Link<E>, e: &EStyle<E>) -> ESize<E> {
        self._size_impl(l,e)
    }
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> {
        self.child_bounds_impl(l,b,e,force)
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    fn childs_ref<'s>(&'s self, r: E::RootRef<'_>, c: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> {
        self.childs.childs(r,c)
    }
    fn into_childs<'s>(self: Box<Self>, r: E::RootRef<'_>, c: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> where Self: 's {
        self.childs.into_childs(r,c)
    }

    fn focusable(&self) -> bool {
        false
    }

    fn child<'s>(&'s self, i: usize, r: E::RootRef<'_>, c: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> {
        self.childs.child(i,r,c)
    }
    fn into_child<'s>(self: Box<Self>, i: usize, r: E::RootRef<'_>, c: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> where Self: 's {
        self.childs.into_child(i,r,c)
    }
}

impl<'w,E,T> Pane<'w,E,T> where
    E: Env,
    T: WidgetArray<E>,
{
    pub fn _render_impl(&self, mut l: Link<E>, r: &mut ERenderer<'_,E>) where
        E: Env,
    {
        let mut r = r.with_style(&self.style);
        let mut r = r.inside_border_by(StdSelectag::BorderOuter,l.ctx);
        let sizes = l.child_sizes(r.style()).expect("Dead Path Inside Pane");
        let bounds = calc_bounds(&r.bounds().size,&sizes,self.orientation); 

        for i in 0..self.childs() {
            let l = l.for_child(i).expect("Dead Path Inside Pane");
            let mut r = r.slice(&bounds[i]);
            r.render_widget(l);
        }
        //TODO FIX viewport
    }

    pub fn _event_direct_impl(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp where
        E: Env,
    {
        let e = try_or_false!(e.filter_inside_bounds_by_style(StdSelectag::BorderOuter,l.ctx));
        
        let sizes = l.child_sizes(&e.style).expect("Dead Path Inside Pane");
        let bounds = calc_bounds(&e.bounds.size,&sizes,self.orientation);

        let mut passed = false;

        for i in 0..self.childs() {
            let mut l = l.for_child(i).expect("Dead Path Inside Pane");
            let sliced = e.slice_bounds(&bounds[i]);
            if let Some(ee) = sliced.filter(&l) {
                passed |= l.event_direct(&ee);
            }
        }

        passed
    }

    pub fn _size_impl(&self, mut l: Link<E>, e: &EStyle<E>) -> ESize<E> where
        E: Env,
    {
        let e = e.and(&self.style);
        let mut s = ESize::<E>::empty();
        l.for_childs(&mut |mut l: Link<E>| s.add(&l.size(&e), self.orientation) ).expect("Dead Path inside Pane");
        s
    }

    pub fn child_bounds_impl(&self, mut l: Link<E>, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> where
        E: Env,
    {
        let e = e.and(&self.style);
        let sizes = l.child_sizes(&e).expect("Dead Path Inside Pane");
        let bounds = calc_bounds(&b.size,&sizes,self.orientation); 

        Ok(bounds)
    }
}

impl<'l,E,T> AsWidget<E> for Pane<'l,E,T> where Self: Widget<E>, E: Env {
    type Widget = Self;
    type WidgetOwned = Self;

    #[inline]
    fn as_widget<'w>(&'w self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Borrowed(self)
    }
    #[inline]
    fn into_widget<'w>(self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        WCow::Owned(self)
    }
    #[inline]
    fn box_into_widget<'w>(self: Box<Self>, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Owned(*self)
    }
    #[inline]
    fn as_widget_dyn<'w,'s>(&'w self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Borrowed(self)
    }
    #[inline]
    fn into_widget_dyn<'w,'s>(self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
        WCow::Owned(Box::new(self))
    }
    #[inline]
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Owned(self)
    }
}
