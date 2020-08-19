use qwutils::imp::option::OptionExt;
use super::*;

qwutils::opion!(add(Size,Border) |s,r| {
    let r = r.border_effective();
    s.x += r.w;
    s.y += r.h;
});

qwutils::opion!(add(SizeAxis,move u32) |s,r| {
    s.min += r;
    s.preferred += r;
    s.max.add_to(r);
});
