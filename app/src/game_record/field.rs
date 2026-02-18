use crate::phi_base::*;
use shua_struct::BinaryField;

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct LevelRecord {
    pub score: u32,
    pub acc: f32,
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct SongEntry {
    pub name: PhiString,
    pub length: VarInt,
    #[binary_field(align = 8)]
    pub unlock: [bool; 5],
    #[binary_field(align = 8)]
    pub fc: [bool; 5],
    #[binary_field(size_func = get_levels_len)]
    pub levels: Vec<LevelRecord>,
}
impl SongEntry {
    fn get_levels_len(&self) -> usize {
        self.unlock.iter().filter(|bit_bool| **bit_bool).count()
    }
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct GameRecord {
    pub version: u8,
    pub song_sum: VarInt,
    #[binary_field(size_field = song_sum)]
    pub song_list: Vec<SongEntry>,
}
