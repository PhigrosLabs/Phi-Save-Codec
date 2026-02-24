use shua_struct::{BinaryError, BinaryField, BitField, BitSlice, Lsb0};

#[derive(Clone, Copy, Debug, Default)]
pub struct VarInt(pub u16);

impl BinaryField<Lsb0> for VarInt {
    type Error = BinaryError;

    #[inline]
    fn parse(bits: &BitSlice<u8, Lsb0>, _ctx: &()) -> Result<Self, Self::Error> {
        if bits.len() < 8 {
            return Err(Self::Error::bit_count_mismatch(8, bits.len()));
        }

        let first = bits[0..8].load_le::<u8>();

        if first > 127 {
            if bits.len() < 16 {
                return Err(Self::Error::bit_count_mismatch(16, bits.len()));
            }

            let second = bits[8..16].load_le::<u8>();
            let value = ((first & 0x7F) as u16) | ((second as u16) << 7);
            Ok(VarInt(value))
        } else {
            Ok(VarInt(first as u16))
        }
    }

    #[inline]
    fn build(&self, bits: &mut BitSlice<u8, Lsb0>, _ctx: &()) -> Result<(), Self::Error> {
        #[cfg(debug_assertions)]
        {
            let len = self.bit_len(_ctx);
            if bits.len() < len {
                return Err(Self::Error::bit_count_mismatch(len, bits.len()));
            }
        }

        if self.0 > 127 {
            let first = ((self.0 & 0x7F) as u8) | 0x80;
            let second = (self.0 >> 7) as u8;
            bits[0..8].store_le(first);
            bits[8..16].store_le(second);
        } else {
            bits[0..8].store_le(self.0 as u8);
        }

        Ok(())
    }

    #[inline]
    fn bit_len(&self, _ctx: &()) -> usize {
        if self.0 > 127 { 16 } else { 8 }
    }
}

impl From<VarInt> for usize {
    fn from(var: VarInt) -> Self {
        var.0 as usize
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

#[derive(Debug, Default)]
pub struct PhiString(pub String);

impl BinaryField<Lsb0> for PhiString {
    type Error = BinaryError;

    #[inline]
    fn parse(bits: &BitSlice<u8, Lsb0>, ctx: &()) -> Result<Self, Self::Error> {
        let varint = VarInt::parse(bits, ctx)?;
        let offset_bits = varint.bit_len(ctx);

        let length_bytes = varint.0 as usize;
        let length_bits = length_bytes * 8;

        let date_len = bits.len();
        if date_len < offset_bits + length_bits {
            return Err(Self::Error::bit_count_mismatch(
                offset_bits + length_bits,
                date_len,
            ));
        }

        let mut bytes = Vec::with_capacity(length_bytes);
        for i in 0..length_bytes {
            let start = offset_bits + i * 8;
            let end = start + 8;
            bytes.push(bits[start..end].load_le::<u8>());
        }

        let s = String::from_utf8_lossy(&bytes).to_string();
        Ok(PhiString(s))
    }

    #[inline]
    fn build(&self, bits: &mut BitSlice<u8, Lsb0>, ctx: &()) -> Result<(), Self::Error> {
        let bytes = self.0.as_bytes();
        let varint = VarInt(bytes.len() as u16);
        let varint_bits = varint.bit_len(ctx);

        #[cfg(debug_assertions)]
        {
            let required_bits = varint_bits + bytes.len() * 8;
            if bits.len() < required_bits {
                return Err(Self::Error::bit_count_mismatch(required_bits, bits.len()));
            }
        }
        varint.build(bits, ctx)?;
        for (i, byte) in bytes.iter().enumerate() {
            let start = varint_bits + i * 8;
            let end = start + 8;
            bits[start..end].store_le(*byte);
        }

        Ok(())
    }

    #[inline]
    fn bit_len(&self, ctx: &()) -> usize {
        let len = self.0.len();
        VarInt(len as u16).bit_len(ctx) + (len * 8)
    }
}

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
