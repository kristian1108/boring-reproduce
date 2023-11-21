# Summary
This repo demonstrates an issue somewhere in the interaction
between TLS & the main PKCS11 library in Rust.

# Dependencies
```shell
brew install softhsm
```

# Run
```shell
cargo run
```

# Sample Output
```shell
➜  boring-repro cargo run
   Compiling boring-repro v0.1.0 (/Users/.../code/boring-repro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/boring-repro`
Assertion failed: (ret == 0), function digest_final, file boringssl_crypto_digests.m, line 41.
[1]    34079 abort      cargo run

```

# Versions
`rustc 1.74.0 (79e9716c9 2023-11-13)`

`softhsm2-util 2.6.1`

`mac os 14.1.1 (23B81)`

