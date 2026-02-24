use std::collections::BTreeMap;

use super::field::*;
use crate::phi_base::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SerializableKey {
    pub read_collection_piece_num: Option<u8>,
    pub unlock_single: Option<bool>,
    pub unlock_collection_piece_num: Option<u8>,
    pub unlock_illustration: Option<bool>,
    pub unlock_avatar: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableGameKey {
    pub version: u8,
    pub keys: BTreeMap<String, SerializableKey>,
    pub lanota_read_keys: [bool; 6],
    pub camellia_read_key: Option<bool>,
    pub side_story4_begin_read_key: Option<bool>,
    pub old_score_cleared_v390: Option<bool>,
}

#[allow(unused_assignments)]
impl From<Key> for SerializableKey {
    fn from(k: Key) -> Self {
        let mut flag_idx = 0;
        let mut result = SerializableKey {
            read_collection_piece_num: None,
            unlock_single: None,
            unlock_collection_piece_num: None,
            unlock_illustration: None,
            unlock_avatar: None,
        };

        if k.r#type.exist_read_collection_piece_num && flag_idx < k.flag.len() {
            result.read_collection_piece_num = Some(k.flag[flag_idx]);
            flag_idx += 1;
        }
        if k.r#type.exist_unlcok_single && flag_idx < k.flag.len() {
            result.unlock_single = Some(k.flag[flag_idx] == 1);
            flag_idx += 1;
        }
        if k.r#type.exist_unlock_collection_piece_num && flag_idx < k.flag.len() {
            result.unlock_collection_piece_num = Some(k.flag[flag_idx]);
            flag_idx += 1;
        }
        if k.r#type.exist_unlock_illustration && flag_idx < k.flag.len() {
            result.unlock_illustration = Some(k.flag[flag_idx] == 1);
            flag_idx += 1;
        }
        if k.r#type.exist_unlock_avatar && flag_idx < k.flag.len() {
            result.unlock_avatar = Some(k.flag[flag_idx] == 1);
            flag_idx += 1;
        }

        result
    }
}

impl From<GameKey> for SerializableGameKey {
    fn from(gk: GameKey) -> Self {
        let mut keys: BTreeMap<String, SerializableKey> = BTreeMap::new();
        for k in gk.key_list.key_list.into_iter() {
            keys.insert(k.key.0.clone(), SerializableKey::from(k));
        }

        SerializableGameKey {
            version: gk.version,
            keys,
            lanota_read_keys: gk.lanota_read_keys,
            camellia_read_key: gk.camellia_read_key,
            side_story4_begin_read_key: gk.side_story4_begin_read_key,
            old_score_cleared_v390: gk.old_score_cleared_v390,
        }
    }
}

impl From<(String, SerializableKey)> for Key {
    fn from((key_str, sk): (String, SerializableKey)) -> Self {
        let mut flag = Vec::new();
        let mut ktype = KeyType::default();

        if let Some(v) = sk.read_collection_piece_num {
            ktype.exist_read_collection_piece_num = true;
            flag.push(v);
        }
        if let Some(v) = sk.unlock_single {
            ktype.exist_unlcok_single = true;
            flag.push(if v { 1 } else { 0 });
        }
        if let Some(v) = sk.unlock_collection_piece_num {
            ktype.exist_unlock_collection_piece_num = true;
            flag.push(v);
        }
        if let Some(v) = sk.unlock_illustration {
            ktype.exist_unlock_illustration = true;
            flag.push(if v { 1 } else { 0 });
        }
        if let Some(v) = sk.unlock_avatar {
            ktype.exist_unlock_avatar = true;
            flag.push(if v { 1 } else { 0 });
        }

        Key {
            key: PhiString(key_str),
            length: flag.len() as u8 + 1,
            r#type: ktype,
            flag,
        }
    }
}

impl From<SerializableGameKey> for GameKey {
    fn from(sgk: SerializableGameKey) -> Self {
        let key_sum = sgk.keys.len();
        let key_list = sgk.keys.into_iter().map(Key::from).collect();

        GameKey {
            version: sgk.version,
            key_list: KeyList {
                key_sum: VarInt(key_sum as u16),
                key_list,
            },
            lanota_read_keys: sgk.lanota_read_keys,
            camellia_read_key: sgk.camellia_read_key,
            side_story4_begin_read_key: sgk.side_story4_begin_read_key,
            old_score_cleared_v390: sgk.old_score_cleared_v390,
        }
    }
}
