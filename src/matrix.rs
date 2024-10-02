#![deny(unsafe_code)]
use crate::key::Key;
use heapless::Vec;
use usbd_human_interface_device::page::Keyboard;

// TODO: Maybe figure out a way to override these when setting a layout, potentially via a macro
pub const ROWS: usize = 6;
pub const COLS: usize = 12;
pub const LAYERS: usize = 2;
const KEYS: usize = ROWS * COLS;

pub type Layer = [[Key; COLS]; ROWS];

#[macro_export]
macro_rules! create_matrix {
    ( $([ $( [$($keycode:expr),+] ),+ ]),+ ) => {
        [$([$(create_key!([$($keycode),+])),+]),+]
    };
}

pub struct Matrix {
    pub layout: Layer,
    pub active_layer: usize,
}

impl Matrix {
    pub fn get_key(&self, row: usize, col: usize) -> &Key {
        &self.layout[row][col]
    }

    pub fn get_key_mut(&mut self, row: usize, col: usize) -> &mut Key {
        &mut self.layout[row][col]
    }
    pub fn increment_layer(&mut self) {
        if self.active_layer < LAYERS {
            self.active_layer += 1;
        }
    }
    pub fn decrement_layer(&mut self) {
        self.active_layer = self.active_layer.saturating_sub(1);
    }
    pub fn set_active_layer(&mut self, layer_number: usize) {
        if layer_number < LAYERS {
            self.active_layer = layer_number;
        }
    }
    pub fn new() -> Matrix {
        Matrix {
            layout: crate::layout::LAYOUT,
            active_layer: 0,
        }
    }

    // TODO: Implement a function that returns IntoIterator<Item = Keyboard>
    pub fn report_active(&self) -> Vec<Keyboard, KEYS> {
        self.layout
            .iter()
            .flatten()
            .map(|key| {
                if key.is_active {
                    key.keycodes[self.active_layer].clone().into()
                } else {
                    Keyboard::NoEventIndicated
                }
            })
            .collect()
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}
