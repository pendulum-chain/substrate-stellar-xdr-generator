use core::convert::TryFrom;
use core::convert::TryInto;
use core::iter;

use sp_std::vec::Vec;

fn extend_to_multiple_of_4(value: usize) -> usize {
    (value + 3) & !3
}

pub enum ReadStreamError {
    SuddenEnd {
        at_position: usize,
        expected_length: usize,
    },
    TypeEndsTooEarly {
        missing_no_of_bytes: isize,
    },
    InvalidBoolean {
        found_integer: i32,
    },
    VarOpaqueExceedsMaxLength {
        at_position: usize,
        max_length: i32,
        actual_length: i32,
    },
    StringExceedsMaxLength {
        at_position: usize,
        max_length: i32,
        actual_length: i32,
    },
    VarArrayExceedsMaxLength {
        at_position: usize,
        max_length: i32,
        actual_length: i32,
    },
    InvalidOptional {
        at_position: usize,
        has_code: u32,
    },
    InvalidEnumDiscriminator {
        at_position: usize,
    },
}

pub struct ReadStream<'a> {
    read_index: usize,
    source: &'a Vec<u8>,
}

impl<'a> ReadStream<'a> {
    pub fn new(source: &Vec<u8>) -> ReadStream {
        ReadStream {
            read_index: 0,
            source,
        }
    }

    fn ensure_size(&self, no_of_bytes_to_read: usize) -> Result<(), ReadStreamError> {
        if no_of_bytes_to_read + self.read_index > self.source.len() {
            return Err(self.generate_sudden_end_error(no_of_bytes_to_read));
        }
        Ok(())
    }

    fn generate_sudden_end_error(&self, no_of_bytes_to_read: usize) -> ReadStreamError {
        ReadStreamError::SuddenEnd {
            at_position: self.source.len(),
            expected_length: no_of_bytes_to_read + self.read_index,
        }
    }

    fn read_next_byte_array<const N: usize>(&mut self) -> Result<&[u8; N], ReadStreamError> {
        let array: Result<&[u8; N], _> =
            (&self.source[self.read_index..self.read_index + N]).try_into();

        match array {
            Ok(array) => {
                self.read_index += N;
                Ok(array)
            }
            Err(_) => Err(self.generate_sudden_end_error(N)),
        }
    }

    pub fn read_next_u32(&mut self) -> Result<u32, ReadStreamError> {
        let array: &[u8; 4] = self.read_next_byte_array()?;
        Ok(u32::from_be_bytes(*array))
    }

    pub fn read_next_i32(&mut self) -> Result<i32, ReadStreamError> {
        let array: &[u8; 4] = self.read_next_byte_array()?;
        Ok(i32::from_be_bytes(*array))
    }

    pub fn read_next_u64(&mut self) -> Result<u64, ReadStreamError> {
        let array: &[u8; 8] = self.read_next_byte_array()?;
        Ok(u64::from_be_bytes(*array))
    }

    pub fn read_next_i64(&mut self) -> Result<i64, ReadStreamError> {
        let array: &[u8; 8] = self.read_next_byte_array()?;
        Ok(i64::from_be_bytes(*array))
    }

    pub fn read_next_binary_data(
        &mut self,
        no_of_bytes: usize,
    ) -> Result<Vec<u8>, ReadStreamError> {
        self.ensure_size(extend_to_multiple_of_4(no_of_bytes))?;
        let result = self.source[self.read_index..self.read_index + no_of_bytes].to_vec();
        self.read_index += extend_to_multiple_of_4(no_of_bytes);
        Ok(result)
    }

    pub fn no_of_bytes_left_to_read(&self) -> isize {
        self.source.len() as isize - self.read_index as isize
    }

    pub fn get_position(&self) -> usize {
        self.read_index
    }
}

pub struct WriteStream {
    result: Vec<u8>,
}

pub enum WriteStreamError {
    StringTooLong { string_length: usize },
}

impl WriteStream {
    pub fn new() -> WriteStream {
        WriteStream {
            result: Vec::with_capacity(128),
        }
    }

    pub fn write_next_u32(&mut self, value: u32) {
        self.result.extend(value.to_be_bytes().iter());
    }

    pub fn write_next_i32(&mut self, value: i32) {
        self.result.extend(value.to_be_bytes().iter());
    }

    pub fn write_next_u64(&mut self, value: u64) {
        self.result.extend(value.to_be_bytes().iter());
    }

    pub fn write_next_i64(&mut self, value: i64) {
        self.result.extend(value.to_be_bytes().iter());
    }

    pub fn write_next_binary_data(&mut self, value: &[u8]) {
        self.result.extend_from_slice(value);
        let length = value.len();
        let no_of_padding_bytes = extend_to_multiple_of_4(length) - length;
        self.result
            .extend(iter::repeat(0).take(no_of_padding_bytes));
    }

    pub fn get_result(self) -> Vec<u8> {
        self.result
    }
}

pub fn is_valid_string(value: &str, max_no_of_bytes: u32) -> bool {
    match u32::try_from(value.len()) {
        Ok(length) => length <= max_no_of_bytes,
        Err(_) => false,
    }
}
