//! An encoder and decoder for [Stellar](https://www.stellar.org/) XDR types
//!
//! This crate is compatible with [Substrate](https://www.substrate.io/) and uses
//! `sp_std` instead of `std`.

#![no_std]

pub mod compound_types;
pub mod streams;
pub mod xdr;
pub mod xdr_codec;
