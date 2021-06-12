# Generator of the no_std crate for encoding and decoding Stellar XDR types

This repository generates a Rust decoder and encoder of all XDR types used in Stellar.

## Usage

```javascript
const { TransactionEnvelope } = require("ts-stellar-xdr");

const transactionEnvelope =
  "AAAAAJM++/BQ/J83ai5alxXDK/s5oNhYQPtYDq4VtLf7qc9eAAAAZAEK1kwAAAACAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAzMnJ6" +
  "nCpdtk2mZPKKIJ9GTynIxfP58O0cQnrpz9ukBsAAAAF9nmWgAAAAAAAAAAB+6nPXgAAAEBKCwRLujMDdruWlHGpvcBYaVKqUDGbpH" +
  "ifZ7bjGmrCs7cldblBe2ZI7AGMC79QQr6peR/jf/HOSDwkXYWJczMH";

const transactionEnvelopeArrayBuffer = base64Decode(transactionEnvelope); // for some base64 decoding function

const transaction = TransactionEnvelope.fromXdr(transactionEnvelopeArrayBuffer);
console.log(transaction);

const encodedTransactionEnvelope = base64Encode(TransactionEnvelope.toXdr(transaction));

console.log(encodedTransactionEnvelope === transactionEnvelope); // true
```

## Developers

How to run locally

### Preparation

```
  npm install
```

### Build Rust XDR serializer/deserializer

```
  npm run build-complete
```

# Assumptions

- this code assumes that the only way cycles in types can occur is if an enum or struct type directly references itself (no indirect cycles)
  - otherwise we need to have `Box` way too often and types get cluttered
  - this assumption is currently true as the only cycle occurs in the type `ClaimPredicate` – and that cycle is direct
  - this assumption might not hold true in the future anymore
    - in that case start to introduce `Box` everytime an enum or struct references a non-primitive type that is not wrapped into a `Vec`
