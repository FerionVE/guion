pub trait VariantDesc: Default + Clone + Sized + 'static {} //: Copy + Clone + Send + Sync + PartialEq + Eq + Hash;

#[derive(Clone,Default)]
pub struct VDefault;

impl VariantDesc for VDefault {}

#[derive(Clone,Default)]
pub struct VHovered;

impl VariantDesc for VHovered {}

#[derive(Clone,Default)]
pub struct VSelected;

impl VariantDesc for VSelected {}

#[derive(Clone,Default)]
pub struct VActivated;

impl VariantDesc for VActivated {}

#[derive(Clone,Default)]
pub struct VDisabled;

impl VariantDesc for VDisabled {}
