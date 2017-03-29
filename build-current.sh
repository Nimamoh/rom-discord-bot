#!/bin/bash

cargo build --release

cp target/release/nimamoh-bot abs/x86_64/nimamoh-bot
cd abs && makepkg -f
