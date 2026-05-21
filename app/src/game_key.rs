use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::Binary;
use crate::utils::{BinaryError, PhiString, VarInt, get_bit, set_bit};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Key {
    Raw(Vec<u8>),
    Normal(NormalKey),
}

impl Default for Key {
    fn default() -> Self {
        Self::Raw(Vec::new())
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NormalKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_collection_piece_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_single: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_collection_piece_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_illustration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_avatar: Option<bool>,
}

impl Binary for Key {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Ok(Key::Raw(data.to_vec()));
        }

        if data.len() < 2 {
            return Ok(Key::Raw(data.to_vec()));
        }

        let mut pos = 0;

        let length = data[pos];
        pos += 1;

        let type_byte = data[pos];
        pos += 1;

        // type 位后 3 bit 必须为 0
        if (type_byte & 0b1110_0000) != 0 {
            return Ok(Key::Raw(data.to_vec()));
        }

        let exist_read = get_bit(type_byte, 0);
        let exist_single = get_bit(type_byte, 1);
        let exist_collection = get_bit(type_byte, 2);
        let exist_illust = get_bit(type_byte, 3);
        let exist_avatar = get_bit(type_byte, 4);

        if data.len() < pos + length as usize - 1 {
            return Ok(Key::Raw(data.to_vec()));
        }

        let key_data = &data[pos..pos + length as usize - 1];

        let expected_len = exist_read as usize
            + exist_single as usize
            + exist_collection as usize
            + exist_illust as usize
            + exist_avatar as usize;

        if expected_len != key_data.len() {
            return Ok(Key::Raw(data.to_vec()));
        }

        let mut idx = 0;

        let mut key = NormalKey::default();

        if exist_read {
            key.read_collection_piece_num = Some(key_data[idx]);
            idx += 1;
        }

        if exist_single {
            key.unlock_single = Some(key_data[idx] == 1);
            idx += 1;
        }

        if exist_collection {
            key.unlock_collection_piece_num = Some(key_data[idx]);
            idx += 1;
        }

        if exist_illust {
            key.unlock_illustration = Some(key_data[idx] == 1);
            idx += 1;
        }

        if exist_avatar {
            key.unlock_avatar = Some(key_data[idx] == 1);
        }

        Ok(Key::Normal(key))
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        match self {
            Key::Raw(raw) => {
                if data.len() < raw.len() {
                    return Err(BinaryError::NotEnoughData);
                }

                data[..raw.len()].copy_from_slice(raw);

                Ok(())
            }

            Key::Normal(key) => {
                let mut flag = Vec::new();
                let mut type_byte = 0u8;

                if let Some(v) = key.read_collection_piece_num {
                    set_bit(&mut type_byte, 0, true);
                    flag.push(v);
                }

                if let Some(v) = key.unlock_single {
                    set_bit(&mut type_byte, 1, true);
                    flag.push(if v { 1 } else { 0 });
                }

                if let Some(v) = key.unlock_collection_piece_num {
                    set_bit(&mut type_byte, 2, true);
                    flag.push(v);
                }

                if let Some(v) = key.unlock_illustration {
                    set_bit(&mut type_byte, 3, true);
                    flag.push(if v { 1 } else { 0 });
                }

                if let Some(v) = key.unlock_avatar {
                    set_bit(&mut type_byte, 4, true);
                    flag.push(if v { 1 } else { 0 });
                }

                let mut pos = 0;

                if data.is_empty() {
                    return Err(BinaryError::NotEnoughData);
                }

                data[pos] = flag.len() as u8 + 1;
                pos += 1;

                if data.len() < pos + 1 {
                    return Err(BinaryError::NotEnoughData);
                }

                data[pos] = type_byte;
                pos += 1;

                if data.len() < pos + flag.len() {
                    return Err(BinaryError::NotEnoughData);
                }

                data[pos..pos + flag.len()].copy_from_slice(&flag);

                Ok(())
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            Key::Raw(raw) => raw.len(),

            Key::Normal(key) => {
                let flag_len = key.read_collection_piece_num.is_some() as usize
                    + key.unlock_single.is_some() as usize
                    + key.unlock_collection_piece_num.is_some() as usize
                    + key.unlock_illustration.is_some() as usize
                    + key.unlock_avatar.is_some() as usize;

                1 + 1 + flag_len
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameKey {
    pub version: u8,
    #[serde(with = "tuple_vec_map")]
    pub keys: Vec<(String, Key)>,
    pub lanota_read_keys: [bool; 6],
    pub camellia_read_key: Option<bool>,
    pub side_story4_begin_read_key: Option<bool>,
    pub old_score_cleared_v390: Option<bool>,
}

impl Binary for GameKey {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let version = data[pos];
        pos += 1;
        let vi = VarInt::read(&data[pos..])?;
        pos += vi.len();
        let key_sum = vi.0 as usize;
        let mut keys = Vec::with_capacity(key_sum);
        for _ in 0..key_sum {
            let ps = PhiString::read(&data[pos..])?;
            pos += ps.len();
            let name = ps.0;
            let key = Key::read(&data[pos..])?;
            pos += key.len();
            keys.push((name, key));
        }

        let mut lanota_read_keys = [false; 6];
        if data.len() > pos {
            let b = data[pos];
            pos += 1;
            for i in 0..6 {
                lanota_read_keys[i] = get_bit(b, i);
            }
        }

        let (camellia_read_key, pos) = if version >= 2 {
            if data.len() > pos {
                let v = (data[pos] & 1) != 0;
                (Some(v), pos + 1)
            } else {
                (None, pos)
            }
        } else {
            (None, pos)
        };
        let (side_story4_begin_read_key, pos) = if version >= 3 {
            if data.len() > pos {
                let v = (data[pos] & 1) != 0;
                (Some(v), pos + 1)
            } else {
                (None, pos)
            }
        } else {
            (None, pos)
        };
        let old_score_cleared_v390 = if version >= 3 {
            if data.len() > pos {
                Some((data[pos] & 1) != 0)
            } else {
                None
            }
        } else {
            None
        };

        Ok(GameKey {
            version,
            keys,
            lanota_read_keys,
            camellia_read_key,
            side_story4_begin_read_key,
            old_score_cleared_v390,
        })
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        let mut pos = 0;
        data[pos] = self.version;
        pos += 1;
        VarInt(self.keys.len() as u16).write(&mut data[pos..])?;
        pos += VarInt(self.keys.len() as u16).len();
        for (name, key) in &self.keys {
            PhiString(name.clone()).write(&mut data[pos..])?;
            pos += PhiString(name.clone()).len();
            key.write(&mut data[pos..])?;
            pos += key.len();
        }

        let mut b = 0u8;
        for i in 0..6 {
            if self.lanota_read_keys[i] {
                b |= 1 << i;
            }
        }
        data[pos] = b;
        pos += 1;

        if let Some(v) = self.camellia_read_key {
            data[pos] = if v { 1 } else { 0 };
            pos += 1;
        }
        if let Some(v) = self.side_story4_begin_read_key {
            data[pos] = if v { 1 } else { 0 };
            pos += 1;
        }
        if let Some(v) = self.old_score_cleared_v390 {
            data[pos] = if v { 1 } else { 0 };
        }
        Ok(())
    }

    fn len(&self) -> usize {
        let mut bytes = 1 + VarInt(self.keys.len() as u16).len();
        for (name, key) in &self.keys {
            bytes += PhiString(name.clone()).len() + key.len();
        }
        bytes += 1; // lanota_read_keys
        if self.camellia_read_key.is_some() {
            bytes += 1;
        }
        if self.side_story4_begin_read_key.is_some() {
            bytes += 1;
        }
        if self.old_score_cleared_v390.is_some() {
            bytes += 1;
        }
        bytes
    }
}
