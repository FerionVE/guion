pub trait Key: Clone {}
pub trait KeyCombo: Clone {}
//TODO move to widgets
///key combos of standard widgets
pub enum StdCombos {
    MouseLeft(),
    MouseRight(),
    MouseMiddle(),
    
    ///button click, takes focus
    ButtonClickActive(), //MouseLeft
    ///click button if focused
    ButtonClickPassive(), //Enter

    TextPaste(), //Ctrl+V
    TextCopy(), //Ctrl+C
    TextSelectAll(), //Ctrl+A

    WindowClose(),


}