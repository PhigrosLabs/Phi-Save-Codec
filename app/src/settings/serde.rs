use super::field::{Settings, SettingsBase};
use crate::phi_base::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableSettingsBase {
    pub chord_support: bool,
    pub fc_ap_indicator: bool,
    pub enable_hit_sound: bool,
    pub low_resolution_mode: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableSettings {
    pub version: u8,
    pub base: SerializableSettingsBase,
    pub device_name: String,
    pub bright: f32,
    pub music_volume: f32,
    pub effect_volume: f32,
    pub hit_sound_volume: f32,
    pub sound_offset: f32,
    pub note_scale: f32,
}

impl From<SettingsBase> for SerializableSettingsBase {
    fn from(s: SettingsBase) -> Self {
        Self {
            chord_support: s.chord_support,
            fc_ap_indicator: s.fc_ap_indicator,
            enable_hit_sound: s.enable_hit_sound,
            low_resolution_mode: s.low_resolution_mode,
        }
    }
}

impl From<SerializableSettingsBase> for SettingsBase {
    fn from(s: SerializableSettingsBase) -> Self {
        SettingsBase {
            chord_support: s.chord_support,
            fc_ap_indicator: s.fc_ap_indicator,
            enable_hit_sound: s.enable_hit_sound,
            low_resolution_mode: s.low_resolution_mode,
        }
    }
}

impl From<Settings> for SerializableSettings {
    fn from(s: Settings) -> Self {
        Self {
            version: s.version,
            base: s.base.into(),
            device_name: s.device_name.0,
            bright: s.bright,
            music_volume: s.music_volume,
            effect_volume: s.effect_volume,
            hit_sound_volume: s.hit_sound_volume,
            sound_offset: s.sound_offset,
            note_scale: s.note_scale,
        }
    }
}

impl From<SerializableSettings> for Settings {
    fn from(s: SerializableSettings) -> Self {
        Settings {
            version: s.version,
            base: s.base.into(),
            device_name: PhiString(s.device_name),
            bright: s.bright,
            music_volume: s.music_volume,
            effect_volume: s.effect_volume,
            hit_sound_volume: s.hit_sound_volume,
            sound_offset: s.sound_offset,
            note_scale: s.note_scale,
        }
    }
}
