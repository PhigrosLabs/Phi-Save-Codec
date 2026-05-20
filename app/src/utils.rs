use alloc::string::String;
use alloc::string::ToString;
use core::fmt;
use serde::{Deserialize, Serialize};

use crate::Binary;

#[derive(Debug)]
pub enum BinaryError {
    NotEnoughData,
    InvalidUtf8,
}

impl fmt::Display for BinaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryError::NotEnoughData => write!(f, "not enough data"),
            BinaryError::InvalidUtf8 => write!(f, "invalid UTF-8 in string field"),
        }
    }
}

impl core::error::Error for BinaryError {}

#[inline(always)]
pub fn get_bit(byte: u8, index: usize) -> bool {
    ((byte >> index) & 1) != 0
}

#[inline(always)]
pub fn set_bit(byte: &mut u8, index: usize, value: bool) {
    if value {
        *byte |= 1 << index;
    } else {
        *byte &= !(1 << index);
    }
}

// --- VarInt: variable-length u16 (1 or 2 bytes, little-endian, MSB gate) ---

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VarInt(pub u16);

impl Binary for VarInt {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        let first = data[0];
        if first > 127 {
            if data.len() < 2 {
                return Err(BinaryError::NotEnoughData);
            }
            Ok(VarInt(((first & 0x7F) as u16) | ((data[1] as u16) << 7)))
        } else {
            Ok(VarInt(first as u16))
        }
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        if data.is_empty() {
            return Err(BinaryError::NotEnoughData);
        }
        if self.0 > 127 {
            if data.len() < 2 {
                return Err(BinaryError::NotEnoughData);
            }
            data[0] = ((self.0 & 0x7F) as u8) | 0x80;
            data[1] = (self.0 >> 7) as u8;
        } else {
            data[0] = self.0 as u8;
        }
        Ok(())
    }

    fn len(&self) -> usize {
        if self.0 > 127 { 2 } else { 1 }
    }
}

impl From<u16> for VarInt {
    fn from(v: u16) -> Self {
        VarInt(v)
    }
}
impl From<VarInt> for u16 {
    fn from(v: VarInt) -> Self {
        v.0
    }
}

// --- PhiString: length-prefixed UTF-8 string (VarInt length + bytes) ---

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhiString(pub String);

impl Binary for PhiString {
    type Error = BinaryError;

    fn read(data: &[u8]) -> Result<Self, Self::Error> {
        let vi = VarInt::read(data)?;
        let start = vi.len();
        let len = vi.0 as usize;
        if data.len() < start + len {
            return Err(BinaryError::NotEnoughData);
        }
        let s = String::from_utf8_lossy(&data[start..start + len]).to_string();
        Ok(PhiString(s))
    }

    fn write(&self, data: &mut [u8]) -> Result<(), Self::Error> {
        let vi = VarInt(self.0.len() as u16);
        vi.write(data)?;
        let start = vi.len();
        if data.len() < start + self.0.len() {
            return Err(BinaryError::NotEnoughData);
        }
        data[start..start + self.0.len()].copy_from_slice(self.0.as_bytes());
        Ok(())
    }

    fn len(&self) -> usize {
        VarInt(self.0.len() as u16).len() + self.0.len()
    }
}

impl From<String> for PhiString {
    fn from(v: String) -> Self {
        PhiString(v)
    }
}
impl From<PhiString> for String {
    fn from(v: PhiString) -> Self {
        v.0
    }
}
