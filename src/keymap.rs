use rmk::action::KeyAction;
use rmk::{a, k, layer, mo};
pub(crate) const COL: usize = 6;
pub(crate) const ROW: usize = 10;
pub(crate) const NUM_LAYER: usize = 16;

#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(LCtrl), k!(Tab), k!(LShift), k!(Enter), k!(MO(14)), k!(CapsLock)],
            [k!(V), k!(G), k!(F), k!(R), k!(WM(Quote, LShift)), a!(No)],
            [k!(C), k!(T), k!(D), k!(E), k!(Grave), a!(No)],
            [k!(X), k!(B), k!(S), k!(W), k!(Escape), a!(No)],
            [k!(Z), k!(LeftBracket), k!(A), k!(Q), k!(Delete), a!(No)],
            [k!(LAlt), k!(Backspace), k!(MO(4)), k!(Space), k!(MO(2)), k!(MO(5))],
            [k!(M), k!(Quote), k!(J), k!(U), k!(H), a!(No)],
            [k!(Comma), k!(WM(Semicolon, LShift)), k!(K), k!(I), k!(Y), a!(No)],
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
            [k!(LAlt), k!(Backspace), k!(MO(4)), k!(Space), a!(No), a!(No)],
            [k!(Home), k!(End), k!(Left), k!(WM(Left, LCtrl)), k!(Home), a!(No)],
            [k!(PageDown), a!(No), k!(Down), k!(WM(Down, LCtrl)), a!(No), a!(No)],
            [k!(PageUp), a!(No), k!(UP), k!(WM(UP, LCtrl)), k!(Insert), a!(No)],
            [k!(End), a!(No), k!(Right), k!(WM(Right, LCtrl)), a!(No), a!(No)]
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
            [k!(Minus), k!(Kc5), k!(Kc4), k!(WM(Kc4, LShift)), k!(Slash), a!(No)],
            [k!(WM(Kc5, LShift)), k!(WM(Dot, LShift)), k!(Kc3), k!(WM(Kc3, LShift)), k!(WM(Comma, LShift)), a!(No)],
            [k!(X), a!(No), k!(Kc2), k!(WM(Kc2, LShift)), k!(Escape), a!(No)],
            [k!(Equal), k!(WM(Grave, LShift)), k!(Kc1), k!(WM(Kc1, LShift)), k!(Delete), a!(No)],
            [k!(LAlt), k!(Backspace), k!(MO(4)), k!(Space), a!(No), a!(No)],
            [k!(KpPlus), k!(WM(Minus, LShift)), k!(Kc7), k!(WM(Kc7, LShift)), k!(Kc6), a!(No)],
            [k!(Comma), k!(WM(Semicolon, LShift)), k!(Kc8), k!(KpAsterisk), k!(WM(Kc6, LShift)), a!(No)],
            [k!(Dot), k!(LGui), k!(Kc9), k!(WM(Kc9, LShift)), k!(Semicolon), a!(No)],
            [k!(WM(Slash, LShift)), a!(No), k!(Kc0), k!(WM(Kc0, LShift)), k!(RightBracket), a!(No)]
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
            [k!(WM(Kc4, LCtrl)), k!(WM(Kc5, LAlt)), k!(WM(Kc4, LAlt)), k!(WM(Kc8, LCtrl)), a!(No), a!(No)],
            [k!(WM(Kc3, LCtrl)), k!(WM(Tab, LCtrl)), k!(WM(Kc3, LAlt)), k!(WM(Kc7, LCtrl)), k!(WM(Tab, LCtrl | LShift)), a!(No)],
            [k!(WM(Kc2, LCtrl)), a!(No), k!(WM(Kc2, LAlt)), k!(WM(Kc6, LCtrl)), a!(No), a!(No)],
            [k!(WM(Kc1, LCtrl)), k!(WM(Kc9, LCtrl)), k!(WM(Kc1, LAlt)), k!(WM(Kc5, LCtrl)), a!(No), a!(No)],
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
