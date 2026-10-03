# nohasher

No-op hasher for using integer-ish keys in `HashMap`-like structures.

## Rust version

The minimum supported Rust version (MSRV) is **1.100**, targeting the stable
Rust 1.100.0 release. The crate has been validated locally on Rust 1.100 beta;
no beta toolchain is pinned or required by this repository.

Use `cargo check`, `cargo clippy`, and `cargo test` with a toolchain meeting the
MSRV. Use `--no-default-features` to check the `no_std` build. Formatting still
requires a locally selected nightly toolchain (`just fmt`) because `.rustfmt.toml`
uses unstable formatting options; nightly is not required to build or test the crate.

### Notes

* For builds with profiles that include `debug_assertions` there are checks enabled that ensure that the hasher is invoked only once but they are disabled otherwise.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) of http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.