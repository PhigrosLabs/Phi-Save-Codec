use crate::Binary;
use crate::utils::{self, BinaryError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Difficulty4 {
    pub ez: bool,
    pub hd: bool,
    #[serde(rename = "in")]
    pub r#in: bool,
    pub at: bool,
}

impl Binary for Difficulty4 {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let b = data[0];
        Ok(Difficulty4 {
            ez: utils::get_bit(b, 0),
            hd: utils::get_bit(b, 1),
            r#in: utils::get_bit(b, 2),
            at: utils::get_bit(b, 3),
        })
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let mut b = 0u8;
        utils::set_bit(&mut b, 0, self.ez);
        utils::set_bit(&mut b, 1, self.hd);
        utils::set_bit(&mut b, 2, self.r#in);
        utils::set_bit(&mut b, 3, self.at);
        data[0] = b;
        Ok(())
    }

    fn len(&self) -> usize {
        1
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Difficulty5 {
    pub ez: bool,
    pub hd: bool,
    #[serde(rename = "in")]
    pub r#in: bool,
    pub at: bool,
    pub legacy: bool,
}

impl Difficulty5 {
    pub fn iter(&self) -> impl Iterator<Item = bool> {
        [self.ez, self.hd, self.r#in, self.at, self.legacy].into_iter()
    }

    pub fn count_unlocked(&self) -> usize {
        self.iter().filter(|&b| b).count()
    }
}

impl Binary for Difficulty5 {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let b = data[0];
        Ok(Difficulty5 {
            ez: utils::get_bit(b, 0),
            hd: utils::get_bit(b, 1),
            r#in: utils::get_bit(b, 2),
            at: utils::get_bit(b, 3),
            legacy: utils::get_bit(b, 4),
        })
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let mut b = 0u8;
        utils::set_bit(&mut b, 0, self.ez);
        utils::set_bit(&mut b, 1, self.hd);
        utils::set_bit(&mut b, 2, self.r#in);
        utils::set_bit(&mut b, 3, self.at);
        utils::set_bit(&mut b, 4, self.legacy);
        data[0] = b;
        Ok(())
    }

    fn len(&self) -> usize {
        1
    }
}
