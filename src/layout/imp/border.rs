use qwutils::imp::option::OptionExt;
use super::*;

//TODO rework border to trait or fn for constrait trait to directly add from style
qwutils::opion!(add(StdGonstraints,Border) |s,r| {
    let r = r.border_effective();
    s.x += r.w;
    s.y += r.h;
});

qwutils::opion!(add(StdGonstraintAxis,move u32) |s,r| {
    s.min += r;
    s.preferred += r;
    s.max.add_to(r);
});
