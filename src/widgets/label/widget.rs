use super::*;
use std::sync::Arc;
use util::{caption::CaptionMut, state::{AtomStateMut, AtomState}};
use validation::{ValidationMut, Validation};

impl<'w,E,Text,GlyphCache> Widget<E> for Label<'w,E,Text,GlyphCache> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: Caption<E>+Validation<E>+'w,
    GlyphCache: AtomState<E,LocalGlyphCache<E>>+Clone,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        let mut r = r.with(&self.style);
        r.with(&[
            StdSelector::ObjForeground,
            StdSelector::ObjText,
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
    fn childs_ref(&self) -> Vec<Resolvable<E>> {
        vec![]
    }
    fn into_childs<'a>(self: Box<Self>) -> Vec<Resolvable<'a,E>> where Self: 'a {
        vec![]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &EStyle<E>, _: bool) -> Result<Vec<Bounds>,()> {
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        false
    }
    fn child(&self, _: usize) -> Result<Resolvable<E>,()> {
        Err(())
    }
    fn into_child<'a>(self: Box<Self>, _: usize) -> Result<Resolvable<'a,E>,()> where Self: 'a {
        Err(())
    }

    impl_traitcast!(
        dyn AtomState<E,LocalGlyphCache<E>> => |s| &s.glyph_cache;
        dyn Validation<E> => |s| &s.text;
        dyn Caption<E> => |s| &s.text;
    );
}

impl<'w,E,Text,GlyphCache> WidgetMut<E> for Label<'w,E,Text,GlyphCache> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: CaptionMut<E>+ValidationMut<E>+'w,
    GlyphCache: AtomStateMut<E,LocalGlyphCache<E>>+Clone,
{
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        vec![]
    }
    fn into_childs_mut<'a>(self: Box<Self>) -> Vec<ResolvableMut<'a,E>> where Self: 'a {
        vec![]
    }
    fn child_mut(&mut self, _: usize) -> Result<ResolvableMut<E>,()> {
        Err(())
    }
    fn into_child_mut<'a>(self: Box<Self>, _: usize) -> Result<ResolvableMut<'a,E>,()> where Self: 'a {
        Err(())
    }

    impl_traitcast_mut!(
        dyn AtomStateMut<E,LocalGlyphCache<E>> => |s| &mut s.glyph_cache;
        dyn ValidationMut<E> => |s| &mut s.text;
        dyn CaptionMut<E> => |s| &mut s.text;
    );
}

impl<'w,E,Text,GlyphCache> Label<'w,E,Text,GlyphCache> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: Caption<E>+Validation<E>+'w,
    GlyphCache: AtomState<E,LocalGlyphCache<E>>+Clone,
{
    fn glyphs(&self, mut l: Link<E>) -> Arc<ESGlyphs<E>> {
        if let Some((v,c)) = self.glyph_cache.get(l.ctx) {
            if self.text.valid(&c) {
                return v;
            }
        }

        let text = self.text.caption();
        let glyphs = Arc::new(ESGlyphs::<E>::generate(text.as_ref(),(20.0,20.0),l.ctx));

        let g = glyphs.refc();
        l.mutate_closure(Box::new(move |mut w,ctx,_| {
            let vali = w.traitcast_mut::<dyn ValidationMut<E>>().unwrap();
            let key = vali.validate();
            let cache = w.traitcast_mut::<dyn AtomStateMut<E,LocalGlyphCache<E>>>().unwrap();
            cache.set( Some((g,key)) ,ctx);
        }));

        glyphs
    }
}
