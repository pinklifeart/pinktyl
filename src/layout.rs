use crate::key::KeyEvent::{KeyCode, LayerShiftHold, None};
use usbd_human_interface_device::page::Keyboard::*;
/// Define the layout here via the create_matrix macro.
/// Layout is defined as a tri-dimentional array: 
/// array of rows -> array of keys -> array of keymaps by layer
///
/// All array elements must be present, even empty ones. For those use ErrorUndefine. 
// Currently Fn key has to be the same across all layers
// TODO: Figure out macros to remove this keycode and repetition madness (っ˘̩╭╮˘̩)っ 

#[rustfmt::skip]
pub const LAYOUT: crate::matrix::Layer = crate::create_matrix!(
[[KeyCode(Grave),None],[KeyCode(Keyboard1),KeyCode(F1)],[KeyCode(Keyboard2),KeyCode(F2)],[KeyCode(Keyboard3),KeyCode(F3)],[KeyCode(Keyboard4),KeyCode(F4)],[KeyCode(Keyboard5),KeyCode(F5)],
    [KeyCode(Keyboard6),KeyCode(F6)],[KeyCode(Keyboard7),KeyCode(F7)],[KeyCode(Keyboard8),KeyCode(F8)],[KeyCode(Keyboard9),KeyCode(F9)],[KeyCode(Keyboard0),KeyCode(F10)],[KeyCode(Minus),KeyCode(F11)]],
[[KeyCode(Tab), None],[KeyCode(Q),None],[KeyCode(W),None],[KeyCode(E),None],[KeyCode(R),None],[KeyCode(T),None],
    [KeyCode(Y),None],[KeyCode(U),None],[KeyCode(I),None],[KeyCode(O),None],[KeyCode(P),None],[KeyCode(Equal),KeyCode(F12)]],
[[KeyCode(Escape),None],[KeyCode(A),None],[KeyCode(S),None],[KeyCode(D),None],[KeyCode(F),None],[KeyCode(G),None],
    [KeyCode(H),None],[KeyCode(J),None],[KeyCode(K),None],[KeyCode(L),None],[KeyCode(Semicolon),None],[KeyCode(Apostrophe),None]],
[[KeyCode(CapsLock),None],[KeyCode(Z),None],[KeyCode(X),None],[KeyCode(C),None],[KeyCode(V),None],[KeyCode(B),None],
    [KeyCode(N),None],[KeyCode(M),None],[KeyCode(Comma),None],[KeyCode(Dot),None],[KeyCode(ForwardSlash),None],[KeyCode(Backslash),None]],
[[KeyCode(PageDown),KeyCode(End)],[KeyCode(PageUp),KeyCode(Home)],[KeyCode(LeftShift),None],[KeyCode(Space),None],[KeyCode(LeftControl),None],[LayerShiftHold,LayerShiftHold],
    [KeyCode(RightGUI),None],[KeyCode(RightControl),None],[KeyCode(ReturnEnter),None],[KeyCode(RightShift),None],[KeyCode(LeftBrace),None],[KeyCode(RightBrace),None]],
[[None,None],[None,None],[None,None],[None,None],[KeyCode(LeftAlt),None],[KeyCode(DeleteBackspace),None],
    [KeyCode(DeleteForward),None],[KeyCode(RightAlt),None],[None,None],[None,None],[None,None],[None,None]]
);
