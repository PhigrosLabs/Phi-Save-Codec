use shua_struct::{BinaryField, BitField, BitSlice, BitVec, Lsb0, Options};

#[derive(Debug, Default, Clone, Copy)]
pub struct VarInt(pub u16);

impl BinaryField<Lsb0> for VarInt {
    #[inline]
    fn parse(bits: &BitSlice<u8, Lsb0>, _opts: &Option<Options>) -> Result<Self, String> {
        if bits.len() < 8 {
            return Err("VarInt parse error: not enough bits".to_string());
        }

        let first = bits[0..8].load_le::<u8>();

        if first > 127 {
            if bits.len() < 16 {
                return Err("VarInt parse error: not enough bits for two-byte VarInt".to_string());
            }

            let second = bits[8..16].load_le::<u8>();
            let value = ((first & 0x7F) as u16) | ((second as u16) << 7);
            Ok(VarInt(value))
        } else {
            Ok(VarInt(first as u16))
        }
    }

    #[inline]
    fn build(&self, _opts: &Option<Options>) -> Result<BitVec<u8>, String> {
        let mut bv = BitVec::new();

        if self.0 > 127 {
            let first = ((self.0 & 0x7F) as u8) | 0x80;
            let second = (self.0 >> 7) as u8;
            bv.extend_from_raw_slice(&[first, second]);
        } else {
            bv.extend_from_raw_slice(&[self.0 as u8]);
        }

        Ok(bv)
    }

    fn bit_len(&self, _opts: &Option<Options>) -> usize {
        if self.0 > 127 { 16 } else { 8 }
    }
}

impl From<VarInt> for usize {
    fn from(var: VarInt) -> Self {
        var.0 as usize
    }
}

#[derive(Debug, Default)]
pub struct PhiString(pub String);
impl BinaryField<Lsb0> for PhiString {
    #[inline]
    fn parse(bits: &BitSlice<u8, Lsb0>, opts: &Option<Options>) -> Result<Self, String> {
        let varint = VarInt::parse(bits, opts)?;
        let offset_bits = varint.bit_len(opts);

        let length_bytes = varint.0 as usize;
        let length_bits = length_bytes
            .checked_mul(8)
            .ok_or_else(|| "String parse error: length overflow".to_string())?;

        if bits.len() < offset_bits + length_bits {
            return Err("String parse error: not enough bits".to_string());
        }

        let mut bytes = Vec::with_capacity(length_bytes);
        for i in 0..length_bytes {
            let start = offset_bits + i * 8;
            let end = start + 8;
            bytes.push(bits[start..end].load_le::<u8>());
        }

        let s = std::str::from_utf8(&bytes)
            .map_err(|e| format!("String parse error: {}, raw: {:02X?}", e, bytes))?;

        Ok(PhiString(s.to_string()))
    }

    #[inline]
    fn build(&self, opts: &Option<Options>) -> Result<BitVec<u8>, String> {
        let bytes = self.0.as_bytes();
        let mut bv = VarInt(bytes.len() as u16).build(opts)?;
        bv.extend_from_raw_slice(bytes);
        Ok(bv)
    }

    #[inline]
    fn bit_len(&self, opts: &Option<Options>) -> usize {
        let len = self.0.len();
        VarInt(len as u16).bit_len(opts) + (len * 8)
    }
}

// <->
impl From<String> for PhiString {
    fn from(s: String) -> Self {
        PhiString(s)
    }
}

impl From<&str> for PhiString {
    fn from(s: &str) -> Self {
        PhiString(s.to_string())
    }
}

impl From<PhiString> for String {
    fn from(phi: PhiString) -> Self {
        phi.0
    }
}

impl From<u16> for VarInt {
    fn from(value: u16) -> Self {
        VarInt(value)
    }
}

impl From<VarInt> for u16 {
    fn from(varint: VarInt) -> Self {
        varint.0
    }
}
