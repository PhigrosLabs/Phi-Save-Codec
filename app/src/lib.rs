#![no_std]

extern crate alloc;

pub mod types;
pub mod utils;

pub mod game_key;
pub mod game_progress;
pub mod game_record;
pub mod settings;
pub mod summary;
pub mod user;

pub trait Binary {
    type Error: core::error::Error;

    fn read(data: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error>;

    fn len(&self) -> usize;
}

pub use crate::game_key::GameKey;
pub use crate::game_progress::GameProgress;
pub use crate::game_record::GameRecord;
pub use crate::settings::Settings;
pub use crate::summary::Summary;
pub use crate::user::User;
