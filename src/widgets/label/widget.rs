use crate::text::layout::*;
use crate::text::stor::TextStorMut;

use super::*;
use std::sync::Arc;
use util::state::{AtomStateMut, AtomState};
use validation::{ValidationMut, Validation};

impl<'w,E,Text,GlyphCache> Widget<E> for Label<'w,E,Text,GlyphCache> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: TextStor<E>+Validation<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
    GlyphCache: AtomState<E,LocalGlyphCache<E>>+Clone,
{
    fn child_paths(&self, _: E::WidgetPath, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut ERenderer<'_,E>) {
        let mut r = r.with_style(&self.style);
        r.with(&[
            StdSelectag::ObjForeground,
            StdSelectag::ObjText,
        ][..])
            .render_text(self.text.caption().as_ref(),self.align,l.ctx);
    }
    fn _event_direct(&self, _: Link<E>, _: &EventCompound<E>) -> EventResp {
        false
    }
    fn _size(&self, l: Link<E>, _: &EStyle<E>) -> ESize<E> {
        let ms = self.glyphs(l).size();
        let ms = ESize::<E>::fixed(ms.w, ms.h);
        ms.max( &self.size )
    }
    fn childs(&self) -> usize {
        0
    }
    fn childs_ref<'s>(&'s self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> {
        vec![]
    }
    fn into_childs<'s>(self: Box<Self>, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> where Self: 's {
        vec![]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &EStyle<E>, _: bool) -> Result<Vec<Bounds>,()> {
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        false
    }
    fn child<'s>(&'s self, _: usize, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> {
        Err(())
    }
    fn into_child<'s>(self: Box<Self>, _: usize, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> where Self: 's {
        Err(())
    }

    impl_traitcast!( dyn Widget<E>:
        dyn AtomState<E,LocalGlyphCache<E>> => |s| &s.glyph_cache;
        dyn Validation<E> => |s| &s.text;
        dyn TextStor<E> => |s| &s.text;
    );
}

impl<'w,E,Text,GlyphCache> Label<'w,E,Text,GlyphCache> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: TextStor<E>+Validation<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
    GlyphCache: AtomState<E,LocalGlyphCache<E>>+Clone,
{
    fn glyphs(&self, mut l: Link<E>) -> Arc<ETextLayout<E>> {
        if let Some((v,c)) = self.glyph_cache.get(l.ctx) {
            if self.text.valid(&c) {
                return v;
            }
        }

        let glyphs: Arc<ETextLayout<E>> = Arc::new(
            TxtLayoutFromStor::<Text,E>::from(&self.text,l.ctx)
        );

        let g = glyphs.refc();
        //TODO fix glyph caching as WidgetMut is GONE, label would require a mutor closure from the view using it
        // l.mutate_closure(Box::new(move |mut w,ctx,_| {
        //     let vali = w.traitcast_mut::<dyn ValidationMut<E>>().unwrap();
        //     let key = vali.validate();
        //     let cache = w.traitcast_mut::<dyn AtomStateMut<E,LocalGlyphCache<E>>>().unwrap();
        //     cache.set( Some((g,key)) ,ctx);
        // }));

        glyphs
    }
}

impl<'l,E,Text,GlyphCache> AsWidget<E> for Label<'l,E,Text,GlyphCache> where Self: Widget<E>, E: Env {
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
