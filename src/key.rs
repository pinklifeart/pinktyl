#![deny(unsafe_code)]

use usbd_human_interface_device::page::Keyboard;

pub struct Key {
    pub keycodes: [Keyboard; crate::matrix::LAYERS],
    pub is_active: bool,
    pub debounce_count: u8,
}

impl Key {
    pub fn set_active(&mut self) {
        self.is_active = true;
    }
    pub fn set_inactive(&mut self) {
        self.is_active = false;
    }
    pub fn set_debounce(&mut self, count: u8) {
        self.debounce_count = count;
    }
    pub fn tick_debounce(&mut self) {
        self.debounce_count = self.debounce_count.saturating_sub(1)
    }
    pub fn get_state(&self) -> bool {
        self.is_active
    }
}
// TODO: Fix this macro so I dont have to spam ErrorUndefine all over the place (〃＞＿＜;〃)
#[macro_export]
macro_rules! create_key {
    ([ $($key:ident),+ ]) => {
        $crate::key::Key {
            keycodes: [$($key),+],
            is_active: false,
            debounce_count: 0,
        }
    };
}
