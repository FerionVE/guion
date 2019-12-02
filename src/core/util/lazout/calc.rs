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
            d.pressure += s.pressure;
            d
        }
    )
}

///calculate the sizes of dir
pub fn calc_dirs(i: &[LazoutDir]) -> Vec<(u32,u32)> {
    let sum = sum(i);
    unimplemented!()
}