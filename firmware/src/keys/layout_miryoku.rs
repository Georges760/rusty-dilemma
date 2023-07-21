use keyberon::action::{k, l, Action, HoldTapAction};
use keyberon::key_code::KeyCode;

use super::chord::Chorder;

pub const COLS_PER_SIDE: usize = 5;
pub const COLS: usize = COLS_PER_SIDE * 2;
pub const ROWS: usize = 3;
pub const N_LAYERS: usize = 8;

#[derive(Clone, Copy)]
pub enum CustomEvent {
    MouseLeft,
    MouseRight,
    MouseScroll,
    MouseMiddle,
}

pub type Layers = keyberon::layout::Layers<COLS, { ROWS + 1 }, N_LAYERS, CustomEvent>;
pub type Layout = keyberon::layout::Layout<COLS, { ROWS + 1 }, N_LAYERS, CustomEvent>;

const HOLD_TIMEOUT: u16 = 400;

macro_rules! hold_tap {
    ($hold:expr, $tap:expr) => {
        Action::HoldTap(&HoldTapAction {
            timeout: HOLD_TIMEOUT,
            hold: k($hold),
            tap: $tap,
            config: keyberon::action::HoldTapConfig::PermissiveHold,
            tap_hold_interval: 200,
        })
    };
}

macro_rules! h_win {
    ($tap:expr) => {
        hold_tap!(KeyCode::LGui, $tap)
    };
}

macro_rules! h_lctrl {
    ($tap:expr) => {
        hold_tap!(KeyCode::LCtrl, $tap)
    };
}

macro_rules! h_lshift {
    ($tap:expr) => {
        hold_tap!(KeyCode::LShift, $tap)
    };
}

macro_rules! h_lalt {
    ($tap:expr) => {
        hold_tap!(KeyCode::LAlt, $tap)
    };
}

const LNAV_SPC: Action<CustomEvent> = Action::HoldTap(&HoldTapAction {
    timeout: HOLD_TIMEOUT,
    hold: l(1),
    tap: k(KeyCode::Space),
    config: keyberon::action::HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});

const LMOUSE_TAB: Action<CustomEvent> = Action::HoldTap(&HoldTapAction {
    timeout: HOLD_TIMEOUT,
    hold: l(2),
    tap: k(KeyCode::Tab),
    config: keyberon::action::HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});

const LBUT_Z: Action<CustomEvent> = Action::HoldTap(&HoldTapAction {
    timeout: HOLD_TIMEOUT,
    hold: l(3),
    tap: k(KeyCode::Z),
    config: keyberon::action::HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});

const LBUT_SLASH: Action<CustomEvent> = Action::HoldTap(&HoldTapAction {
    timeout: HOLD_TIMEOUT,
    hold: l(3),
    tap: k(KeyCode::Slash),
    config: keyberon::action::HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});

const LMEDIA_ESC: Action<CustomEvent> = Action::HoldTap(&HoldTapAction {
    timeout: HOLD_TIMEOUT,
    hold: l(4),
    tap: k(KeyCode::Escape),
    config: keyberon::action::HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});

const LNUM_BSPC: Action<CustomEvent> = Action::HoldTap(&HoldTapAction {
    timeout: HOLD_TIMEOUT,
    hold: l(5),
    tap: k(KeyCode::BSpace),
    config: keyberon::action::HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});

const LSYMB_ENTER: Action<CustomEvent> = Action::HoldTap(&HoldTapAction {
    timeout: HOLD_TIMEOUT,
    hold: l(6),
    tap: k(KeyCode::Enter),
    config: keyberon::action::HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});

const LFUN_DEL: Action<CustomEvent> = Action::HoldTap(&HoldTapAction {
    timeout: HOLD_TIMEOUT,
    hold: l(7),
    tap: k(KeyCode::Delete),
    config: keyberon::action::HoldTapConfig::PermissiveHold,
    tap_hold_interval: 0,
});

type A = Action<CustomEvent>;

pub fn chorder() -> Chorder {
    dilemma_macros::chords!()
}

const SUPER_A: A = h_win!(k(KeyCode::A));
const ALT_R: A = h_lalt!(k(KeyCode::R));
const CTRL_S: A = h_lctrl!(k(KeyCode::S));
const SHIFT_T: A = h_lshift!(k(KeyCode::T));
const SHIFT_N: A = h_lshift!(k(KeyCode::N));
const CTRL_E: A = h_lctrl!(k(KeyCode::E));
const ALT_I: A = h_lalt!(k(KeyCode::I));
const SUPER_O: A = h_win!(k(KeyCode::O));

// row 4 is weird
//
// x x 2 0 1  -  8 9 7 x x
#[rustfmt::skip]
pub static LAYERS: Layers  = keyberon::layout::layout! {
    // Base
    // with Colemak Mod-DH alphas
    {
        [Q W F P B J L U Y '\''],
        [{SUPER_A} {ALT_R} {CTRL_S} {SHIFT_T} G M {SHIFT_N} {CTRL_E} {ALT_I} {SUPER_O}],
        [{LBUT_Z} X C D V K H , . {LBUT_SLASH}],
        [{LNAV_SPC} {LMOUSE_TAB} {LMEDIA_ESC} n n n n {LFUN_DEL} {LSYMB_ENTER} {LNUM_BSPC}],
    }
    // Nav
    {
        [n n n n n Again Paste Copy Cut Undo],
        [LGui LAlt LCtrl LShift n CapsLock Left Down Up Right],
        [n n n n n Insert Home PgDown PgUp End],
        [n n n n n n n Delete Enter BSpace],
    }
    // Mouse
    {
        [n n n n n Again Paste Copy Cut Undo],
        [LGui LAlt LCtrl LShift n n n n n n], // TODO mouse movement
        [n n n n n n n n n n], // TODO wheel
        [n n n n n n n {Action::Custom(CustomEvent::MouseMiddle)} {Action::Custom(CustomEvent::MouseRight)} {Action::Custom(CustomEvent::MouseLeft)}],
    }
    // Button
    {
        [Undo Cut Copy Paste Again Again Paste Copy Cut Undo],
        [LGui LAlt LCtrl LShift n n RShift RCtrl RAlt RGui],
        [Undo Cut Copy Paste Again Again Paste Copy Cut Undo],
        [{Action::Custom(CustomEvent::MouseLeft)} {Action::Custom(CustomEvent::MouseRight)} {Action::Custom(CustomEvent::MouseMiddle)} n n n n {Action::Custom(CustomEvent::MouseMiddle)} {Action::Custom(CustomEvent::MouseRight)} {Action::Custom(CustomEvent::MouseLeft)}],
    }
    // Media
    {
        [n n n n n n n n n n], // TODO RGB
        [LGui LAlt LCtrl LShift n n MediaPreviousSong MediaVolDown MediaVolUp MediaNextSong], // TODO External Power Toggle
        [n n n n n n n n n n], // TODO BlueTooth
        [n n n n n n n MediaStop MediaPlayPause MediaMute],
    }
    // Num
    {
        ['[' 7 8 9 ']' n n n n n],
        [; 4 5 6 = n RShift RCtrl RAlt RGui],
        ['`' 1 2 3 '\\' n n n n n],
        [0 - . n n n n n n n],
    }
    // Sym
    {
        ['{' & * '(' '}' n n n n n],
        [: $ % ^ + n RShift RCtrl RAlt RGui],
        [~ ! @ # | n n n n n],
        [')' '_' '(' n n n n n n n],
    }
    // Fun
    {
        [F12 F7 F8 F9 PScreen n n n n n],
        [F11 F4 F5 F6 ScrollLock n RShift RCtrl RAlt RGui],
        [F10 F1 F2 F3 Pause n n n n n],
        [Space Tab Application n n n n n n n],
    }
};
