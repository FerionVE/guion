use crate::queron::Queron;
use crate::text::layout::*;
use crate::widget::cache::{WidgetCache, StdRenderCachors, ValidationStat};
use crate::widget::dyn_tunnel::WidgetDyn;

use super::*;
use std::sync::Arc;
use util::state::AtomState;
use validation::Validation;

impl<'w,E,Text> Widget<E> for Label<'w,E,Text> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: TextStor<E>+Validation<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
{
    type Cache = LabelCache<E>;

    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    
    fn _render<P>(
        &self,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where P: Queron<E> + ?Sized {
        let mut need_render = force_render;

        need_render |= StdRenderCachors::current(stack).validate(&mut cache.std_render_cachors);

        //TODO cachor align and style stuff e.g. bg color
        //TODO text layout cachors
        need_render |= self.glyphs(stack, cache, ctx);

        if cache.align_cachor != Some(self.align) {
            need_render = true;
            cache.align_cachor = Some(self.align);
        }

        if !need_render {return;}

        let render_props = StdRenderProps::new(&stack);

        let text_layout = cache.text_cache.as_ref().unwrap();

        renderer.fill_rect(
            &render_props
                .with_style_color_type(TestStyleColorType::Bg),
            ctx
        );

        //TODO way to inject props/style to widget
        renderer.render_preprocessed_text(
            text_layout,
            Default::default(),
            &StdRenderProps::new(&stack)
                .inner_aligned(text_layout.display_size(),self.align),
            ctx
        );

        cache.text_rendered = true;
    }

    fn _event_direct<P,Evt>(
        &self,
        stack: &P,
        event: &Evt,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        false
    }

    fn _size<P>(
        &self,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where P: Queron<E> + ?Sized {
        self.glyphs(stack, cache, ctx);

        let ms = cache.text_cache.as_ref().unwrap().display_size();
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
        dyn Validation<E> => |s| &s.text;
        dyn TextStor<E> => |s| &s.text;
    );
}

impl<'w,E,Text> Label<'w,E,Text> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: TextStor<E>+Validation<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
{
    fn glyphs(&self, stack: &(impl Queron<E> + ?Sized), cache: &mut LabelCache<E>, ctx: &mut E::Context<'_>) -> ValidationStat {
        //TODO also cachor e.g. style that affects text
        if cache.text_cachor.is_none() || cache.text_cache.is_none() || !self.text.valid(&**cache.text_cachor.as_ref().unwrap()) { //TODO old Validation trait bad coercion
            cache.text_cachor = Some(self.text.validation());
            cache.text_cache = Some(TxtLayoutFromStor::from(&self.text,ctx));
            cache.text_rendered = false;
        }
        ValidationStat::from_valid(cache.text_rendered)
    }
}

impl<'z,E,Text> AsWidget<'z,E> for Label<'z,E,Text> where Self: Widget<E>, E: Env {
    type Widget<'v> = Self where 'z: 'v;
    type WidgetCache = <Self as Widget<E>>::Cache;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: dispatchor::AsWidgetDispatch<'z,Self,R,E>
    {
        f.call(self, root, ctx)
    }
}

#[derive(Default)]
pub struct LabelCache<E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, {
    text_cache: Option<ETextLayout<E>>,
    text_cachor: Option<Arc<dyn Any>>,
    text_rendered: bool,
    std_render_cachors: Option<StdRenderCachors<E>>,
    align_cachor: Option<(f32,f32)>,
    //render_style_cachor: Option<<ERenderer<'_,E> as RenderStdWidgets<E>>::RenderPreprocessedTextStyleCachors>,
}

impl<E> WidgetCache<E> for LabelCache<E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E> {
    fn reset_current(&mut self) {}
}
