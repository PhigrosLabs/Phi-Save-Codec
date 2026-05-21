use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::Binary;
use crate::types::Difficulty5;
use crate::utils::{BinaryError, PhiString, VarInt};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelRecord {
    pub score: u32,
    pub acc: f32,
    pub fc: bool,
}

impl Binary for LevelRecord {
    type Error = BinaryError;
    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(BinaryError::NotEnoughData);
        }
        Ok(LevelRecord {
            score: u32::from_le_bytes(data[0..4].try_into().unwrap()),
            acc: f32::from_le_bytes(data[4..8].try_into().unwrap()),
            fc: false,
        })
    }
    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        if data.len() < 8 {
            return Err(BinaryError::NotEnoughData);
        }
        data[0..4].copy_from_slice(&self.score.to_le_bytes());
        data[4..8].copy_from_slice(&self.acc.to_le_bytes());
        Ok(())
    }
    fn len(&self) -> usize {
        8
    }
}

// --- SongRecord ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum SongRecord {
    Raw(Vec<u8>),
    Normal(NormalSongRecord),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NormalSongRecord {
    #[serde(rename = "EZ", skip_serializing_if = "Option::is_none")]
    pub ez: Option<LevelRecord>,
    #[serde(rename = "HD", skip_serializing_if = "Option::is_none")]
    pub hd: Option<LevelRecord>,
    #[serde(rename = "IN", skip_serializing_if = "Option::is_none")]
    pub r#in: Option<LevelRecord>,
    #[serde(rename = "AT", skip_serializing_if = "Option::is_none")]
    pub at: Option<LevelRecord>,
    #[serde(rename = "Legacy", skip_serializing_if = "Option::is_none")]
    pub legacy: Option<LevelRecord>,
}

impl Binary for SongRecord {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        fn parse_normal(data: &[u8]) -> Result<NormalSongRecord, BinaryError> {
            let mut pos = 0;

            let payload_len = VarInt::read(&data[pos..])?;
            pos += payload_len.len();

            // unlock + fc
            if data.len() < pos + 2 {
                return Err(BinaryError::NotEnoughData);
            }

            let unlock = Difficulty5::read(&data[pos..])?;
            pos += unlock.len();

            let fc = Difficulty5::read(&data[pos..])?;
            pos += fc.len();

            let num_levels = unlock.count_unlocked();

            let mut levels = Vec::with_capacity(num_levels);

            for _ in 0..num_levels {
                if data.len() < pos + 8 {
                    return Err(BinaryError::NotEnoughData);
                }

                let lr = LevelRecord::read(&data[pos..])?;
                pos += lr.len();
                levels.push(lr);
            }

            // payload 长度校验
            let expected_payload = num_levels * 8 + 2;
            if payload_len.0 as usize != expected_payload {
                return Err(BinaryError::InvalidData);
            }

            let mut sr = NormalSongRecord::default();

            let mut idx = 0;

            let mut take = |unlocked: bool, fc_flag: bool| -> Option<LevelRecord> {
                if !unlocked {
                    return None;
                }

                let base = levels.get(idx)?;
                idx += 1;

                Some(LevelRecord {
                    score: base.score,
                    acc: base.acc,
                    fc: fc_flag,
                })
            };

            sr.ez = take(unlock.ez, fc.ez);
            sr.hd = take(unlock.hd, fc.hd);
            sr.r#in = take(unlock.r#in, fc.r#in);
            sr.at = take(unlock.at, fc.at);
            sr.legacy = take(unlock.legacy, fc.legacy);

            Ok(sr)
        }

        match parse_normal(data) {
            Ok(v) => Ok(SongRecord::Normal(v)),
            Err(_) => Ok(SongRecord::Raw(data.to_vec())),
        }
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        match self {
            SongRecord::Raw(raw) => {
                if data.len() < raw.len() {
                    return Err(BinaryError::NotEnoughData);
                }

                data[..raw.len()].copy_from_slice(raw);
                Ok(())
            }

            SongRecord::Normal(sr) => {
                let mut unlock = Difficulty5::default();
                let mut fc = Difficulty5::default();
                let mut level_count = 0;

                if let Some(lr) = &sr.ez {
                    unlock.ez = true;
                    fc.ez = lr.fc;
                    level_count += 1;
                }

                if let Some(lr) = &sr.hd {
                    unlock.hd = true;
                    fc.hd = lr.fc;
                    level_count += 1;
                }

                if let Some(lr) = &sr.r#in {
                    unlock.r#in = true;
                    fc.r#in = lr.fc;
                    level_count += 1;
                }

                if let Some(lr) = &sr.at {
                    unlock.at = true;
                    fc.at = lr.fc;
                    level_count += 1;
                }

                if let Some(lr) = &sr.legacy {
                    unlock.legacy = true;
                    fc.legacy = lr.fc;
                    level_count += 1;
                }

                let mut pos = 0;

                let payload = (level_count as u16 * 8) + 2;

                let varint = VarInt(payload);

                varint.write(&mut data[pos..])?;
                pos += varint.len();

                unlock.write(&mut data[pos..])?;
                pos += unlock.len();

                fc.write(&mut data[pos..])?;
                pos += fc.len();

                let mut write_one = |opt: &Option<LevelRecord>| -> Result<(), BinaryError> {
                    if let Some(lr) = opt {
                        lr.write(&mut data[pos..])?;
                        pos += lr.len();
                    }
                    Ok(())
                };

                write_one(&sr.ez)?;
                write_one(&sr.hd)?;
                write_one(&sr.r#in)?;
                write_one(&sr.at)?;
                write_one(&sr.legacy)?;

                Ok(())
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            SongRecord::Raw(raw) => raw.len(),

            SongRecord::Normal(sr) => {
                let level_count = sr.ez.is_some() as usize
                    + sr.hd.is_some() as usize
                    + sr.r#in.is_some() as usize
                    + sr.at.is_some() as usize
                    + sr.legacy.is_some() as usize;

                let payload = (level_count as u16 * 8) + 2;

                VarInt(payload).len() + 2 + level_count * 8
            }
        }
    }
}

// --- GameRecord ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameRecord {
    pub version: u8,
    #[serde(with = "tuple_vec_map")]
    pub songs: Vec<(String, SongRecord)>,
}

impl Binary for GameRecord {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let version = data[pos];
        pos += 1;
        let vi = VarInt::read(&data[pos..])?;
        pos += vi.len();
        let song_sum = vi.0 as usize;
        let mut songs = Vec::with_capacity(song_sum);
        for _ in 0..song_sum {
            let ps = PhiString::read(&data[pos..])?;
            pos += ps.len();
            let name = ps.0;
            let sr = SongRecord::read(&data[pos..])?;
            pos += sr.len();
            songs.push((name, sr));
        }
        Ok(GameRecord { version, songs })
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        let mut pos = 0;
        data[pos] = self.version;
        pos += 1;
        VarInt(self.songs.len() as u16).write(&mut data[pos..])?;
        pos += VarInt(self.songs.len() as u16).len();
        for (name, sr) in &self.songs {
            PhiString(name.clone()).write(&mut data[pos..])?;
            pos += PhiString(name.clone()).len();
            sr.write(&mut data[pos..])?;
            pos += sr.len();
        }
        Ok(())
    }

    fn len(&self) -> usize {
        let mut bytes = 1 + VarInt(self.songs.len() as u16).len();
        for (name, sr) in &self.songs {
            bytes += PhiString(name.clone()).len() + sr.len();
        }
        bytes
    }
}
