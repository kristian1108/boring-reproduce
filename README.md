# Summary
This repo demonstrates an issue somewhere in the interaction
between TLS & the main PKCS11 library in Rust.

# Dependencies
```shell
nix-env -iA nixpkgs.opensc
```

# Run
```shell
cargo run
```

# Sample Output
```shell
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/boring-repro`
[1]    25058 segmentation fault  cargo run
```

# Versions
`rustc 1.75.0 (82e1608df 2023-12-21)`

`mac os 14.1.1 (23B81)`

