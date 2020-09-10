# cl[git]: Command Line GIT wrappers

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/clgit.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/clgit)
[![crates.io](https://img.shields.io/crates/v/clgit.svg)](https://crates.io/crates/clgit)
[![docs.rs](https://docs.rs/clgit/badge.svg)](https://docs.rs/clgit)
[![%23![forbid(unsafe_code)]](https://img.shields.io/github/search/MaulingMonkey/clgit/unsafe%2bextension%3Ars?color=green&label=%23![forbid(unsafe_code)])](https://github.com/MaulingMonkey/clgit/search?q=forbid%28unsafe_code%29+extension%3Ars)
[![rust: 1.32.0](https://img.shields.io/badge/rust-1.32.0%2B-green.svg)](https://gist.github.com/MaulingMonkey/c81a9f18811079f19326dac4daa5a359#minimum-supported-rust-versions-msrv)
[![License](https://img.shields.io/crates/l/clgit.svg)](https://github.com/MaulingMonkey/clgit)
[![Build Status](https://travis-ci.com/MaulingMonkey/clgit.svg?branch=master)](https://travis-ci.com/MaulingMonkey/clgit)
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/clgit/status.svg)](https://deps.rs/repo/github/MaulingMonkey/clgit) -->

### Pros

* Fully integrates with your local [git]
* <code>[#![forbid(unsafe_code)]](https://github.com/MaulingMonkey/clgit/search?q=forbid%28unsafe_code%29+extension%3Ars)</code>
* No dependencies
* MSRV: 1.32.0

### Cons

* Requires a [git] installation
* Extra overhead from constantly spawning new processes
* Leaner API



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.



<h2 name="contribution">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.



[git]:          https://git-scm.com/
[git2]:         https://crates.io/crates/git2
[libgit2-sys]:  https://crates.io/crates/libgit2-sys
[libgit2]:      https://libgit2.org/



## Alternative: [git2] / [libgit2-sys] / [libgit2]

Links:
[github](https://github.com/rust-lang/git2-rs),
[docs.rs](https://docs.rs/git2/0.13.11/git2/),
[crates.io](https://crates.io/crates/git2)

### Pros

* [git2] is much more widely used/vetted/tested
* [git2] is higher performance, probably
* No need to separately install git

### Cons

* [!Sync](https://docs.rs/git2/0.13.11/git2/struct.Repository.html#impl-Sync)
* [libgit2-sys] has annoying OpenSSL dev dependencies on linux to build
* [libgit2]'s [license](https://github.com/libgit2/libgit2/blob/master/COPYING) is complicated and GPL-laden
* Multiple crates pulling in different versions of [libgit2-sys] will cause build conflicts requiring upstream patches
* Unsafe-laden FFI makes me nervous
* Reading git repositories with an old libgit2 created by a newer git command line sounds like version mismatch incompatability bugs waiting to happen.
* May not fully integrate with any custom git hooks you may have setup
