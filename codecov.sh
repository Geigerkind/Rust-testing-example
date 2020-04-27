export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
export CODECOV_TOKEN="71475190-65f3-4b7f-b80f-03b62def4e9c"
cargo clean;
cargo update;
cargo build;
cargo test;
cargo test --verbose $CARGO_OPTIONS | zip -0 ccov.zip `find . \( -name "*.gc*" \) -print`;
~/.cargo/bin/grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" --ignore "src/tests/*" -o lcov.info;
bash <(curl -s https://codecov.io/bash) -f lcov.info;
genhtml -o ./target/debug/coverage/ --branch-coverage --show-details --highlight --ignore-errors source --legend ./lcov.info
rm ./ccov.zip
rm ./lcov.info
