use super::*;

pub fn calc_bounds<S: ISize>(outer: &Dims, w: &[S], o: Orientation) -> Vec<Bounds> {
    //TODO! real impl
    
    let h = outer.h / w.len() as u32;
    let mut dest = Vec::with_capacity(w.len());

    for (i,s) in w.iter().enumerate() {
        dest.push(Bounds::from_xywh(0, (outer.h*(i as u32)/(w.len() as u32)) as i32, outer.w, h));
    }

    dest
}

pub fn calc_bounds_dir(size: usize, w: &[SizeAxis]) -> Vec<(u32,u32)> {
    todo!()
}