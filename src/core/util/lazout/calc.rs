use crate::core::util::bounded_widget::BoundedWidget;
use crate::core::env::*;
use std::borrow::Borrow;
use crate::core::widget::Widget;
use super::*;

pub fn sum(i: &[LazoutDir]) -> LazoutDir {
    i.iter().fold(
        LazoutDir{
            min: 0,
            max: 0,
            optimal: 0, 
            pressure: 0.0,
        },
        |mut d,s| {
            d.min += s.min;
            d.max += s.max;
            d.optimal += s.optimal;
            d.pressure += s.pressure.max(0.0);
            d
        }
    )
}

pub fn calc<W: Borrow<E::WidgetID>, E: Env>(i: &[W], b: (u32,u32), o: Orientation, c: &E::Ctx) -> Vec<BoundedWidget<E>> {
    let lazouts = i.iter()
        .map(|l| 
            c.widgets()
                .get(l.borrow())
                .expect("Lost Widget")
                .lazout()
        )
        .collect::<Vec<_>>();

    match o {
        Orientation::Horizontal() => {
            calc_dirs(lazouts, o, b.0);
        }
        Orientation::Vertical() => {
            unimplemented!()
        }
    }
}

///calculate the sizes of dir
pub fn calc_dirs(i: &[Lazout], o: Orientation, dest: u32) -> Vec<(u32,u32)> {
    let sum = sum(i,o);
    
    let mut out = Vec::with_capacity(i.len());
    let mut off = 0;

    for l in i {
        let rel = l.pressure.max(0.0) / sum.pressure;
        let expand = (l.max-l.optimal) as f32 * rel;
        //let out = l.optimal + l.expand.
    }
    unimplemented!()
}