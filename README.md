# Harpoon
A Linux screenreader, written in the Rust programming language.

## Installation
### Toolchain Setup
In order to compile the program, please be sure that you have the Rust toolchain installed. You may do so by using the rustup utility.
In addition, you will also need to ensure that you have speech-dispatcher present on your system.
Finally, ensure that you have at-spi installed on your system.

### Building
Run cargo build to compile the program with debugging support. If you with to generate a release build, use cargo build --release.
### Installation
To install the program to your user's path, use cargo install --path .
