use super::*;

pub fn calc_bounds<S: ISize>(outer: &Dims, ws: &[S], o: Orientation) -> Vec<Bounds> {
    if ws.is_empty() {return vec![];}

    let lenf = ws.len() as f32;

    let mut par = SizeAxis::empty();
    let mut unpar = SizeAxis::empty();

    for w in ws {
        let w: Size = w.as_std();
        par += w.par(o);
        unpar &= w.unpar(o);
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
