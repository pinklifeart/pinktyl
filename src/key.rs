#![deny(unsafe_code)]

use defmt::Format;
use usbd_human_interface_device::page::Keyboard;

#[derive(Debug, Format, Clone, Copy)]
pub enum KeyAction {
    LayerShiftHold,
    KeyCode(Keyboard),
    NoAction,
}

impl From<Keyboard> for KeyAction {
    fn from(value: Keyboard) -> Self {
        Self::KeyCode(value)
    }
}

impl From<KeyAction> for Keyboard {
    fn from(value: KeyAction) -> Self {
        match value {
            KeyAction::KeyCode(k) => k,
            KeyAction::LayerShiftHold => Keyboard::NoEventIndicated,
            KeyAction::NoAction => Keyboard::ErrorUndefine,
        }
    }
}

#[derive(Format, Debug, Clone, Copy)]
pub enum StateChange {
    SetActive,
    SetInactive,
    DebounceTick,
    LayerUp,
    LayerDown,
    NoChange,
}

impl TryFrom<u8> for StateChange {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<StateChange, Self::Error> {
        match value {
            0 => Ok(Self::SetActive),
            1 => Ok(Self::SetInactive),
            2 => Ok(Self::DebounceTick),
            3 => Ok(Self::LayerUp),
            4 => Ok(Self::LayerDown),
            5 => Ok(Self::NoChange),
            _ => Err("No corresponding StateChange variant for the provided u8 value."),
        }
    }
}

#[derive(Debug, Format)]
pub struct Message {
    pub state_change: StateChange,
    pub row: usize,
    pub col: usize,
}

impl Message {
    pub fn new(state_change: StateChange, row: usize, col: usize) -> Self {
        Message {
            state_change,
            row,
            col,
        }
    }
}

pub struct Key {
    pub keycodes: [KeyAction; crate::matrix::LAYERS],
    pub is_active: bool,
    pub debounce_count: u8,
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
                    KeyAction::LayerShiftHold => Some(StateChange::LayerUp),
                    _ => Some(StateChange::SetActive),
                }
            }
        } else if self.is_active {
            self.set_inactive();
            self.set_debounce(debounce_limit);
            match self.keycodes[active_layer] {
                KeyAction::LayerShiftHold => Some(StateChange::LayerDown),
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
    pub fn tick_debounce(&mut self) {
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
