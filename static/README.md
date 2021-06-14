# A coder/decoder of Stellar XDR types

This repository generates a Rust decoder and encoder of all XDR types used in Stellar. The code is generated through the project `https://github.com/Pendulum-Chain/substrate-stellar-xdr-generator`.

## Usage

This is example code to decode and encode a Stellar transaction.

```
use substrate_stellar_xdr::{xdr, xdr_codec::XdrCodec};

const ENVELOPE: &str = "AAAAAgAAAAC9xFYU1gQJeH4apEfzJkMCsW5DL4GEWRpyVjQHOlWVzgAAAZA\
CGsQoAAQytgAAAAAAAAAAAAAAAgAAAAAAAAADAAAAAVhMUEcAAAAAxxJMrxQQOx9raxDm3\
lINsLvksi7tj1BCQXzWTtqigbgAAAAAAAAAAAbK5N8CprKDAExLQAAAAAAAAAAAAAAAAAA\
AAAMAAAAAAAAAAVhMUEcAAAAAxxJMrxQQOx9raxDm3lINsLvksi7tj1BCQXzWTtqigbgAA\
AAAlV2+xQAEaBMAJiWgAAAAAAAAAAAAAAAAAAAAATpVlc4AAABAaX11e1dGcDkXrFT5s3Q\
N6x3v4kQqJ/1VIjqO00y6OStd70/aYiXR35e4289RvmBTudJ5Q05PaRsD8p1qa17VDQ==";

fn main() {
    let xdr = base64::decode(ENVELOPE).unwrap();
    let envelope = xdr::TransactionEnvelope::from_xdr(&xdr).unwrap();
    println!("{:#?}", envelope);
    assert_eq!(xdr, envelope.to_xdr().unwrap());
}
```

All Stellar XDR types are defined in the module `xdr`. Each type implements the `xdr_codex::XdrCodec` trait, which defines the following two useful methods:

- `fn to_xdr(&self) -> Result<Vec<u8>, WriteStreamError>`
- `from_xdr(buffer: &Vec<u8>) -> Result<Self, ReadStreamError>`

### Features

- `main-types-only`: when specified, will only export the main types (Transactions, Operations, etc.)
