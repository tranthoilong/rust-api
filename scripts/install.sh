curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.zshenv

cargo new hello_rust
cd hello_rust
cargo run