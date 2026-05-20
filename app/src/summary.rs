use crate::Binary;
use crate::utils::{BinaryError, PhiString, VarInt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Level {
    pub clear: u16,
    pub fc: u16,
    pub phi: u16,
}

impl Binary for Level {
    type Error = BinaryError;
    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 6 {
            return Err(BinaryError::NotEnoughData);
        }
        Ok(Level {
            clear: u16::from_le_bytes(data[0..2].try_into().unwrap()),
            fc: u16::from_le_bytes(data[2..4].try_into().unwrap()),
            phi: u16::from_le_bytes(data[4..6].try_into().unwrap()),
        })
    }
    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        if data.len() < 6 {
            return Err(BinaryError::NotEnoughData);
        }
        data[0..2].copy_from_slice(&self.clear.to_le_bytes());
        data[2..4].copy_from_slice(&self.fc.to_le_bytes());
        data[4..6].copy_from_slice(&self.phi.to_le_bytes());
        Ok(())
    }
    fn len(&self) -> usize {
        6
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MultiLevel {
    pub ez: Level,
    pub hd: Level,
    #[serde(rename = "in")]
    pub r#in: Level,
    pub at: Level,
}

impl Binary for MultiLevel {
    type Error = BinaryError;
    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let ez = Level::read(&data[pos..])?;
        pos += ez.len();
        let hd = Level::read(&data[pos..])?;
        pos += hd.len();
        let r#in = Level::read(&data[pos..])?;
        pos += r#in.len();
        let at = Level::read(&data[pos..])?;
        Ok(MultiLevel { ez, hd, r#in, at })
    }
    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        let mut pos = 0;
        self.ez.write(&mut data[pos..])?;
        pos += self.ez.len();
        self.hd.write(&mut data[pos..])?;
        pos += self.hd.len();
        self.r#in.write(&mut data[pos..])?;
        pos += self.r#in.len();
        self.at.write(&mut data[pos..])?;
        Ok(())
    }
    fn len(&self) -> usize {
        24
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub save_version: u8,
    pub challenge_mode_rank: u16,
    pub rks: f32,
    pub game_version: VarInt,
    pub avatar: PhiString,
    pub level: MultiLevel,
}

impl Binary for Summary {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let save_version = data[pos];
        pos += 1;
        let challenge_mode_rank = u16::from_le_bytes(data[pos..pos + 2].try_into().unwrap());
        pos += 2;
        let rks = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let game_version = VarInt::read(&data[pos..])?;
        pos += game_version.len();
        let avatar = PhiString::read(&data[pos..])?;
        pos += avatar.len();
        let level = MultiLevel::read(&data[pos..])?;
        Ok(Summary {
            save_version,
            challenge_mode_rank,
            rks,
            game_version,
            avatar,
            level,
        })
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        let mut pos = 0;
        data[pos] = self.save_version;
        pos += 1;
        data[pos..pos + 2].copy_from_slice(&self.challenge_mode_rank.to_le_bytes());
        pos += 2;
        data[pos..pos + 4].copy_from_slice(&self.rks.to_le_bytes());
        pos += 4;
        self.game_version.write(&mut data[pos..])?;
        pos += self.game_version.len();
        self.avatar.write(&mut data[pos..])?;
        pos += self.avatar.len();
        self.level.write(&mut data[pos..])?;
        Ok(())
    }

    fn len(&self) -> usize {
        1 + 2 + 4 + self.game_version.len() + self.avatar.len() + 24
    }
}
