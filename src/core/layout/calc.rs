use super::*;

pub fn calc_bounds<S: ISize>(outer: &Dims, ws: &[S], o: Orientation) -> Vec<Bounds> {
    if ws.is_empty() {return vec![];}

    let width = outer.par(o) as u32;

    let pars: Vec<SizeAxis> = ws.iter()
        .map(|x| x.as_std().par(o).clone() )
        .collect();

    let snap = {
        let axis_sum: SizeAxis = pars.iter().fold(SizeAxis::empty(), |acc,v| acc+v );

        if axis_sum.min > width {
            0
        } else if axis_sum.preferred > width {
            1
        } else {
            2
        }
    };

    fn lower(axis: &SizeAxis, snap: usize) -> u32 {
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

    let mut v: Vec<Option<u32>> = vec![None;ws.len()];

    let mut allocated = 0u32;

    let mut pressure_sum = 0f32;

    let mut repeat = true;
    let mut last = false;

    loop {
        repeat = false;

        let free = width-allocated;

        pressure_sum = 0.0;
        for (_,axis) in v.iter().zip(pars.iter()).filter(|(s,_)| s.is_none() ) {
            pressure_sum += axis.pressure;
        }

        for (val,axis) in v.iter_mut().zip(pars.iter()).filter(|(s,_)| s.is_none() ) {
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

    let mut dest = Vec::with_capacity(ws.len());

    for v in v.iter() {
        let par = v.unwrap();
        
        let b = Bounds::from_ori(out_off as i32, 0, par, outer.unpar(o), o);

        out_off += par;

        dest.push(b);
    }

    dest
}

pub fn calc_bounds_old<S: ISize>(outer: &Dims, ws: &[S], o: Orientation) -> Vec<Bounds> {
    if ws.is_empty() {return vec![];}

    let lenf = ws.len() as f32;

    let mut par = SizeAxis::empty();
    let mut unpar = SizeAxis::empty();

    let mut pressure_sum = 0.0;

    for w in ws {
        let w: Size = w.as_std();
        let p = w.par(o);
        let u = w.unpar(o);
        par += p;
        unpar &= u;
        pressure_sum += p.pressure; 
    }

    let dest_w = outer.par(o) as i32;

    let growable = dest_w - (par.min as i32);
    let growablef = growable as f32;

    let would_grow = ws.iter()
        .map(|v| {
            let v = v.as_std();
            let axis = v.par(o);

            let part_w = dest_w as f32 / lenf;

            let growabl = part_w - (axis.min as f32);

            //let budget_scaled = budget*rest/dest_w;
            growabl.max(0.0)
        })
        .collect::<Vec<_>>();

    let would_grow_sum = would_grow.iter()
        //.map(|(_,g)| g)
        .sum::<f32>();

    assert!(would_grow_sum >= growablef);

    let mut out_off = 0u32;

    let mut dest = Vec::with_capacity(ws.len());

    for (v,g) in ws.iter().zip(would_grow) {
        let v = v.as_std();
        let v = v.par(o);

        let par = (g * growablef / would_grow_sum) as u32 + v.min;
        
        let b = Bounds::from_ori(out_off as i32, 0, par, outer.unpar(o), o);

        out_off += par;

        dest.push(b);
    }

    dest
}
