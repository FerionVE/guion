use std::any::Any;
use std::sync::Arc;

use crate::aliases::{ERenderer, EEvent, ETextLayout, ESize};
use crate::cachor::AsCachor;
use crate::dispatchor::AsWidgetDispatch;
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::layout::Gonstraints;
use crate::widget::as_widget::AsWidget;
use crate::{event_new, impl_traitcast, EventResp};
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::queron::Queron;
use crate::render::{StdRenderProps, TestStyleColorType};
use crate::render::widgets::RenderStdWidgets;
use crate::text::layout::{TxtLayoutFromStor, TxtLayout};
use crate::text::stor::TextStor;
use crate::util::tabulate::{TabulateOrigin, TabulateDirection, TabulateResponse};
use crate::widget::{Widget, WidgetWithResolveChildDyn};
use crate::widget::cache::{StdRenderCachors, ValidationStat, WidgetCache};
use crate::widget::dyn_tunnel::WidgetDyn;

use super::Label;

impl<E,Text> Widget<E> for Label<E,Text> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: TextStor<E> + AsCachor<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
{
    type Cache = LabelCache<E,Text::Cachor>;
    
    fn _render<P,Ph>(
        &self,
        _path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        mut force_render: bool,
        cache: &mut Self::Cache,
        _root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        let mut need_render = force_render;

        StdRenderCachors::current(stack)
            .validate(&mut cache.std_render_cachors, &mut need_render, &mut force_render);

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

    fn _event_direct<P,Ph,Evt>(
        &self,
        _: &Ph,
        _: &P,
        _: &Evt,
        _: Option<&(dyn PathResolvusDyn<E>+'_)>,
        _: &mut Self::Cache,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
    ) -> EventResp where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        false
    }

    fn _size<P,Ph>(
        &self,
        _path: &Ph,
        stack: &P,
        cache: &mut Self::Cache,
        _root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
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

    fn with_resolve_child<'s,F,R>(
        &'s self,
        sub_path: &(dyn PathResolvusDyn<E>+'_),
        mut callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'a,'c,'cc> FnMut(Result<WidgetWithResolveChildDyn<'a,E>,E::Error>,&'c mut E::Context<'cc>) -> R
    {
        (callback)(Err(todo!()),ctx)
    }

    fn _call_tabulate_on_child_idx<P,Ph>(
        &self,
        idx: usize,
        path: &Ph,
        stack: &P,
        op: TabulateOrigin<E>,
        dir: TabulateDirection,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Result<TabulateResponse<E>,E::Error>
    where 
        Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized
    {
        Err(todo!())
    }
    
    // fn child_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     Ok(vec![])
    // }
    fn focusable(&self) -> bool {
        false
    }

    impl_traitcast!( dyn WidgetDyn<E>:
        //dyn AsCachor<E> => |s| &s.text;
        dyn TextStor<E> => |s| &s.text;
    );
}

impl<E,Text> Label<E,Text> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: TextStor<E> + AsCachor<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
{
    fn glyphs(&self, stack: &(impl Queron<E> + ?Sized), cache: &mut LabelCache<E,Text::Cachor>, ctx: &mut E::Context<'_>) -> ValidationStat {
        //TODO also cachor e.g. style that affects text
        if cache.text_cachor.is_none() || cache.text_cache.is_none() || !self.text.valid(cache.text_cachor.as_ref().unwrap()) { //TODO old Validation trait bad coercion
            cache.text_cachor = Some(self.text.cachor());
            cache.text_cache = Some(TxtLayoutFromStor::from(&self.text,ctx));
            cache.text_rendered = false;
        }
        ValidationStat::from_valid(cache.text_rendered)
    }
}

impl<E,Text> AsWidget<E> for Label<E,Text> where Self: Widget<E>, E: Env {
    type Widget<'v,'z> = Self where 'z: 'v, Self: 'z;
    type WidgetCache = <Self as Widget<E>>::Cache;

    #[inline]
    fn with_widget<'w,R>(&self, f: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        f.call(self, root, ctx)
    }
}

pub struct LabelCache<E,TC> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, TC: Clone + PartialEq + 'static {
    text_cache: Option<ETextLayout<E>>,
    text_cachor: Option<TC>,
    text_rendered: bool,
    std_render_cachors: Option<StdRenderCachors<E>>,
    align_cachor: Option<(f32,f32)>,
    //render_style_cachor: Option<<ERenderer<'_,E> as RenderStdWidgets<E>>::RenderPreprocessedTextStyleCachors>,
}

impl<E, TC> Default for LabelCache<E, TC> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, TC: Clone + PartialEq + 'static {
    #[inline]
    fn default() -> Self {
        Self {
            text_cache: None,
            text_cachor: None,
            text_rendered: false,
            std_render_cachors: None,
            align_cachor: None
        }
    }
}

impl<E,TC> WidgetCache<E> for LabelCache<E,TC> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, TC: Clone + PartialEq + 'static {
    fn reset_current(&mut self) {}
}
