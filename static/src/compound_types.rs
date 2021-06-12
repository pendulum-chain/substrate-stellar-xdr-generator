use sp_std::vec::Vec;

use crate::streams::{ReadStream, ReadStreamError, WriteStream, WriteStreamError};
use crate::xdr_codec::XdrCodec;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LimitedVarOpaque<const N: i32>(Vec<u8>);
impl<const N: i32> LimitedVarOpaque<N> {
    fn new(vec: Vec<u8>) -> Option<Self> {
        match vec.len() > N as usize {
            true => None,
            false => Some(LimitedVarOpaque(vec)),
        }
    }
}

impl<const N: i32> XdrCodec for LimitedVarOpaque<N> {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_u32(self.0.len() as u32);
        write_stream.write_next_binary_data(&self.0[..]);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        let length = read_stream.read_next_u32()? as i32;
        match length > N {
            true => Err(ReadStreamError::VarOpaqueExceedsMaxLength {
                at_position: read_stream.get_position(),
                max_length: N,
                actual_length: length,
            }),
            false => Ok(
                LimitedVarOpaque::new(read_stream.read_next_binary_data(length as usize)?).unwrap(),
            ),
        }
    }
}

#[allow(dead_code)]
pub type UnlimitedVarOpaque = LimitedVarOpaque<{ i32::MAX }>;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LimitedString<const N: i32>(Vec<u8>);
impl<const N: i32> LimitedString<N> {
    fn new(vec: Vec<u8>) -> Option<Self> {
        match vec.len() > N as usize {
            true => None,
            false => Some(LimitedString(vec)),
        }
    }
}

impl<const N: i32> XdrCodec for LimitedString<N> {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_u32(self.0.len() as u32);
        write_stream.write_next_binary_data(&self.0[..]);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        let length = read_stream.read_next_u32()? as i32;
        match length > N {
            true => Err(ReadStreamError::StringExceedsMaxLength {
                at_position: read_stream.get_position(),
                max_length: N,
                actual_length: length,
            }),
            false => Ok(
                LimitedString::new(read_stream.read_next_binary_data(length as usize)?).unwrap(),
            ),
        }
    }
}

#[allow(dead_code)]
pub type UnlimitedString = LimitedString<{ i32::MAX }>;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LimitedVarArray<T, const N: i32>(Vec<T>);
impl<T, const N: i32> LimitedVarArray<T, N> {
    fn new(vec: Vec<T>) -> Option<Self> {
        match vec.len() > N as usize {
            true => None,
            false => Some(LimitedVarArray(vec)),
        }
    }
}

impl<T: XdrCodec, const N: i32> XdrCodec for LimitedVarArray<T, N> {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_u32(self.0.len() as u32);
        for item in self.0.iter() {
            item.to_xdr_buffered(write_stream)?;
        }
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        let length = read_stream.read_next_u32()? as i32;
        match length > N {
            true => Err(ReadStreamError::VarArrayExceedsMaxLength {
                at_position: read_stream.get_position(),
                max_length: N,
                actual_length: length,
            }),
            false => {
                let mut result = Vec::<T>::with_capacity(length as usize);
                for _ in 0..length {
                    result.push(T::from_xdr_buffered(read_stream)?)
                }
                Ok(LimitedVarArray::new(result).unwrap())
            }
        }
    }
}

#[allow(dead_code)]
pub type UnlimitedVarArray<T> = LimitedVarArray<T, { i32::MAX }>;
