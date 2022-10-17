use std::fmt::Debug;

use crate::aliases::EEKey;
use crate::env::Env;
use crate::newpath::PathResolvusDyn;
use crate::util::bounds::Offset;

pub trait PressedKey<E> where E: Env {
    fn key(&self) -> EEKey<E>;
    /// the widget at which the keypress started
    fn widget(&self) -> &(dyn PathResolvusDyn<E>+'_);
    /// the timestamp at which the keypress started
    fn ts(&self) -> u64;
    fn cursor(&self) -> Option<Offset>;
}

pub trait Key: Clone + PartialEq + Debug + for<'a> PartialEq<MatchKeyCode<'a>> + for<'a> PartialEq<MatchScanCode<'a>> {
    type Origin;
    
    fn origin(&self) -> Self::Origin;
}

/// KeyCode (virtual)
/// This is not a definite descriptor, Multiple Redudant Variants for same key, so it's only for matching with Backend Keys TODO encode in name
#[derive(Clone,Copy,PartialEq)]
pub enum MatchKeyCode<'a> {
    /// e.g. Letters, Numbers, \n, \backspace, slash = divide, ...
    CharCaseSensitive(&'a str),
    /// e.g. Letters, Numbers, \n, \backspace, slash = divide, ...
    CharCaseInsensitive(&'a str),
    /// e.g. numbers from number row, keypad, ...
    Number(usize),

    MouseLeft,
    MouseRight,
    MouseCenter,
    
    KbdBackspace,
    KbdTab,
    KbdReturn,
    KbdEscape,
    KbdSpace,
    KbdExclaim,
    KbdQuotedbl,
    KbdHash,
    KbdDollar,
    KbdPercent,
    KbdAmpersand,
    KbdQuote,
    KbdLeftParen,
    KbdRightParen,
    KbdAsterisk,
    KbdPlus,
    KbdComma,
    KbdMinus,
    KbdPeriod,
    KbdSlash,
    KbdNum0,
    KbdNum1,
    KbdNum2,
    KbdNum3,
    KbdNum4,
    KbdNum5,
    KbdNum6,
    KbdNum7,
    KbdNum8,
    KbdNum9,
    KbdColon,
    KbdSemicolon,
    KbdLess,
    KbdEquals,
    KbdGreater,
    KbdQuestion,
    KbdAt,
    KbdLeftBracket,
    KbdRightBracket,
    KbdBackslash,
    KbdCaret,
    KbdUnderscore,
    KbdBackquote,
    KbdA,
    KbdB,
    KbdC,
    KbdD,
    KbdE,
    KbdF,
    KbdG,
    KbdH,
    KbdI,
    KbdJ,
    KbdK,
    KbdL,
    KbdM,
    KbdN,
    KbdO,
    KbdP,
    KbdQ,
    KbdR,
    KbdS,
    KbdT,
    KbdU,
    KbdV,
    KbdW,
    KbdX,
    KbdY,
    KbdZ,
    KbdDelete,
    KbdCapsLock,
    KbdF1,
    KbdF2,
    KbdF3,
    KbdF4,
    KbdF5,
    KbdF6,
    KbdF7,
    KbdF8,
    KbdF9,
    KbdF10,
    KbdF11,
    KbdF12,
    KbdPrintScreen,
    KbdScrollLock,
    KbdPause,
    KbdInsert,
    KbdHome,
    KbdPageUp,
    KbdEnd,
    KbdPageDown,
    KbdRight,
    KbdLeft,
    KbdDown,
    KbdUp,
    KbdNumLock,
    KpDivide,
    KpMultiply,
    KpMinus,
    KpPlus,
    KpEnter,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    Kp0,
    KpPeriod,
    KbdApplication,
    KbdPower,
    KpEquals,
    KbdF13,
    KbdF14,
    KbdF15,
    KbdF16,
    KbdF17,
    KbdF18,
    KbdF19,
    KbdF20,
    KbdF21,
    KbdF22,
    KbdF23,
    KbdF24,
    KbdExecute,
    KbdHelp,
    KbdMenu,
    KbdSelect,
    KbdStop,
    KbdAgain,
    KbdUndo,
    KbdCut,
    KbdCopy,
    KbdPaste,
    KbdFind,
    KbdMute,
    KbdVolumeUp,
    KbdVolumeDown,
    KpComma,
    KpEqualsAS400,
    KbdAltErase,
    KbdSysreq,
    KbdCancel,
    KbdClear,
    KbdPrior,
    KbdReturn2,
    KbdSeparator,
    KbdOut,
    KbdOper,
    KbdClearAgain,
    KbdCrSel,
    KbdExSel,
    Kp00,
    Kp000,
    KbdThousandsSeparator,
    KbdDecimalSeparator,
    KbdCurrencyUnit,
    KbdCurrencySubUnit,
    KpLeftParen,
    KpRightParen,
    KpLeftBrace,
    KpRightBrace,
    KpTab,
    KpBackspace,
    KpA,
    KpB,
    KpC,
    KpD,
    KpE,
    KpF,
    KpXor,
    KpPower,
    KpPercent,
    KpLess,
    KpGreater,
    KpAmpersand,
    KpDblAmpersand,
    KpVerticalBar,
    KpDblVerticalBar,
    KpColon,
    KpHash,
    KpSpace,
    KpAt,
    KpExclam,
    KpMemStore,
    KpMemRecall,
    KpMemClear,
    KpMemAdd,
    KpMemSubtract,
    KpMemMultiply,
    KpMemDivide,
    KpPlusMinus,
    KpClear,
    KpClearEntry,
    KpBinary,
    KpOctal,
    KpDecimal,
    KpHexadecimal,
    /// LCtrl OR RCtrl
    KbdCtrl,
    /// LShift OR RShift
    KbdShift,
    /// LAlt OR RAlt
    KbdAlt,
    /// LGui OR RGui
    KbdGui,
    KbdLCtrl,
    KbdLShift,
    KbdLAlt,
    KbdLGui,
    KbdRCtrl,
    KbdRShift,
    KbdRAlt,
    KbdRGui,
    KbdAltGr,
    KbdMode,
    KbdAudioNext,
    KbdAudioPrev,
    KbdAudioStop,
    KbdAudioPlay,
    KbdAudioMute,
    KbdMediaSelect,
    KbdWww,
    KbdMail,
    KbdCalculator,
    KbdComputer,
    KbdAcSearch,
    KbdAcHome,
    KbdAcBack,
    KbdAcForward,
    KbdAcStop,
    KbdAcRefresh,
    KbdAcBookmarks,
    KbdBrightnessDown,
    KbdBrightnessUp,
    KbdDisplaySwitch,
    KbdKbdIllumToggle,
    KbdKbdIllumDown,
    KbdKbdIllumUp,
    KbdEject,
    KbdSleep,
}

/// ScanCode (physical)
/// This is not a definite descriptor, Multiple Redudant Variants for same key, so it's only for matching with Backend Keys TODO encode in name
#[derive(Clone,Copy,PartialEq)]
pub enum MatchScanCode<'a> {
    KbdA,
    KbdB,
    KbdC,
    KbdD,
    KbdE,
    KbdF,
    KbdG,
    KbdH,
    KbdI,
    KbdJ,
    KbdK,
    KbdL,
    KbdM,
    KbdN,
    KbdO,
    KbdP,
    KbdQ,
    KbdR,
    KbdS,
    KbdT,
    KbdU,
    KbdV,
    KbdW,
    KbdX,
    KbdY,
    KbdZ,
    KbdNum1,
    KbdNum2,
    KbdNum3,
    KbdNum4,
    KbdNum5,
    KbdNum6,
    KbdNum7,
    KbdNum8,
    KbdNum9,
    KbdNum0,
    KbdReturn,
    KbdEscape,
    KbdBackspace,
    KbdTab,
    KbdSpace,
    KbdMinus,
    KbdEquals,
    KbdLeftBracket,
    KbdRightBracket,
    KbdBackslash,
    KbdNonUsHash,
    KbdSemicolon,
    KbdApostrophe,
    KbdGrave,
    KbdComma,
    KbdPeriod,
    KbdSlash,
    KbdCapsLock,
    KbdF1,
    KbdF2,
    KbdF3,
    KbdF4,
    KbdF5,
    KbdF6,
    KbdF7,
    KbdF8,
    KbdF9,
    KbdF10,
    KbdF11,
    KbdF12,
    KbdPrintScreen,
    KbdScrollLock,
    KbdPause,
    KbdInsert,
    KbdHome,
    KbdPageUp,
    KbdDelete,
    KbdEnd,
    KbdPageDown,
    KbdRight,
    KbdLeft,
    KbdDown,
    KbdUp,
    KbdNumLock,
    KpDivide,
    KpMultiply,
    KpMinus,
    KpPlus,
    KpEnter,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    Kp0,
    KpPeriod,
    KbdNonUsBackslash,
    KbdApplication,
    KbdPower,
    KpEquals,
    KbdF13,
    KbdF14,
    KbdF15,
    KbdF16,
    KbdF17,
    KbdF18,
    KbdF19,
    KbdF20,
    KbdF21,
    KbdF22,
    KbdF23,
    KbdF24,
    KbdExecute,
    KbdHelp,
    KbdMenu,
    KbdSelect,
    KbdStop,
    KbdAgain,
    KbdUndo,
    KbdCut,
    KbdCopy,
    KbdPaste,
    KbdFind,
    KbdMute,
    KbdVolumeUp,
    KbdVolumeDown,
    KpComma,
    KpEqualsAS400,
    KbdInternational1,
    KbdInternational2,
    KbdInternational3,
    KbdInternational4,
    KbdInternational5,
    KbdInternational6,
    KbdInternational7,
    KbdInternational8,
    KbdInternational9,
    KbdLang1,
    KbdLang2,
    KbdLang3,
    KbdLang4,
    KbdLang5,
    KbdLang6,
    KbdLang7,
    KbdLang8,
    KbdLang9,
    KbdAltErase,
    KbdSysReq,
    KbdCancel,
    KbdClear,
    KbdPrior,
    KbdReturn2,
    KbdSeparator,
    KbdOut,
    KbdOper,
    KbdClearAgain,
    KbdCrSel,
    KbdExSel,
    Kp00,
    Kp000,
    KbdThousandsSeparator,
    KbdDecimalSeparator,
    KbdCurrencyUnit,
    KbdCurrencySubUnit,
    KpLeftParen,
    KpRightParen,
    KpLeftBrace,
    KpRightBrace,
    KpTab,
    KpBackspace,
    KpA,
    KpB,
    KpC,
    KpD,
    KpE,
    KpF,
    KpXor,
    KpPower,
    KpPercent,
    KpLess,
    KpGreater,
    KpAmpersand,
    KpDblAmpersand,
    KpVerticalBar,
    KpDblVerticalBar,
    KpColon,
    KpHash,
    KpSpace,
    KpAt,
    KpExclam,
    KpMemStore,
    KpMemRecall,
    KpMemClear,
    KpMemAdd,
    KpMemSubtract,
    KpMemMultiply,
    KpMemDivide,
    KpPlusMinus,
    KpClear,
    KpClearEntry,
    KpBinary,
    KpOctal,
    KpDecimal,
    KpHexadecimal,
    KbdLCtrl,
    KbdLShift,
    KbdLAlt,
    KbdLGui,
    KbdRCtrl,
    KbdRShift,
    KbdRAlt,
    KbdRGui,
    KbdMode,
    KbdAudioNext,
    KbdAudioPrev,
    KbdAudioStop,
    KbdAudioPlay,
    KbdAudioMute,
    KbdMediaSelect,
    KbdWww,
    KbdMail,
    KbdCalculator,
    KbdComputer,
    KbdAcSearch,
    KbdAcHome,
    KbdAcBack,
    KbdAcForward,
    KbdAcStop,
    KbdAcRefresh,
    KbdAcBookmarks,
    KbdBrightnessDown,
    KbdBrightnessUp,
    KbdDisplaySwitch,
    KbdKbdIllumToggle,
    KbdKbdIllumDown,
    KbdKbdIllumUp,
    KbdEject,
    KbdSleep,
    KbdApp1,
    KbdApp2,
    KbdNum,

    Char(&'a str),
}

/// Key combinations implemented by guion
pub enum ExtCode {
    CtrlA,
    CtrlB,
    CtrlC,
    CtrlD,
    CtrlE,
    CtrlF,
    CtrlG,
    CtrlH,
    CtrlI,
    CtrlJ,
    CtrlK,
    CtrlL,
    CtrlM,
    CtrlN,
    CtrlO,
    CtrlP,
    CtrlQ,
    CtrlR,
    CtrlS,
    CtrlT,
    CtrlU,
    CtrlV,
    CtrlW,
    CtrlX,
    CtrlY,
    CtrlZ,

    ShiftA,
    ShiftB,
    ShiftC,
    ShiftD,
    ShiftE,
    ShiftF,
    ShiftG,
    ShiftH,
    ShiftI,
    ShiftJ,
    ShiftK,
    ShiftL,
    ShiftM,
    ShiftN,
    ShiftO,
    ShiftP,
    ShiftQ,
    ShiftR,
    ShiftS,
    ShiftT,
    ShiftU,
    ShiftV,
    ShiftW,
    ShiftX,
    ShiftY,
    ShiftZ,

    AltA,
    AltB,
    AltC,
    AltD,
    AltE,
    AltF,
    AltG,
    AltH,
    AltI,
    AltJ,
    AltK,
    AltL,
    AltM,
    AltN,
    AltO,
    AltP,
    AltQ,
    AltR,
    AltS,
    AltT,
    AltU,
    AltV,
    AltW,
    AltX,
    AltY,
    AltZ,

    CtrlShiftA,
    CtrlShiftB,
    CtrlShiftC,
    CtrlShiftD,
    CtrlShiftE,
    CtrlShiftF,
    CtrlShiftG,
    CtrlShiftH,
    CtrlShiftI,
    CtrlShiftJ,
    CtrlShiftK,
    CtrlShiftL,
    CtrlShiftM,
    CtrlShiftN,
    CtrlShiftO,
    CtrlShiftP,
    CtrlShiftQ,
    CtrlShiftR,
    CtrlShiftS,
    CtrlShiftT,
    CtrlShiftU,
    CtrlShiftV,
    CtrlShiftW,
    CtrlShiftX,
    CtrlShiftY,
    CtrlShiftZ,

    CtrlAltA,
    CtrlAltB,
    CtrlAltC,
    CtrlAltD,
    CtrlAltE,
    CtrlAltF,
    CtrlAltG,
    CtrlAltH,
    CtrlAltI,
    CtrlAltJ,
    CtrlAltK,
    CtrlAltL,
    CtrlAltM,
    CtrlAltN,
    CtrlAltO,
    CtrlAltP,
    CtrlAltQ,
    CtrlAltR,
    CtrlAltS,
    CtrlAltT,
    CtrlAltU,
    CtrlAltV,
    CtrlAltW,
    CtrlAltX,
    CtrlAltY,
    CtrlAltZ,

    CtrlShiftAltA,
    CtrlShiftAltB,
    CtrlShiftAltC,
    CtrlShiftAltD,
    CtrlShiftAltE,
    CtrlShiftAltF,
    CtrlShiftAltG,
    CtrlShiftAltH,
    CtrlShiftAltI,
    CtrlShiftAltJ,
    CtrlShiftAltK,
    CtrlShiftAltL,
    CtrlShiftAltM,
    CtrlShiftAltN,
    CtrlShiftAltO,
    CtrlShiftAltP,
    CtrlShiftAltQ,
    CtrlShiftAltR,
    CtrlShiftAltS,
    CtrlShiftAltT,
    CtrlShiftAltU,
    CtrlShiftAltV,
    CtrlShiftAltW,
    CtrlShiftAltX,
    CtrlShiftAltY,
    CtrlShiftAltZ,
}
