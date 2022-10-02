use crate::queron::Queron;
use crate::text::layout::*;
use crate::widget::dyn_tunnel::WidgetDyn;

use super::*;
use std::sync::Arc;
use util::state::AtomState;
use validation::Validation;

impl<'w,E,Text,GlyphCache> Widget<E> for Label<'w,E,Text,GlyphCache> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: TextStor<E>+Validation<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
    GlyphCache: AtomState<E,LocalGlyphCache<E>>+Clone,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    
    fn _render<P>(
        &self,
        stack: &P,
        r: &mut ERenderer<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where P: Queron<E> + ?Sized {
        //TODO way to inject props/style to widget
        r.render_text(
            self.text.caption().as_ref(),
            self.align,
            stack,
            ctx,
        )
    }

    fn _event_direct<P,Evt>(
        &self,
        stack: &P,
        e: &Evt,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        false
    }

    fn _size<P>(
        &self,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where P: Queron<E> + ?Sized {
        let ms = self.glyphs(ctx).display_size();
        let ms = ESize::<E>::fixed(ms.w, ms.h);
        ms.max( &self.size )
    }

    fn childs(&self) -> usize {
        0
    }
    fn with_child<'s,F,R>(
        &'s self,
        i: usize,
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'www,'ww,'c,'cc> FnOnce(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R
    {
        (callback)(Err(()),ctx)
    }
    
    fn child_bounds<P>(&self, stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        false
    }

    impl_traitcast!( dyn WidgetDyn<E>:
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
    fn glyphs(&self, ctx: &mut E::Context<'_>) -> Arc<ETextLayout<E>> {
        if let Some((v,c)) = self.glyph_cache.get(ctx) {
            if self.text.valid(&c) {
                return v;
            }
        }

        let glyphs: Arc<ETextLayout<E>> = Arc::new(
            TxtLayoutFromStor::<Text,E>::from(&self.text,ctx)
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

impl<'z,E,Text,GlyphCache> AsWidget<'z,E> for Label<'z,E,Text,GlyphCache> where Self: Widget<E>, E: Env {
    type Widget<'v> = Self where 'z: 'v;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: dispatchor::AsWidgetDispatch<'z,Self,R,E>
    {
        f.call(self, root, ctx)
    }
}
