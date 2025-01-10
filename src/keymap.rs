use rmk::action::KeyAction;
use rmk::keycode::ModifierCombination;
use rmk::{a, k, layer, mo, wm};
pub(crate) const COL: usize = 6;
pub(crate) const ROW: usize = 10;
pub(crate) const NUM_LAYER: usize = 16;

#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    let lalt = ModifierCombination::new_from(false, false, true, false, false);
    let lctrl = ModifierCombination::new_from(false, false, false, false, true);
    let lshift = ModifierCombination::new_from(false, false, false, true, false);
    [
        layer!([
            [k!(LCtrl), k!(Tab), k!(LShift), k!(Enter), mo!(14), k!(CapsLock)],
            [k!(V), k!(G), k!(F), k!(R), wm!(Quote, lshift), a!(No)],
            [k!(C), k!(T), k!(D), k!(E), k!(Grave), a!(No)],
            [k!(X), k!(B), k!(S), k!(W), k!(Escape), a!(No)],
            [k!(Z), k!(LeftBracket), k!(A), k!(Q), k!(Delete), a!(No)],
            [k!(LAlt), k!(Backspace), mo!(4), k!(Space), mo!(2), mo!(5)],
            [k!(M), k!(Quote), k!(J), k!(U), k!(H), a!(No)],
            [k!(Comma), wm!(Semicolon, lshift), k!(K), k!(I), k!(Y), a!(No)],
            [k!(Dot), k!(LGui), k!(L), k!(O), k!(N), a!(No)],
            [k!(Slash), k!(Backslash), k!(Semicolon), k!(P), k!(RightBracket), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [k!(LCtrl), k!(Tab), k!(LShift), k!(Enter), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), k!(Escape), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), k!(Delete), a!(No)],
            [k!(LAlt), k!(Backspace), mo!(4), k!(Space), a!(No), a!(No)],
            [k!(Home), k!(End), k!(Left), wm!(Left, lctrl), k!(Home), a!(No)],
            [k!(PageDown), a!(No), k!(Down), wm!(Down, lctrl), a!(No), a!(No)],
            [k!(PageUp), a!(No), k!(UP), wm!(UP, lctrl), k!(Insert), a!(No)],
            [k!(End), a!(No), k!(Right), wm!(Right, lctrl), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [k!(LCtrl), k!(Tab), k!(LShift), k!(Enter), a!(No), a!(No)],
            [k!(Minus), k!(Kc5), k!(Kc4), wm!(Kc4, lshift), k!(Slash), a!(No)],
            [wm!(Kc5, lshift), wm!(Dot, lshift), k!(Kc3), wm!(Kc3, lshift), wm!(Comma, lshift), a!(No)],
            [k!(X), a!(No), k!(Kc2), wm!(Kc2, lshift), k!(Escape), a!(No)],
            [k!(Equal), wm!(Grave, lshift), k!(Kc1), wm!(Kc1, lshift), k!(Delete), a!(No)],
            [k!(LAlt), k!(Backspace), mo!(4), k!(Space), a!(No), a!(No)],
            [k!(KpPlus), wm!(Minus, lshift), k!(Kc7), wm!(Kc7, lshift), k!(Kc6), a!(No)],
            [k!(Comma), wm!(Semicolon, lshift), k!(Kc8), k!(KpAsterisk), wm!(Kc6, lshift), a!(No)],
            [k!(Dot), k!(LGui), k!(Kc9), wm!(Kc9, lshift), k!(Semicolon), a!(No)],
            [wm!(Slash, lshift), a!(No), k!(Kc0), wm!(Kc0, lshift), k!(RightBracket), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(F14), k!(F5), k!(F4), k!(F24), k!(F15), a!(No)],
            [k!(F13), a!(No), k!(F3), k!(F23), a!(No), a!(No)],
            [k!(F12), a!(No), k!(F2), k!(F22), a!(No), a!(No)],
            [k!(F11), a!(No), k!(F1), k!(F21), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(F17), k!(F16), k!(F7), a!(No), k!(F6), a!(No)],
            [k!(F18), a!(No), k!(F8), a!(No), a!(No), a!(No)],
            [k!(F19), a!(No), k!(F9), a!(No), a!(No), a!(No)],
            [k!(F20), a!(No), k!(F10), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [wm!(Kc4, lctrl), wm!(Kc5, lalt), wm!(Kc4, lalt), wm!(Kc8, lctrl), a!(No), a!(No)],
            [wm!(Kc3, lctrl), wm!(Tab, lctrl), wm!(Kc3, lalt), wm!(Kc7, lctrl), wm!(Tab, lctrl | lshift), a!(No)],
            [wm!(Kc2, lctrl), a!(No), wm!(Kc2, lalt), wm!(Kc6, lctrl), a!(No), a!(No)],
            [wm!(Kc1, lctrl), wm!(Kc9, lctrl), wm!(Kc1, lalt), wm!(Kc5, lctrl), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), k!(User17), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), k!(MouseBtn1), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), k!(MouseBtn4), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(MouseBtn2), a!(No), k!(MouseBtn1), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(MouseBtn3), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ])
    ]
}
