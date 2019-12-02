use crate::core::util::bounds::Bounds;
use crate::core::util::bounded_widget::BoundedWidget;
use crate::core::env::*;
use std::borrow::Borrow;
use crate::core::widget::Widget;
use super::*;
use qwutils::*;

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
            d.max.add_to_lossy(s.max);
            d.optimal.add_to_lossy(s.optimal);
            d.pressure += s.pressure.max(0.0);
            d
        }
    )
    unimplemented!() //TODO WRONG
}

pub fn calc<W: Borrow<E::WidgetID>, E: Env>(i: &[W], b: (u32,u32), o: Orientation, c: &E::Ctx) -> Vec<BoundedWidget<E>> {
    let lazouts = i.iter()
        .map(|l| 
            (
                l.borrow(),
                c.widgets()
                    .get(l.borrow())
                    .expect("Lost Widget")
                    .lazout()
            )
        )
        .collect::<Vec<_>>();

    let dd = lazouts.iter().map(|e| e.1[o].clone() ).collect::<Vec<_>>();

    match o {
        Orientation::Horizontal() => {
            let s = calc_dirs(&dd[..], b.0);
        
            s.into_iter().zip(lazouts.into_iter())
            .map(|(pos,(id,laz))| {
                BoundedWidget {
                    id: id.clone(),
                    bounds: Bounds::from_xywh(pos.0, 0, pos.1, b.1)
                }
            }).collect()
        }
        Orientation::Vertical() => {
            let s = calc_dirs(&dd[..], b.1);
        
            s.into_iter().zip(lazouts.into_iter())
            .map(|(pos,(id,laz))| {
                BoundedWidget {
                    id: id.clone(),
                    bounds: Bounds::from_xywh(0, pos.0, b.0, pos.1)
                }
            }).collect()
        }
    }
}

///calculate the sizes of dir
pub fn calc_dirs(i: &[LazoutDir], dest: u32) -> Vec<(u32,u32)> {
    let sum = sum(i);
    
    let mut out = Vec::with_capacity(i.len());
    let mut off = 0;

    for l in i {
        let rel = l.pressure.max(0.0) / sum.pressure;
        let expand = (l.max-l.optimal) as f32 * rel;
        //let out = l.optimal + l.expand.
    }
    unimplemented!();
    out
}