pub struct StandardConf {
    pub drag_trigger_distance: u32,
    pub fwd_tabulate_key: KeyCombo,
    pub rev_tabulate_key: KeyCombo,
}

pub type KeyCombo = Vec<u32>;