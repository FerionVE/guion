use self::bounds::Dims;

pub mod bounds;
pub mod border;
pub mod bounded_widget;
pub mod translate;
//pub mod mapped;
pub mod error;
pub mod tabulate;
pub mod immu;

pub trait AsRefMut<T> {
    fn as_ref(&self) -> &T;
    fn as_mut(&mut self) -> &mut T;
}

impl<T,I> AsRefMut<I> for T where T: AsRef<I> + AsMut<I> {
    #[inline]
    fn as_ref(&self) -> &I {
        self.as_ref()
    }
    #[inline]
    fn as_mut(&mut self) -> &mut I {
        self.as_mut()
    }
}

pub type ScrollOff = (i32,i32);

/// Normalizes scroll offset to viewport to inside inner area
/// 
/// The scroll/viewport off is the offset of the viewport relative to the inner area  
/// the viewport_off can only be negative if the viewport is bigger than the inner area
/// 
/// viewport_off: the non-normalized viewport scroll offset
/// inner_size: the size of the inner ares
/// viewport_size: the size of the visible viewport in which the inner area is scrolled/offsetted
#[inline]
pub fn normalize_scroll_off(viewport_off: ScrollOff, inner_size: Dims, viewport_size: Dims, negative_scroll: bool) -> ScrollOff {
    (
        normalize_scroll_off_axis(inner_size.w, viewport_size.w, viewport_off.0, negative_scroll),
        normalize_scroll_off_axis(inner_size.h, viewport_size.h, viewport_off.1, negative_scroll),
    )
}

#[inline]
fn normalize_scroll_off_axis(inner_size: u32, viewport_size: u32, viewport_off: i32, negative_scroll: bool) -> i32 {
    if viewport_size > inner_size {
        if negative_scroll {
            viewport_off.clamp(inner_size as i32 - viewport_size as i32, 0)
        } else {
            0
        }
    }else{
        viewport_off.clamp(0, inner_size as i32 - viewport_size as i32)
    }
}
