use crate::phi_base::*;
use shua_struct::BinaryField;

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct ProgressBase {
    pub is_first_run: bool,
    pub legacy_chapter_finished: bool,
    pub already_show_collection_tip: bool,
    pub already_show_auto_unlock_in_tip: bool,
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct Money {
    pub kib: VarInt,
    pub mib: VarInt,
    pub gib: VarInt,
    pub tib: VarInt,
    pub pib: VarInt,
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct Chapter8Base {
    pub unlock_begin: bool,
    pub unlock_second_phase: bool,
    pub passed: bool,
}

#[derive(Debug, Default, BinaryField)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct GameProgress {
    pub version: u8,
    #[binary_field(align = 8)]
    pub base: ProgressBase,
    pub completed: PhiString,
    pub song_update_info: VarInt,
    pub challenge_mode_rank: u16,
    pub money: Money,
    #[binary_field(align = 8)]
    pub unlock_flag_of_spasmodic: [bool; 4],
    #[binary_field(align = 8)]
    pub unlock_flag_of_igallta: [bool; 4],
    #[binary_field(align = 8)]
    pub unlock_flag_of_rrharil: [bool; 4],
    pub flag_of_song_record_key: [bool; 8], // is_version_at_least_1
    #[binary_field(align = 8, if_func = "is_version_at_least_2")]
    pub random_version_unlocked: [bool; 6],
    #[binary_field(align = 8, if_func = "is_version_at_least_3")]
    pub chapter8_base: Chapter8Base,
    #[binary_field(align = 8, if_func = "is_version_at_least_3")]
    pub chapter8_song_unlocked: [bool; 6],
    #[binary_field(align = 8, if_func = "is_version_at_least_4")]
    pub flag_of_song_record_key_takumi: Option<[bool; 3]>,
}

impl GameProgress {
    fn is_version_at_least_4(&self) -> bool {
        self.version >= 4
    }

    fn is_version_at_least_3(&self) -> bool {
        self.version >= 3
    }

    fn is_version_at_least_2(&self) -> bool {
        self.version >= 2
    }
}
