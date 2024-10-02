#![deny(unsafe_code)]

use defmt::Format;
use usbd_human_interface_device::page::Keyboard;

#[derive(Debug, Format, Clone)]
pub enum KeyEvent {
    LayerShiftHold,
    KeyCode(Keyboard),
    None,
}
impl From<Keyboard> for KeyEvent {
    fn from(value: Keyboard) -> Self {
        Self::KeyCode(value)
    }
}

impl From<KeyEvent> for Keyboard {
    fn from(value: KeyEvent) -> Self {
        match value {
            KeyEvent::KeyCode(k) => k,
            KeyEvent::LayerShiftHold => Keyboard::NoEventIndicated,
            KeyEvent::None => Keyboard::ErrorUndefine,
        }
    }
}

pub struct Key {
    pub keycodes: [KeyEvent; crate::matrix::LAYERS],
    pub is_active: bool,
    pub debounce_count: u8,
}

#[derive(Format)]
pub enum StateChange {
    SetActive,
    SetInactive,
    DebounceTick,
    LayerUp,
    LayerDown,
}

impl Key {
    pub fn sync_state(
        &mut self,
        to_active: bool,
        debounce_limit: u8,
        active_layer: usize,
    ) -> Option<StateChange> {
        self.tick_debounce();
        if self.debounce_count > 0 {
            Some(StateChange::DebounceTick)
        } else if to_active {
            if self.is_active {
                None
            } else {
                self.set_active();
                self.set_debounce(debounce_limit);
                match self.keycodes[active_layer] {
                    KeyEvent::LayerShiftHold => Some(StateChange::LayerUp),
                    _ => Some(StateChange::SetActive),
                }
            }
        } else if self.is_active {
            self.set_inactive();
            self.set_debounce(debounce_limit);
            match self.keycodes[active_layer] {
                KeyEvent::LayerShiftHold => Some(StateChange::LayerDown),
                _ => Some(StateChange::SetInactive),
            }
        } else {
            None
        }
    }
    #[inline(always)]
    fn set_active(&mut self) {
        self.is_active = true;
    }
    #[inline(always)]
    fn set_inactive(&mut self) {
        self.is_active = false;
    }
    #[inline(always)]
    fn set_debounce(&mut self, count: u8) {
        self.debounce_count = count;
    }
    #[inline(always)]
    fn tick_debounce(&mut self) {
        self.debounce_count = self.debounce_count.saturating_sub(1)
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
// TODO: Fix this macro so I dont have to spam ErrorUndefine all over the place (〃＞＿＜;〃)
#[macro_export]
macro_rules! create_key {
    ([ $($key:expr),+ ]) => {
        $crate::key::Key {
            keycodes: [$($key),+],
            is_active: false,
            debounce_count: 0,
        }
    };
}
