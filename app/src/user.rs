use crate::Binary;
use crate::utils::{BinaryError, PhiString};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub version: u8,
    pub show_player_id: bool,
    pub self_intro: PhiString,
    pub avatar: PhiString,
    pub background: PhiString,
}

impl Binary for User {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        let mut pos = 0;
        if data.len() < 2 {
            return Err(BinaryError::NotEnoughData);
        }
        let version = data[pos];
        pos += 1;
        let show_player_id = (data[pos] & 1) != 0;
        pos += 1;
        let self_intro = PhiString::read(&data[pos..])?;
        pos += self_intro.len();
        let avatar = PhiString::read(&data[pos..])?;
        pos += avatar.len();
        let background = PhiString::read(&data[pos..])?;
        Ok(User {
            version,
            show_player_id,
            self_intro,
            avatar,
            background,
        })
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        let mut pos = 0;
        data[pos] = self.version;
        pos += 1;
        data[pos] = if self.show_player_id { 1 } else { 0 };
        pos += 1;
        self.self_intro.write(&mut data[pos..])?;
        pos += self.self_intro.len();
        self.avatar.write(&mut data[pos..])?;
        pos += self.avatar.len();
        self.background.write(&mut data[pos..])?;
        Ok(())
    }

    fn len(&self) -> usize {
        2 + self.self_intro.len() + self.avatar.len() + self.background.len()
    }
}
