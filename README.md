cryptopals-rust [![Build Status](https://travis-ci.org/dguenther/cryptopals-rust.svg)](https://travis-ci.org/dguenther/cryptopals-rust)
===============

Very rough solutions for the [Cryptopals](http://cryptopals.com) series of crypto challenges. I've only completed the first set so far.

Common functions are located in `src/cryptopalslib`, and binaries for individual challenges are located in the `src/setx` folders.

I wanted to write certain functions myself, even though they can already be found in Rust (for example, base64 conversion can be found in [rustc-serialize](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/index.html)). I haven't put effort into optimizing the code, so I wouldn't expect it to be as performant as possible.

I was focused on finding solutions for the challenges, so I haven't necessarily revisited the code to clean it up. I'd also like for this code to be idiomatic Rust, but I'm still learning what that means, so I'd appreciate any suggestions for how to make it better.

How to run
==========

1. Head over to [Rust's site](http://www.rust-lang.org) and install the latest stable version of Rust.

2. Clone the repo change directory to the repo's root.

3. To run tests, run `cargo test`. To run programs for individual challenges, run `cargo run --bin <set>-<challenge>` (for example: `cargo run --bin 1-1).