//! BLE UUIDs (16, 32 or 128 bits).
//!
//! Bluetooth assigns UUID to identify services and characteristics. In order to
//! save space, many common UUIDs can be represented and transmitted as 16- or
//! 32-bit aliases instead of the full 128 bits.
//!
//! The shorter UUIDs can be converted to their full 128-bit counterparts by
//! making use of the so-called Bluetooth Base UUID:
//!
//! `00000000-0000-1000-8000-00805F9B34FB`
//!
//! A 16-bit UUID alias can be converted to its 32-bit equivalent by
//! zero-extending it: `0xABCD` becomes `0x0000ABCD`.
//!
//! A 32-bit UUID alias can then be converted to its full 128-bit equivalent by
//! placing it in the first 4 Bytes of the Base UUID. Hence `0x1234ABCD` would
//! become:
//!
//! `1234ABCD-0000-1000-8000-00805F9B34FB`

use {
    crate::ble::{
        utils::{FromBytes, MutSliceExt, SliceExt, ToBytes},
        Error,
    },
    byteorder::{BigEndian, ByteOrder},
};

pub use uuid::Uuid;

// FIXME the uuid crate should offer a const fn `from_u128`
const BASE_UUID: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, /*-*/ 0x00, 0x00, /*-*/ 0x10, 00, /*-*/ 0x80, 0x00,
    /*-*/ 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB,
];

/// A 16-bit UUID alias.
///
/// Can be converted to its 32- and 128-bit equivalents via `.into()`.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Uuid16(pub u16);

/// A 32-bit UUID alias.
///
/// Can be converted to its 128-bit equivalent via `.into()`.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Uuid32(pub u32);

impl From<Uuid16> for Uuid32 {
    fn from(smol: Uuid16) -> Self {
        Uuid32(smol.0.into())
    }
}

impl Into<Uuid> for Uuid16 {
    fn into(self) -> Uuid {
        Uuid32::from(self).into()
    }
}

impl Into<Uuid> for Uuid32 {
    fn into(self) -> Uuid {
        let mut buf = BASE_UUID;
        BigEndian::write_u32(&mut buf, self.0);
        Uuid::from_bytes(buf)
    }
}

impl ToBytes for Uuid16 {
    fn space_needed(&self) -> usize {
        2
    }

    fn to_bytes(&self, buffer: &mut &mut [u8]) -> Result<(), Error> {
        buffer.write_slice(&self.0.to_be_bytes())
    }
}

impl ToBytes for Uuid32 {
    fn space_needed(&self) -> usize {
        4
    }

    fn to_bytes(&self, buffer: &mut &mut [u8]) -> Result<(), Error> {
        buffer.write_slice(&self.0.to_be_bytes())
    }
}

impl ToBytes for Uuid {
    fn space_needed(&self) -> usize {
        16
    }

    fn to_bytes(&self, buffer: &mut &mut [u8]) -> Result<(), Error> {
        buffer.write_slice(self.as_bytes())
    }
}

impl FromBytes for Uuid16 {
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self, Error> {
        let array = bytes.read_array().ok_or(Error::Eof)?;
        Ok(Uuid16(u16::from_be_bytes(array)))
    }
}

impl FromBytes for Uuid32 {
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self, Error> {
        let array = bytes.read_array().ok_or(Error::Eof)?;
        Ok(Uuid32(u32::from_be_bytes(array)))
    }
}

impl FromBytes for Uuid {
    fn from_bytes(bytes: &mut &[u8]) -> Result<Self, Error> {
        let array = bytes.read_array().ok_or(Error::Eof)?;
        Ok(Uuid::from_bytes(array))
    }
}

/// Marker for UUID types.
///
/// This is useful when being generic over the specific type of UUID used. It
/// also brings in the `ToBytes` and `FromBytes` trait bounds that are likely
/// needed as well.
pub trait IsUuid: ToBytes + FromBytes + Copy {}

impl IsUuid for Uuid16 {}
impl IsUuid for Uuid32 {}
impl IsUuid for Uuid {}