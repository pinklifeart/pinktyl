#![deny(unsafe_code)]
use crate::key::Key;
use usbd_human_interface_device::page::Keyboard;

// TODO: Maybe figure out a way to override these when setting a layout, potentially via a macro
pub const ROWS: usize = 6;
pub const COLS: usize = 12;
pub const LAYERS: usize = 2;

pub type Layer = [[Key; COLS]; ROWS];

#[macro_export]
macro_rules! create_matrix {
    ( $([ $( [$($keycode:ident),+] ),+ ]),+ ) => {
        [$([$(create_key!([$($keycode),+])),+]),+]
    };
}

pub struct Matrix {
    layout: Layer,
    active_layer: usize,
}

impl Matrix {
    pub fn get_key(&self, row: usize, col: usize) -> &Key {
        &self.layout[row][col]
    }

    pub fn get_key_mut(&mut self, row: usize, col: usize) -> &mut Key {
        &mut self.layout[row][col]
    }
    pub fn set_active_layer(&mut self, layer_number: usize) {
        if layer_number < LAYERS {
            self.active_layer = layer_number;
        }
    }

    // TODO: Implement a function that returns IntoIterator<Item = Keyboard>
    pub fn report_active(&self) -> &[Keyboard] {
        unimplemented!();
    }
}
