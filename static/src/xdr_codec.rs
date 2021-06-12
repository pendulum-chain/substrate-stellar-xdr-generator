use core::convert::TryInto;
use sp_std::{boxed::Box, vec::Vec};

use crate::streams::{ReadStream, ReadStreamError, WriteStream, WriteStreamError};

pub trait XdrCodec: Sized {
    fn to_xdr(&self) -> Result<Vec<u8>, WriteStreamError> {
        let mut write_stream = WriteStream::new();
        self.to_xdr_buffered(&mut write_stream)?;
        Ok(write_stream.get_result())
    }

    fn from_xdr(buffer: &Vec<u8>) -> Result<Self, ReadStreamError> {
        let mut read_stream = ReadStream::new(buffer);
        let value = Self::from_xdr_buffered(&mut read_stream)?;
        if !read_stream.no_of_bytes_left_to_read() != 0 {
            return Err(ReadStreamError::TypeEndsTooEarly {
                missing_no_of_bytes: read_stream.no_of_bytes_left_to_read(),
            });
        }

        Ok(value)
    }

    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError>;

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError>;
}

impl XdrCodec for u64 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_u64(*self);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        read_stream.read_next_u64()
    }
}

impl XdrCodec for i64 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_i64(*self);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        read_stream.read_next_i64()
    }
}

impl XdrCodec for u32 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_u32(*self);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        read_stream.read_next_u32()
    }
}

impl XdrCodec for i32 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_i32(*self);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        read_stream.read_next_i32()
    }
}

impl XdrCodec for bool {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_i32(if *self { 1 } else { 0 });
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        let parsed_int = read_stream.read_next_i32()?;
        match parsed_int {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ReadStreamError::InvalidBoolean {
                found_integer: parsed_int,
            }),
        }
    }
}

impl XdrCodec for () {
    fn to_xdr_buffered(&self, _write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        Ok(())
    }

    fn from_xdr_buffered(_read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        Ok(())
    }
}

impl<T: XdrCodec, const N: usize> XdrCodec for [T; N] {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        for item in self.iter() {
            item.to_xdr_buffered(write_stream)?;
        }
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        let mut result = Vec::<T>::with_capacity(N);
        for _ in 0..N {
            result.push(T::from_xdr_buffered(read_stream)?)
        }
        result.try_into().map_err(|_| unreachable!())
    }
}

impl<const N: usize> XdrCodec for [u8; N] {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_binary_data(self);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        let value = read_stream.read_next_binary_data(N)?;
        value.try_into().map_err(|_| unreachable!())
    }
}

impl<T: XdrCodec> XdrCodec for Option<T> {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        match self {
            None => write_stream.write_next_u32(0),
            Some(value) => {
                write_stream.write_next_u32(1);
                value.to_xdr_buffered(write_stream)?
            }
        }
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        match read_stream.read_next_u32()? {
            0 => Ok(None),
            1 => T::from_xdr_buffered(read_stream).map(|ok| Some(ok)),
            code => Err(ReadStreamError::InvalidOptional {
                at_position: read_stream.get_position(),
                has_code: code,
            }),
        }
    }
}

impl<T: XdrCodec> XdrCodec for Box<T> {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        self.as_ref().to_xdr_buffered(write_stream)
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        Ok(Box::new(T::from_xdr_buffered(read_stream)?))
    }
}
