use crate::aliases::ETextLayout;
use crate::env::Env;
use crate::text::layout::TxtLayout;
use crate::util::bounds::{Offset, Bounds};

pub fn max_off<E>(g: &ETextLayout<E>, b: &Bounds) -> Offset where E: Env {
    let size = g.display_size();
    Offset {
        x: size.w.saturating_sub( b.w() ) as i32,
        y: size.h.saturating_sub( b.h() ) as i32,
    }
}
