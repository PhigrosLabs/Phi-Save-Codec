use crate::phi_base::*;
use shua_struct::BinaryField;

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct Level {
    pub clear: u16,
    pub fc: u16,
    pub phi: u16,
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct MultiLevel {
    pub ez: Level,
    pub hd: Level,
    pub r#in: Level,
    pub at: Level,
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct Summary {
    pub save_version: u8,
    pub challenge_mode_rank: u16,
    pub rks: f32,
    pub game_version: VarInt,
    pub avatar: PhiString,
    pub level: MultiLevel,
}
