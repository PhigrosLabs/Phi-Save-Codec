use crate::phi_base::*;
use shua_struct::BinaryField;

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct KeyType {
    pub exist_read_collection_piece_num: bool,
    pub exist_unlcok_single: bool,
    pub exist_unlock_collection_piece_num: bool,
    pub exist_unlock_illustration: bool,
    pub exist_unlock_avatar: bool,
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct Key {
    pub key: PhiString,
    pub length: u8,
    #[binary_field(align = 8)]
    pub r#type: KeyType,
    #[binary_field(count_func = get_flag_len)]
    pub flag: Vec<u8>,
}
impl Key {
    fn get_flag_len(&self) -> usize {
        (self.length).saturating_sub(1) as usize
    }
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct KeyList {
    pub key_sum: VarInt,
    #[binary_field(count_field = key_sum)]
    pub key_list: Vec<Key>,
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct GameKey {
    pub version: u8,
    pub key_list: KeyList,
    #[binary_field(align = 8)] // is_version_at_least_1
    pub lanota_read_keys: [bool; 6],
    #[binary_field(align = 8, if_func = "is_version_at_least_2")]
    pub camellia_read_key: Option<bool>,
    #[binary_field(align = 8, if_func = "is_version_at_least_3")]
    pub side_story4_begin_read_key: Option<bool>,
    #[binary_field(align = 8, if_func = "is_version_at_least_3")]
    pub old_score_cleared_v390: Option<bool>,
}

impl GameKey {
    fn is_version_at_least_3(&self) -> bool {
        self.version >= 3
    }

    fn is_version_at_least_2(&self) -> bool {
        self.version >= 2
    }
}
