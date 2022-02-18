use super::*;
use util::state::*;
use crate::event::key::Key;
use crate::style::standard::cursor::StdCursor; //TODO fix req of this import

impl<'w,E,L,R,V,TrMut> Widget<E> for SplitPane<'w,E,L,R,V,TrMut> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<E>,
    L: AsWidget<E>,
    R: AsWidget<E>,
    V: AtomState<E,f32>,
    TrMut: TriggerMut<E>,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, mut l: Link<E>, r: &mut ERenderer<'_,E>) {
        let mut r = r.with_style(&self.style);
        let mut r = r.inside_border_by(StdSelectag::BorderOuter,l.ctx);
        let bounds = self.calc_bounds(r.bounds(),self.state.get(l.ctx)); 

        {
            if l.state().is_hovered(&self.id) {
                let cursor = match self.orientation {
                    Orientation::Horizontal => StdCursor::SizeWE,
                    Orientation::Vertical => StdCursor::SizeNS,
                };

                r.set_cursor_specific(&cursor.into(),l.ctx);
            }

            r.slice_abs(&bounds[1])
                .with(StdSelectag::ObjForeground)
                .fill_rect(l.ctx);
        }

        {
            let left = l.for_child(0).expect("Dead Path inside Pane");
            let mut r = r.slice_abs(&bounds[0]);
            r.render_widget(left);
        }
        {
            let right = l.for_child(1).expect("Dead Path inside Pane");
            let mut r = r.slice_abs(&bounds[2]);
            r.render_widget(right);
        }
        {
            //TODO render center
        }
        //TODO FIX viewport
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = e.with_style(&self.style);
        let e = try_or_false!(e.filter_inside_bounds_by_style(StdSelectag::BorderOuter,l.ctx));

        let o = self.orientation;
        let mut bounds = self.calc_bounds(&e.bounds,self.state.get(l.ctx)); 

        let mut passed = false;

        //if let Some(e) = e.slice_bounds(&bounds[1]).filter(&l) {
            if let Some(mm) = e.event.is_mouse_move() {
                //if mouse is down and was pressed on us
                if let Some(_) = l.state().is_pressed_and_id(MatchKeyCode::MouseLeft,self.id.clone()) {
                    let cursor = l.state().cursor_pos().expect("TODO");
                    let mut cx = cursor.par(o);
                    let (mut wx0, ww) = e.bounds.par(o);
                    let mut wx1 = wx0 + ww as i32;

                    let l_min = l.for_child(0)
                        .expect("Dead Path inside Pane").size(&e.style)
                        .par(o).min();
                    let r_min = l.for_child(1)
                        .expect("Dead Path inside Pane").size(&e.style)
                        .par(o).min();

                    wx0 += (self.width/2) as i32;
                    wx1 -= (self.width/2) as i32;

                    let ewx0 = wx0 + l_min as i32;
                    let ewx1 = wx1 - r_min as i32;

                    cx = cx.min(ewx1-1).max(ewx0);
                    
                    if ewx1 > ewx0 {
                        let ww = wx1 - wx0;
                        cx = cx - wx0;
                        let fcx = (cx as f32)/(ww as f32);

                        if let Some(t) = self.updater.boxed(fcx) {
                            l.mutate_closure(t)
                        }

                        bounds = self.calc_bounds(&e.bounds,fcx);
                    }
                }
            }
        //}
        {
            let mut left = l.for_child(0).expect("Dead Path inside Pane");
            let sliced = &e & &bounds[0]; //TODO opion impl
            if let Some(ee) = sliced.filter(&left) {
                passed |= left.event_direct(&ee);
            }
        }
        {
            let mut right = l.for_child(1).expect("Dead Path inside Pane");
            let sliced = &e & &bounds[2];
            if let Some(ee) = sliced.filter(&right) {
                passed |= right.event_direct(&ee);
            }
        }
        passed
    }
    fn _size(&self, mut l: Link<E>, e: &EStyle<E>) -> ESize<E> {
        let e = e.and(&self.style);
        let mut s = ESize::<E>::empty();
        l.for_childs(&mut |mut l: Link<E>| s.add(&l.size(&e), self.orientation) ).expect("Dead Path inside Pane");
        s.add_space(self.width,self.orientation);
        s
    }
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> {
        Ok(self.calc_bounds(b,self.state.get(l.ctx)))
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    fn childs_ref<'s>(&'s self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> {
        self.childs.childs(root,ctx)
    }
    fn into_childs<'s>(self: Box<Self>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> where Self: 's {
        self.childs.into_childs(root,ctx)
    }

    fn focusable(&self) -> bool {
        false
    }

    fn child<'s>(&'s self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> {
        self.childs.child(i,root,ctx)
    }
    fn into_child<'s>(self: Box<Self>, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> where Self: 's {
        self.childs.into_child(i,root,ctx)
    }

    impl_traitcast!( dyn Widget<E>:
        dyn AtomState<E,f32> => |s| &s.state;
    );
}

impl<'w,E,L,R,V,TrMut> SplitPane<'w,E,L,R,V,TrMut> where
    E: Env,
    V: AtomState<E,f32>,
{
    fn calc_bounds(&self, b: &Bounds, v: f32) -> Vec<Bounds> {
        let handle_width = self.width.min(b.w());
        let o = self.orientation;
        let (x,w) = b.par(o);
        let (y,h) = b.unpar(o);
        let w0 = ((w as f32 - handle_width as f32)*v.clamp(0.0,1.0)) as u32;
        let w2 = w - w0 - handle_width;
        let x1 = x + w0 as i32;
        let x2 = x1 + handle_width as i32;
        let left = Bounds::from_ori(x, y, w0, h, o);
        let center = Bounds::from_ori(x1, y, handle_width, h, o);
        let right = Bounds::from_ori(x2, y, w2, h, o);
        vec![left,center,right]
    }
}

impl<'l,E,L,R,V,TrMut> AsWidget<E> for SplitPane<'l,E,L,R,V,TrMut> where Self: Widget<E>, E: Env {
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
