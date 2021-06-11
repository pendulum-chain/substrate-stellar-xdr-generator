use sp_std::vec::Vec;

use crate::streams::{ReadStream, ReadStreamError, WriteStream, WriteStreamError};

pub trait XdrDecode: Sized {
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

    fn is_valid(&self) -> bool;
}

impl XdrDecode for u64 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_u64(*self);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        read_stream.read_next_u64()
    }

    fn is_valid(&self) -> bool {
        true
    }
}

impl XdrDecode for i64 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_i64(*self);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        read_stream.read_next_i64()
    }

    fn is_valid(&self) -> bool {
        true
    }
}

impl XdrDecode for u32 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_u32(*self);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        read_stream.read_next_u32()
    }

    fn is_valid(&self) -> bool {
        true
    }
}

impl XdrDecode for i32 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        write_stream.write_next_i32(*self);
        Ok(())
    }

    fn from_xdr_buffered(read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        read_stream.read_next_i32()
    }

    fn is_valid(&self) -> bool {
        true
    }
}

impl XdrDecode for bool {
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

    fn is_valid(&self) -> bool {
        true
    }
}

impl XdrDecode for () {
    fn to_xdr_buffered(&self, _write_stream: &mut WriteStream) -> Result<(), WriteStreamError> {
        Ok(())
    }

    fn from_xdr_buffered(_read_stream: &mut ReadStream) -> Result<Self, ReadStreamError> {
        Ok(())
    }

    fn is_valid(&self) -> bool {
        true
    }
}
