use alloc::collections::BTreeMap;
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SongRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ez: Option<LevelRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd: Option<LevelRecord>,
    #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
    pub r#in: Option<LevelRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<LevelRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<LevelRecord>,
}

impl Binary for SongRecord {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let _vi = VarInt::read(&data[pos..])?;
        pos += _vi.len(); // length = levels_len * 8 + 2
        let unlock = Difficulty5::read(&data[pos..])?;
        pos += unlock.len();
        let fc = Difficulty5::read(&data[pos..])?;
        pos += fc.len();

        let num_levels = unlock.count_unlocked();
        let mut levels = Vec::with_capacity(num_levels);
        for _ in 0..num_levels {
            let lr = LevelRecord::read(&data[pos..])?;
            pos += lr.len();
            levels.push(lr);
        }

        let mut sr = SongRecord::default();
        let mut idx = 0;
        let mut set = |unlocked: bool, fc: bool| -> Option<LevelRecord> {
            if unlocked {
                let lr = levels.get(idx).map(|l| LevelRecord {
                    score: l.score,
                    acc: l.acc,
                    fc,
                });
                idx += 1;
                lr
            } else {
                None
            }
        };
        sr.ez = set(unlock.ez, fc.ez);
        sr.hd = set(unlock.hd, fc.hd);
        sr.r#in = set(unlock.r#in, fc.r#in);
        sr.at = set(unlock.at, fc.at);
        sr.legacy = set(unlock.legacy, fc.legacy);
        Ok(sr)
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        let mut unlock = Difficulty5::default();
        let mut fc = Difficulty5::default();
        let mut level_count = 0;

        if let Some(lr) = &self.ez {
            unlock.ez = true;
            fc.ez = lr.fc;
            level_count += 1;
        }
        if let Some(lr) = &self.hd {
            unlock.hd = true;
            fc.hd = lr.fc;
            level_count += 1;
        }
        if let Some(lr) = &self.r#in {
            unlock.r#in = true;
            fc.r#in = lr.fc;
            level_count += 1;
        }
        if let Some(lr) = &self.at {
            unlock.at = true;
            fc.at = lr.fc;
            level_count += 1;
        }
        if let Some(lr) = &self.legacy {
            unlock.legacy = true;
            fc.legacy = lr.fc;
            level_count += 1;
        }

        let mut pos = 0;
        VarInt((level_count as u16 * 8) + 2).write(&mut data[pos..])?;
        pos += VarInt((level_count as u16 * 8) + 2).len();
        unlock.write(&mut data[pos..])?;
        pos += unlock.len();
        fc.write(&mut data[pos..])?;
        pos += fc.len();
        let write_one = |opt: &Option<LevelRecord>,
                         data: &mut [u8],
                         pos: usize|
         -> Result<usize, BinaryError> {
            if let Some(lr) = opt {
                lr.write(&mut data[pos..])?;
                Ok(pos + lr.len())
            } else {
                Ok(pos)
            }
        };
        pos = write_one(&self.ez, data, pos)?;
        pos = write_one(&self.hd, data, pos)?;
        pos = write_one(&self.r#in, data, pos)?;
        pos = write_one(&self.at, data, pos)?;
        write_one(&self.legacy, data, pos)?;
        Ok(())
    }

    fn len(&self) -> usize {
        let level_count = self.ez.is_some() as usize
            + self.hd.is_some() as usize
            + self.r#in.is_some() as usize
            + self.at.is_some() as usize
            + self.legacy.is_some() as usize;
        VarInt((level_count as u16 * 8) + 2).len() + 1 + 1 + level_count * 8
    }
}

// --- GameRecord ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameRecord {
    pub version: u8,
    pub songs: BTreeMap<String, SongRecord>,
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
        let mut songs = BTreeMap::new();
        for _ in 0..song_sum {
            let ps = PhiString::read(&data[pos..])?;
            pos += ps.len();
            let name = ps.0;
            let sr = SongRecord::read(&data[pos..])?;
            pos += sr.len();
            songs.insert(name, sr);
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
