#![no_std]

extern crate alloc;

mod compound_types;
mod streams;
mod types;
mod xdr_codec;

fn main() {
    let a = sp_std::vec![2u32, 3];
    //println!("{} Hello, world!", a);
}
