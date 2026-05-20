use crate::Binary;
use crate::utils::{self, BinaryError, PhiString};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SettingsBase {
    pub chord_support: bool,
    pub fc_ap_indicator: bool,
    pub enable_hit_sound: bool,
    pub low_resolution_mode: bool,
}

impl Binary for SettingsBase {
    type Error = BinaryError;
    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let b = data[0];
        Ok(SettingsBase {
            chord_support: utils::get_bit(b, 0),
            fc_ap_indicator: utils::get_bit(b, 1),
            enable_hit_sound: utils::get_bit(b, 2),
            low_resolution_mode: utils::get_bit(b, 3),
        })
    }
    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let mut b = 0u8;
        utils::set_bit(&mut b, 0, self.chord_support);
        utils::set_bit(&mut b, 1, self.fc_ap_indicator);
        utils::set_bit(&mut b, 2, self.enable_hit_sound);
        utils::set_bit(&mut b, 3, self.low_resolution_mode);
        data[0] = b;
        Ok(())
    }
    fn len(&self) -> usize {
        1
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub version: u8,
    pub base: SettingsBase,
    pub device_name: PhiString,
    pub bright: f32,
    pub music_volume: f32,
    pub effect_volume: f32,
    pub hit_sound_volume: f32,
    pub sound_offset: f32,
    pub note_scale: f32,
}

impl Binary for Settings {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        let mut pos = 0;
        let version = data[pos];
        pos += 1;
        let base = SettingsBase::read(&data[pos..])?;
        pos += base.len();
        let device_name = PhiString::read(&data[pos..])?;
        pos += device_name.len();
        let bright = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let music_volume = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let effect_volume = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let hit_sound_volume = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let sound_offset = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let note_scale = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        Ok(Settings {
            version,
            base,
            device_name,
            bright,
            music_volume,
            effect_volume,
            hit_sound_volume,
            sound_offset,
            note_scale,
        })
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        let mut pos = 0;
        data[pos] = self.version;
        pos += 1;
        self.base.write(&mut data[pos..])?;
        pos += self.base.len();
        self.device_name.write(&mut data[pos..])?;
        pos += self.device_name.len();
        data[pos..pos + 4].copy_from_slice(&self.bright.to_le_bytes());
        pos += 4;
        data[pos..pos + 4].copy_from_slice(&self.music_volume.to_le_bytes());
        pos += 4;
        data[pos..pos + 4].copy_from_slice(&self.effect_volume.to_le_bytes());
        pos += 4;
        data[pos..pos + 4].copy_from_slice(&self.hit_sound_volume.to_le_bytes());
        pos += 4;
        data[pos..pos + 4].copy_from_slice(&self.sound_offset.to_le_bytes());
        pos += 4;
        data[pos..pos + 4].copy_from_slice(&self.note_scale.to_le_bytes());
        Ok(())
    }

    fn len(&self) -> usize {
        1 + 1 + self.device_name.len() + 4 * 6
    }
}
