set -ex

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

cargo build --all --all-features --bins --tests --examples
cargo build --all --all-features --bins --tests --examples --release

cargo build --all --all-features --bins --tests --examples --target wasm32-unknown-unknown
cargo build --all --all-features --bins --tests --examples --target wasm32-unknown-unknown --release

cargo test
cargo test --release