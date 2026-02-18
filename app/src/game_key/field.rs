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
    #[binary_field(size_func = get_flag_len)]
    pub flag: Vec<u8>,
}
impl Key {
    fn get_flag_len(&self) -> usize {
        return (self.length).saturating_sub(1) as usize;
    }
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct KeyList {
    pub key_sum: VarInt,
    #[binary_field(size_field = key_sum)]
    pub key_list: Vec<Key>,
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct GameKey {
    #[binary_field(check_func = "check_version")]
    pub version: u8,
    pub key_list: KeyList,
    #[binary_field(align = 8)]
    pub lanota_read_keys: [bool; 6],
    pub camellia_read_key: [bool; 8],
    #[binary_field(align = 8, if_func = "is_version_at_least_3")]
    pub side_story4_begin_read_key: Option<bool>,
    #[binary_field(align = 8, if_func = "is_version_at_least_3")]
    pub old_score_cleared_v390: Option<bool>,
}

impl GameKey {
    fn check_version(&self) -> Option<String> {
        if self.version < 2 {
            return Some("Not supported".into());
        } else {
            return None;
        }
    }

    fn is_version_at_least_3(&self) -> bool {
        self.version >= 3
    }
}
