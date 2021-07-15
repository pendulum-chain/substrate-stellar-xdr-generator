# DEPRECATED

This crate is deprecated, it is now part of the [Substrate Stellar SDK](https://crates.io/crates/substrate-stellar-sdk).

# A coder/decoder of Stellar XDR types

This repository generates a Rust decoder and encoder of all XDR types used in Stellar. The code is generated via the [Substrate Stellar XDR generator](`https://github.com/pendulum-chain/substrate-stellar-xdr-generator`).

## Usage

This is example code to decode and encode a Stellar transaction.

```rust
use substrate_stellar_xdr::{xdr, xdr_codec::XdrCodec};

const ENVELOPE: &str =
    "AAAAAgAAAAC9xFYU1gQJeH4apEfzJkMCsW5DL4GEWRpyVjQHOlWVzgAAAZA\
    CGsQoAAQytgAAAAAAAAAAAAAAAgAAAAAAAAADAAAAAVhMUEcAAAAAxxJMrxQQOx9raxDm3\
    lINsLvksi7tj1BCQXzWTtqigbgAAAAAAAAAAAbK5N8CprKDAExLQAAAAAAAAAAAAAAAAAA\
    AAAMAAAAAAAAAAVhMUEcAAAAAxxJMrxQQOx9raxDm3lINsLvksi7tj1BCQXzWTtqigbgAA\
    AAAlV2+xQAEaBMAJiWgAAAAAAAAAAAAAAAAAAAAATpVlc4AAABAaX11e1dGcDkXrFT5s3Q\
    N6x3v4kQqJ/1VIjqO00y6OStd70/aYiXR35e4289RvmBTudJ5Q05PaRsD8p1qa17VDQ==";

fn main() {
    let envelope = xdr::TransactionEnvelope::from_base64_xdr(ENVELOPE).unwrap();
    println!("{:#?}", envelope);
    assert_eq!(xdr, envelope.to_xdr());
}
```

All Stellar XDR types are defined in the module `xdr`. Each type implements the `xdr_codex::XdrCodec` trait, which defines the following two useful methods:

- `fn to_xdr(&self) -> Vec<u8>`: encode as binary XDR
- `fn to_base64_xdr(&self) -> Vec<u8>`: encode as XDR, afterwards encode result as base64
- `from_xdr<T: AsRef<[u8]>>(input: T) -> Result<Self, DecodeError>`: decode binary XDR
- `from_base64_xdr<T: AsRef<[u8]>>(input: T) -> Result<Self, DecodeError>`: decode as base64, then decode result as XDR

### Features

- `all-types`: when specified, will generate all types – otherwise only those types are generated that are in the dependency tree of the types `TransactionEnvelope`, `TransactionResult`, `TransactionMeta`, `EnvelopeType` and `TransactionSignaturePayload`
