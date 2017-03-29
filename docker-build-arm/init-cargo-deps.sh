#!/bin/bash

cargo new --bin toto
mkdir -p toto/.cargo
echo "discord = \"0.7.0\"" >> toto/Cargo.toml
echo "[target.arm-unknown-linux-gnueabihf]" >> toto/.cargo/config
echo "linker = \"arm-linux-gnueabihf-gcc\"" >> toto/.cargo/config
cd toto && cargo build --release --target arm-unknown-linux-gnueabihf
cd .. && rm -rf toto/

