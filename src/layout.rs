use usbd_human_interface_device::page::Keyboard::*;
/// Define the layout here via the create_matrix macro.
/// Layout is defined as a tri-dimentional array: 
/// array of rows -> array of keys -> array of keymaps by layer
///
/// All array elements must be present, even empty ones. For those use ErrorUndefine. 
// Currently Fn key has to be the same across all layers
// TODO: Figure out macros to remove this ErrorUndefine madness (っ˘̩╭╮˘̩)っ 

#[rustfmt::skip]
const LAYOUT: crate::matrix::Layer = crate::create_matrix!(
[[Grave,ErrorUndefine],[Keyboard1,F1],[Keyboard2,F2],[Keyboard3,F3],[Keyboard4,F4],[Keyboard5,F5],
    [Keyboard6,F6],[Keyboard7,F7],[Keyboard8, F8],[Keyboard9,F9],[Keyboard0,F10],[Minus,F11]],
[[Tab, ErrorUndefine],[Q,ErrorUndefine],[W,ErrorUndefine],[E,ErrorUndefine],[R,ErrorUndefine],[T,ErrorUndefine],
    [Y,ErrorUndefine],[U,ErrorUndefine],[I,ErrorUndefine],[O,ErrorUndefine],[P,ErrorUndefine],[Equal,F12]],
[[Escape,ErrorUndefine],[A,ErrorUndefine],[S,ErrorUndefine],[D,ErrorUndefine],[F,ErrorUndefine],[G,ErrorUndefine],
        [H,ErrorUndefine],[J,ErrorUndefine],[K,ErrorUndefine],[L,ErrorUndefine],[Semicolon,ErrorUndefine],[Apostrophe,ErrorUndefine]],
[[CapsLock,ErrorUndefine],[Z,ErrorUndefine],[X,ErrorUndefine],[C,ErrorUndefine],[V,ErrorUndefine],[B,ErrorUndefine],
        [N,ErrorUndefine],[M,ErrorUndefine],[Comma,ErrorUndefine],[Dot,ErrorUndefine],[ForwardSlash,ErrorUndefine],[Backslash,ErrorUndefine]],
[[PageDown, End],[PageUp,Home],[LeftShift,ErrorUndefine],[Space,ErrorUndefine],[LeftControl,ErrorUndefine],[F24,ErrorUndefine],
        [RightGUI,ErrorUndefine],[RightControl,ErrorUndefine],[ReturnEnter,ErrorUndefine],[RightShift,ErrorUndefine],[LeftBrace,ErrorUndefine],[RightBrace,ErrorUndefine]],
[[ErrorUndefine,ErrorUndefine],[ErrorUndefine,ErrorUndefine],[ErrorUndefine,ErrorUndefine],[ErrorUndefine,ErrorUndefine],[LeftAlt,ErrorUndefine],[DeleteBackspace,ErrorUndefine],
        [DeleteForward,ErrorUndefine],[RightAlt,ErrorUndefine],[ErrorUndefine,ErrorUndefine],[ErrorUndefine,ErrorUndefine],[ErrorUndefine,ErrorUndefine],[ErrorUndefine,ErrorUndefine]]
);
