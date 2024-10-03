use crate::key::KeyAction::{KeyCode, LayerShiftHold, NoAction};
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
[[KeyCode(Grave),NoAction],[KeyCode(Keyboard1),KeyCode(F1)],[KeyCode(Keyboard2),KeyCode(F2)],[KeyCode(Keyboard3),KeyCode(F3)],[KeyCode(Keyboard4),KeyCode(F4)],[KeyCode(Keyboard5),KeyCode(F5)],
    [KeyCode(Keyboard6),KeyCode(F6)],[KeyCode(Keyboard7),KeyCode(F7)],[KeyCode(Keyboard8),KeyCode(F8)],[KeyCode(Keyboard9),KeyCode(F9)],[KeyCode(Keyboard0),KeyCode(F10)],[KeyCode(Minus),KeyCode(F11)]],
[[KeyCode(Tab), NoAction],[KeyCode(Q),NoAction],[KeyCode(W),NoAction],[KeyCode(E),NoAction],[KeyCode(R),NoAction],[KeyCode(T),NoAction],
    [KeyCode(Y),NoAction],[KeyCode(U),NoAction],[KeyCode(I),KeyCode(UpArrow)],[KeyCode(O),NoAction],[KeyCode(P),NoAction],[KeyCode(Equal),KeyCode(F12)]],
[[KeyCode(Escape),NoAction],[KeyCode(A),NoAction],[KeyCode(S),NoAction],[KeyCode(D),NoAction],[KeyCode(F),NoAction],[KeyCode(G),NoAction],
    [KeyCode(H),NoAction],[KeyCode(J),KeyCode(LeftArrow)],[KeyCode(K),KeyCode(DownArrow)],[KeyCode(L),KeyCode(RightArrow)],[KeyCode(Semicolon),NoAction],[KeyCode(Apostrophe),NoAction]],
[[KeyCode(CapsLock),NoAction],[KeyCode(Z),NoAction],[KeyCode(X),NoAction],[KeyCode(C),NoAction],[KeyCode(V),NoAction],[KeyCode(B),NoAction],
    [KeyCode(N),NoAction],[KeyCode(M),NoAction],[KeyCode(Comma),NoAction],[KeyCode(Dot),NoAction],[KeyCode(ForwardSlash),NoAction],[KeyCode(Backslash),NoAction]],
[[KeyCode(PageDown),KeyCode(End)],[KeyCode(PageUp),KeyCode(Home)],[KeyCode(LeftShift),KeyCode(LeftShift)],[KeyCode(Space),NoAction],[KeyCode(LeftControl),KeyCode(LeftControl)],[LayerShiftHold,LayerShiftHold],
    [KeyCode(RightGUI),KeyCode(RightGUI)],[KeyCode(RightControl),KeyCode(RightControl)],[KeyCode(ReturnEnter),NoAction],[KeyCode(RightShift),KeyCode(RightShift)],[KeyCode(LeftBrace),NoAction],[KeyCode(RightBrace),NoAction]],
[[NoAction,NoAction],[NoAction,NoAction],[NoAction,NoAction],[NoAction,NoAction],[KeyCode(DeleteBackspace),NoAction],[KeyCode(LeftAlt),KeyCode(LeftAlt)],
    [KeyCode(RightAlt),NoAction],[KeyCode(DeleteForward),NoAction],[NoAction,NoAction],[NoAction,NoAction],[NoAction,NoAction],[NoAction,NoAction]]
);
