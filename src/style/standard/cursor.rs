#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum StdCursor {
    Default,
    Arrow,
    IBeam,
    Wait,
    Crosshair,
    WaitArrow,
    SizeNWSE,
    SizeNESW,
    SizeWE,
    SizeNS,
    SizeAll,
    No,
    Hand,
}

impl Default for StdCursor {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}
