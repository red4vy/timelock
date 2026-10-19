# timelock

> A native Solana program that locks native SOL until a user-defined Unix timestamp.

---

## TL;DR

- **What:** A Solana program that lets an authority deposit SOL into a vault and withdraw it only after `unlock_timestamp`.
- **Status:** v1, native SOL only. SPL Token support planned for v2.
- **Audited:** No. Use at your own risk.
- **Program ID:**: `TLnxmTpQfoeuPKqJwWeBp9qeg2igUqh2UbrhMZiX53h`.

## ⚠️ Disclaimer

This code has **not been audited** by a third-party. Use at your own risk.

## Quick start

Install:
```bash
git clone https://github.com/red4vy/timelock.git
cd timelock
```

Build the program:
```bash
cargo build-sbf
```

Run the test suite:
```bash
cargo test
```

## License

Licensed under the Apache License, Version 2.0. See [`LICENSE`](./LICENSE).
