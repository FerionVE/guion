use super::*;
use util::{state::*};

impl<'w,E,W,Scroll,Stil> Widget<'w,E> for Area<'w,E,W,Scroll,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    W: AsWidget<'w,E>+StatizeSized<E>+'w,
    Scroll: AtomState<E,(u32,u32)>+StatizeSized<E>,
    Stil: Clone + StatizeSized<E>,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, mut l: Link<E>, r: &mut RenderLink<E>) {
        let mut r = r.with(&self.style);
        let mut r = r.inside_border_by(StdTag::BorderOuter,l.ctx);
        r.with(&[
            StdTag::ObjBorder,
            StdTag::Focused(l.is_focused()),
            StdTag::BorderVisual,
        ][..])
            .fill_border_inner(l.ctx);
        r.inside_border_by(StdTag::BorderVisual,l.ctx)
            .render_widget(l.for_child(0).unwrap());
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = try_or_false!(e.filter_bounds_by_border(l.style_provider(),StdTag::BorderOuter));
        let e = try_or_false!(e.filter_bounds_by_border(l.style_provider(),StdTag::BorderVisual));
        todo!()
    }
    fn _size(&self, _: Link<E>) -> ESize<E> {
        self.size.clone()
    }
    fn childs(&self) -> usize {
        1
    }
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        vec![self.inner.as_ref()]
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        vec![self.inner.into_ref()]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, _: bool) -> Result<Vec<Bounds>,()> {
        todo!() // TODO complete inner bounds or just view
    }
    fn focusable(&self) -> bool {
        todo!()
    }
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        if i != 0 {return Err(());}
        Ok(self.inner.as_ref())
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        if i != 0 {return Err(());}
        Ok(self.inner.into_ref())
    }
}

impl<'w,E,W,Scroll,Stil> WidgetMut<'w,E> for Area<'w,E,W,Scroll,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>,
    W: AsWidgetMut<'w,E>+StatizeSized<E>+'w,
    Scroll: AtomStateMut<E,(u32,u32)>+StatizeSized<E>,
    Stil: Clone + StatizeSized<E>,
{
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        vec![self.inner.as_mut()]
    }
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        vec![self.inner.into_mut()]
    }
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a {
        if i != 0 {return Err(());}
        Ok(self.inner.as_mut())
    }
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        if i != 0 {return Err(());}
        Ok(self.inner.into_mut())
    }

    impl_traitcast!(
        dyn AtomStateMut<E,(u32,u32)> => |s| &s.scroll;
    );
    impl_traitcast_mut!(
        dyn AtomStateMut<E,(u32,u32)> => |s| &mut s.scroll;
    );
}
