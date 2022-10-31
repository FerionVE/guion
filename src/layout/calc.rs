use crate::layout::GonstraintAxis;
use crate::util::bounds::{Dims, Bounds};

use super::size::StdGonstraintAxis;
use super::{Orientation, Gonstraints};

pub fn calc_bounds<S,SK>(outer: &Dims, child_constraints: &[SK], mut constraint_key: impl FnMut(&SK) -> S, o: Orientation) -> Vec<Bounds> where S: Gonstraints {
    if child_constraints.is_empty() {return vec![];}

    let pars: Vec<StdGonstraintAxis> = child_constraints.iter()
        .map(|x| (constraint_key)(x).into().par(o) )
        .collect();

    calc_bounds2(outer, &pars, o)
}

pub fn calc_bounds2(outer: &Dims, parallel_child_axis: &[StdGonstraintAxis], o: Orientation) -> Vec<Bounds> {
    if parallel_child_axis.is_empty() {return vec![];}

    let width = outer.par(o);

    let snap = {
        let axis_sum: StdGonstraintAxis = parallel_child_axis.iter().fold(StdGonstraintAxis::zero(), |acc,v| acc+v );

        if axis_sum.min > width {
            0
        } else if axis_sum.preferred > width {
            1
        } else {
            2
        }
    };

    fn lower(axis: &StdGonstraintAxis, snap: usize) -> u32 {
        if snap > 1 {
            axis.preferred
        } else if snap == 1 {
            axis.min
        } else {
            0
        }
    }

    fn press_part(my_pressure: f32, free: u32, pressure_sum: f32) -> u32 {
        ((free as f32)*my_pressure/pressure_sum) as u32
    }

    let mut v: Vec<Option<u32>> = vec![None;parallel_child_axis.len()];

    let mut allocated = 0u32;

    let mut last = false;

    loop {
        let mut repeat = false;

        let free = width-allocated;
        let mut pressure_sum = 0f32;

        for (_,axis) in v.iter().zip(parallel_child_axis.iter()).filter(|(s,_)| s.is_none() ) {
            pressure_sum += axis.pressure;
        }

        for (val,axis) in v.iter_mut().zip(parallel_child_axis.iter()).filter(|(s,_)| s.is_none() ) {
            let should = press_part(axis.pressure,free,pressure_sum);

            let lower = lower(axis,snap);

            if last {
                allocated += should;
                *val = Some(should);
            } else if should < lower {
                repeat = true;
                allocated += lower;
                *val = Some(lower);
            } else if axis.max.is_some() && should > axis.max.unwrap() {
                repeat = true;
                allocated += axis.max.unwrap();
                *val = Some(axis.max.unwrap());
            }
        }

        if last {break;}
        if !repeat {last=true;}
    }

    assert!(allocated <= width);

    let mut out_off = 0u32;

    let mut dest = Vec::with_capacity(parallel_child_axis.len());

    for v in v.iter() {
        let par = v.unwrap();
        
        let b = Bounds::from_ori(out_off as i32, 0, par, outer.unpar(o), o);

        out_off += par;

        dest.push(b);
    }

    dest
}
