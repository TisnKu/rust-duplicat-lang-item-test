# Test command
cargo test --target target.json -Z build-std=core --no-run

Error:
```
error: duplicate lang item in crate `core` (which `rustc_std_workspace_core` depends on): `sized`.
  |
  = note: the lang item is first defined in crate `core` (which `foo` depends on)
  = note: first definition in `core` loaded from /Users/txku/projects/foo/target/target/debug/deps/libcore-537b20fcb96fe124.rlib
  = note: second definition in `core` loaded from /Users/txku/projects/foo/target/target/debug/deps/libcore-82e4bc7133bc00e1.rlib, /Users/txku/projects/foo/target/target/debug/deps/libcore-82e4bc7133bc00e1.rmeta
...
```

# rustc +nightly -V
rustc 1.62.0-nightly (8bf93e9b6 2022-04-09)

# cargo -V
cargo 1.62.0-nightly (e2e2dddeb 2022-04-05)

# rustup show
Default host: aarch64-apple-darwin
rustup home:  /Users/txku/.rustup

installed toolchains
--------------------

stable-aarch64-apple-darwin (default)
nightly-aarch64-apple-darwin

installed targets for active toolchain
--------------------------------------

aarch64-apple-darwin
thumbv7em-none-eabihf

active toolchain
----------------

nightly-aarch64-apple-darwin (directory override for '/Users/txku/projects/foo')
rustc 1.62.0-nightly (8bf93e9b6 2022-04-09)