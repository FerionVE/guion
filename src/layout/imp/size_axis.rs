use qwutils::OptionExt;

use crate::layout::size::StdGonstraintAxis;

qwutils::opion!(add(StdGonstraintAxis,StdGonstraintAxis) |s,r| {
    s.min += r.min;
    s.preferred += r.preferred;
    s.max.add_to_lossy(r.max);
    s.pressure = s.pressure.max(r.pressure);
});

qwutils::opion!(bitand(StdGonstraintAxis,StdGonstraintAxis) |s,r| {
    s.min = s.min.max(r.min);
    s.preferred = s.preferred.max(r.preferred);
    s.max.with_mut_if_saturating( r.max, #[inline] |s,r| *s = (*s).min(*r) );

    s.max.map(#[inline] |m| s.preferred = s.preferred.min(m) );
    s.preferred = s.preferred.max(s.min);
    s.pressure = s.pressure.max(r.pressure);
});
