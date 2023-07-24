<h1 align="center">lthash-rs</h1>
</br>
<div align="center">
<!-- Version -->
  <a href="https://crates.io/crates/lthash-rs">
    <img src="https://img.shields.io/crates/v/lthash-rs"
    alt="Crates.io version" />
  </a>
  <!-- Docs -->
  <a href="https://docs.rs/lthash-rs">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/lthash-rs">
    <img src="https://img.shields.io/crates/d/lthash-rs"
      alt="Download" />
  </a>
  </div>
<br />
  <div align="center">
  <small>Built by the <a href="https://runtimemachines.io">RunTime Machines team</a></small>
</div>

## Description and scope of the project

This repository contains an implementation of LtHash, as
[defined](https://cseweb.ucsd.edu/~daniele/papers/IncHash.pdf) by
Bellare and Micciancio and later [specified more concretely](https://eprint.iacr.org/2019/227.pdf) by researchers at Facebook.

## Limitations

LtHash is vulnerable to multiset input collisions. A multiset is a
set containing more than one instance of a particular element. In particular, it is trivial to produce a collision in `lthash16` by adding the same input to the hash 2^16 times. One way to prevent this is to concatenate each input with a unique piece of metadata, such as an index.

## Installation

### Build

```sh
cargo build --release
```

### Test

```sh
cargo test
```

## License

Licensed under Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the license, shall be
licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
