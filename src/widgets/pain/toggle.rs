pub trait Toggle {}

pub struct TOwned {}
pub struct TRef {}
pub struct TMut {}

impl Toggle for TOwned {}
impl Toggle for TRef {}
impl Toggle for TMut {}