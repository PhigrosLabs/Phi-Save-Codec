use serde::{Deserialize, Serialize};

use crate::Binary;
use crate::types::Difficulty4;
use crate::utils::{self, BinaryError, PhiString, VarInt};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProgressBase {
    pub is_first_run: bool,
    pub legacy_chapter_finished: bool,
    pub already_show_collection_tip: bool,
    pub already_show_auto_unlock_in_tip: bool,
}

impl Binary for ProgressBase {
    type Error = BinaryError;
    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let b = data[0];
        Ok(ProgressBase {
            is_first_run: utils::get_bit(b, 0),
            legacy_chapter_finished: utils::get_bit(b, 1),
            already_show_collection_tip: utils::get_bit(b, 2),
            already_show_auto_unlock_in_tip: utils::get_bit(b, 3),
        })
    }
    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let mut b = 0u8;
        utils::set_bit(&mut b, 0, self.is_first_run);
        utils::set_bit(&mut b, 1, self.legacy_chapter_finished);
        utils::set_bit(&mut b, 2, self.already_show_collection_tip);
        utils::set_bit(&mut b, 3, self.already_show_auto_unlock_in_tip);
        data[0] = b;
        Ok(())
    }
    fn len(&self) -> usize {
        1
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Money {
    pub kib: VarInt,
    pub mib: VarInt,
    pub gib: VarInt,
    pub tib: VarInt,
    pub pib: VarInt,
}

impl Binary for Money {
    type Error = BinaryError;
    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let kib = VarInt::read(&data[pos..])?;
        pos += kib.len();
        let mib = VarInt::read(&data[pos..])?;
        pos += mib.len();
        let gib = VarInt::read(&data[pos..])?;
        pos += gib.len();
        let tib = VarInt::read(&data[pos..])?;
        pos += tib.len();
        let pib = VarInt::read(&data[pos..])?;
        Ok(Money {
            kib,
            mib,
            gib,
            tib,
            pib,
        })
    }
    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        let mut pos = 0;
        self.kib.write(&mut data[pos..])?;
        pos += self.kib.len();
        self.mib.write(&mut data[pos..])?;
        pos += self.mib.len();
        self.gib.write(&mut data[pos..])?;
        pos += self.gib.len();
        self.tib.write(&mut data[pos..])?;
        pos += self.tib.len();
        self.pib.write(&mut data[pos..])?;
        Ok(())
    }
    fn len(&self) -> usize {
        self.kib.len() + self.mib.len() + self.gib.len() + self.tib.len() + self.pib.len()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Chapter8Base {
    pub unlock_begin: bool,
    pub unlock_second_phase: bool,
    pub passed: bool,
}

impl Binary for Chapter8Base {
    type Error = BinaryError;
    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let b = data[0];
        Ok(Chapter8Base {
            unlock_begin: utils::get_bit(b, 0),
            unlock_second_phase: utils::get_bit(b, 1),
            passed: utils::get_bit(b, 2),
        })
    }
    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let mut b = 0u8;
        utils::set_bit(&mut b, 0, self.unlock_begin);
        utils::set_bit(&mut b, 1, self.unlock_second_phase);
        utils::set_bit(&mut b, 2, self.passed);
        data[0] = b;
        Ok(())
    }
    fn len(&self) -> usize {
        1
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameProgress {
    pub version: u8,
    pub base: ProgressBase,
    pub completed: PhiString,
    pub song_update_info: VarInt,
    pub challenge_mode_rank: u16,
    pub money: Money,
    pub unlock_flag_of_spasmodic: Difficulty4,
    pub unlock_flag_of_igallta: Difficulty4,
    pub unlock_flag_of_rrharil: Difficulty4,
    pub flag_of_song_record_key: [bool; 8],
    pub random_version_unlocked: Option<[bool; 6]>,
    pub chapter8_base: Option<Chapter8Base>,
    pub chapter8_song_unlocked: Option<[bool; 6]>,
    pub flag_of_song_record_key_takumi: Option<[bool; 3]>,
}

fn read_bool_array<const N: usize>(data: &[u8]) -> Result<[bool; N], BinaryError> {
    if data.is_empty() {
        return Err(BinaryError::NotEnoughData);
    }
    let b = data[0];
    let mut arr = [false; N];
    for i in 0..N {
        arr[i] = utils::get_bit(b, i);
    }
    Ok(arr)
}

fn write_bool_array<const N: usize>(arr: &[bool; N], data: &mut [u8]) -> Result<(), BinaryError> {
    if data.is_empty() {
        return Err(BinaryError::NotEnoughData);
    }
    let mut b = 0u8;
    for i in 0..N {
        if arr[i] {
            b |= 1 << i;
        }
    }
    data[0] = b;
    Ok(())
}

impl Binary for GameProgress {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let version = data[pos];
        pos += 1;
        let base = ProgressBase::read(&data[pos..])?;
        pos += base.len();
        let completed = PhiString::read(&data[pos..])?;
        pos += completed.len();
        let song_update_info = VarInt::read(&data[pos..])?;
        pos += song_update_info.len();
        let challenge_mode_rank = u16::from_le_bytes(data[pos..pos + 2].try_into().unwrap());
        pos += 2;
        let money = Money::read(&data[pos..])?;
        pos += money.len();
        let spasmodic = Difficulty4::read(&data[pos..])?;
        pos += spasmodic.len();
        let igallta = Difficulty4::read(&data[pos..])?;
        pos += igallta.len();
        let rrharil = Difficulty4::read(&data[pos..])?;
        pos += rrharil.len();
        let flag_of_song_record_key = read_bool_array::<8>(&data[pos..])?;
        pos += 1;
        let (random_version_unlocked, pos) = if version >= 2 {
            let v = read_bool_array::<6>(&data[pos..])?;
            (Some(v), pos + 1)
        } else {
            (None, pos)
        };
        let (chapter8_base, pos) = if version >= 3 {
            let v = Chapter8Base::read(&data[pos..])?;
            let n = v.len();
            (Some(v), pos + n)
        } else {
            (None, pos)
        };
        let (chapter8_song_unlocked, pos) = if version >= 3 {
            let v = read_bool_array::<6>(&data[pos..])?;
            (Some(v), pos + 1)
        } else {
            (None, pos)
        };
        let flag_of_song_record_key_takumi = if version >= 4 {
            Some(read_bool_array::<3>(&data[pos..])?)
        } else {
            None
        };
        Ok(GameProgress {
            version,
            base,
            completed,
            song_update_info,
            challenge_mode_rank,
            money,
            unlock_flag_of_spasmodic: spasmodic,
            unlock_flag_of_igallta: igallta,
            unlock_flag_of_rrharil: rrharil,
            flag_of_song_record_key,
            random_version_unlocked,
            chapter8_base,
            chapter8_song_unlocked,
            flag_of_song_record_key_takumi,
        })
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        let mut pos = 0;
        data[pos] = self.version;
        pos += 1;
        self.base.write(&mut data[pos..])?;
        pos += self.base.len();
        self.completed.write(&mut data[pos..])?;
        pos += self.completed.len();
        self.song_update_info.write(&mut data[pos..])?;
        pos += self.song_update_info.len();
        data[pos..pos + 2].copy_from_slice(&self.challenge_mode_rank.to_le_bytes());
        pos += 2;
        self.money.write(&mut data[pos..])?;
        pos += self.money.len();
        self.unlock_flag_of_spasmodic.write(&mut data[pos..])?;
        pos += 1;
        self.unlock_flag_of_igallta.write(&mut data[pos..])?;
        pos += 1;
        self.unlock_flag_of_rrharil.write(&mut data[pos..])?;
        pos += 1;
        write_bool_array(&self.flag_of_song_record_key, &mut data[pos..])?;
        pos += 1;
        if let Some(ref v) = self.random_version_unlocked {
            write_bool_array(v, &mut data[pos..])?;
            pos += 1;
        }
        if let Some(ref v) = self.chapter8_base {
            v.write(&mut data[pos..])?;
            pos += v.len();
        }
        if let Some(ref v) = self.chapter8_song_unlocked {
            write_bool_array(v, &mut data[pos..])?;
            pos += 1;
        }
        if let Some(ref v) = self.flag_of_song_record_key_takumi {
            write_bool_array(v, &mut data[pos..])?;
        }
        Ok(())
    }

    fn len(&self) -> usize {
        let mut bytes = 1
            + 1
            + self.completed.len()
            + self.song_update_info.len()
            + 2
            + self.money.len()
            + 3
            + 1;
        if self.random_version_unlocked.is_some() {
            bytes += 1;
        }
        if self.chapter8_base.is_some() {
            bytes += 1;
        }
        if self.chapter8_song_unlocked.is_some() {
            bytes += 1;
        }
        if self.flag_of_song_record_key_takumi.is_some() {
            bytes += 1;
        }
        bytes
    }
}
