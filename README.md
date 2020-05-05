# Rust-testing-example
Testing tools:
* Random testing (JAVA tool: Randoop)
    * https://github.com/altsysrq/proptest
    * https://github.com/BurntSushi/quickcheck
* Input Space Partitioning (Combinatorial Testing)
    * ACTS: Generate input parameter and just parse them, see acts test
* Graph Coverage (Java tool: EclEmma)
* Logic Coverage (Java tool: EclEmma)
    * https://github.com/xd009642/tarpaulin
    * https://github.com/mozilla/grcov
* Syntax Coverage / Mutation Testing (Java tool: MuJava, Major)
    * https://github.com/llogiq/mutagen
    * https://llogiq.github.io/2018/02/14/mutagen.html
    * https://github.com/rust-fuzz/afl.rs

Bonus:
* Parameterized testing: https://github.com/frondeus/test-case/blob/master/README.md
* Snapshot testing: https://crates.io/crates/insta
* Popular JUnit Syntax: https://github.com/utkarshkukreti/speculate.rs
* Mocking library?: https://github.com/mockiato/mockiato
    
# Installation
1. Install rust:
```shell script
yay -S rustup lcov
rustup install nightly
```
2. Install cargo:
```shell script
yay -S cargo
cargo install cargo-tarpaulin
cargo install cargo-mutagen
cargo install grcov
```
3. Run tests
```shell script
# Standard tests and random testing
cargo test

# Line coverage
cargo tarpaulin -v

# Mutation testing
MUTATION_ID=1 cargo test
MUTATION_ID=2 cargo test

# Grcov
bash ./codecov.sh
# For results open:
firefox target/debug/coverage/index.html
# Codecov.io displays some data but not branch coverage
# Link: https://codecov.io/gh/Geigerkind/Rust-testing-example
```
