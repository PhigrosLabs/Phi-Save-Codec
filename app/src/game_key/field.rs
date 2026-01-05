use crate::phi_base::*;
use shua_struct::BinaryStruct;

#[derive(Debug, Default, BinaryStruct)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct Key {
    pub name: PhiString,
    pub length: u8,
    #[binary_field(align = 8)]
    pub ktype: [bool; 5],
    #[binary_field(size_func = get_flag_len,align = 8,sub_align = 1)]
    pub flag: Vec<bool>,
}
impl Key {
    fn get_flag_len(&self) -> usize {
        return (self.length).saturating_sub(1) as usize;
    }
}

#[derive(Debug, Default, BinaryStruct)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct KeyList {
    pub key_sum: VarInt,
    #[binary_field(size_field = key_sum)]
    pub key_list: Vec<Key>,
}

#[derive(Debug, Default, BinaryStruct)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct GameKey {
    pub key_list: KeyList,
    #[binary_field(align = 8)]
    pub lanota_read_keys: [bool; 6],
    pub camellia_read_key: [bool; 8],
    #[binary_field(align = 8)]
    pub side_story4_begin_read_key: bool,
    #[binary_field(align = 8)]
    pub old_score_cleared_v390: bool,
}
