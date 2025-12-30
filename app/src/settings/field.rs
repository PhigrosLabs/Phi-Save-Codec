pub(crate) use crate::phi_base::*;
use bitvec::prelude::*;
use shua_struct::field::{BinaryField, Options};
use shua_struct_macro::binary_struct;
use std::cell::Cell;

#[derive(Debug, Default)]
#[binary_struct(bit_order = Lsb0)]
pub struct SettingsBase {
    pub chord_support: bool,
    pub fc_ap_indicator: bool,
    pub enable_hit_sound: bool,
    pub low_resolution_mode: bool,
}

#[derive(Debug, Default)]
#[binary_struct(bit_order = Lsb0)]
pub struct Settings {
    #[binary_field(align = 8)]
    pub base: SettingsBase,
    pub device_name: PhiString,
    pub bright: f32,
    pub music_volume: f32,
    pub effect_volume: f32,
    pub hit_sound_volume: f32,
    pub sound_offset: f32,
    pub note_scale: f32,
}
