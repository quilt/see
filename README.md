# SEE - Simple EE
This EE demonstrate balance transfers in ETH 2.  The EE code can be run natively or in eWasm.

### Build
```bash
$ cargo build
```

### Run EE code target for native platform
```bash
$ cargo run
```

### Run EE code target for wasm
```bash
$ cargo test
```

To enable printouts:
```bash
$ cargo test -- --nocapture
```

> Note:  The project is currently configured to run the EE targeted for eWasm only during integration tests.