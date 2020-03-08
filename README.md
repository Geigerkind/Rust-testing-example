# Rust-testing-example
Sample repository to test different testing frameworks and tools in Rust

These are purposes for tools Im looking for:
* Random testing (JAVA tool: Randoop)
    * https://github.com/altsysrq/proptest
* Input Space Partitioning (Combinatorial Testing) (Java tool: ACTS)
* Graph Coverage (Java tool: EclEmma)
* Logic Coverage (Java tool: EclEmma)
    * https://github.com/xd009642/tarpaulin
    * https://github.com/mozilla/grcov
* Syntax Coverage / Mutation Testing (Java tool: MuJava, Major)
    * https://github.com/Geal/mutant
    * https://github.com/llogiq/mutagen
    * https://llogiq.github.io/2018/02/14/mutagen.html

Bonus:
* Parameterized testing
    * https://github.com/frondeus/test-case/blob/master/README.md
    
# Installation
1. Install rust:
```shell script
yay -S rustup
rustup install nightly
```
2. Install cargo:
```shell script
yay -S cargo
cargo install cargo-tarpaulin
```
3. Run tests
```shell script
# Standard tests and random testing
cargo test

# Line coverage
cargo tarpaulin -v
```