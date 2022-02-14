# Flint
A simple kernel in rust.

## Building and running
First change the toolchain to nightly.
```
rustup override set nightly
```
Install bootimage
```
cargo install bootimage
```
Then add rust sources so we could compile them to the special target.
```
rustup component add rust-src
rustup component add llvm-tools-preview
```
Build the program using cargo as usual.
```
cargo build
```
