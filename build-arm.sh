#!/bin/bash

DOCKER_NAME=armv7-rust

sudo docker build -t $DOCKER_NAME docker-build-arm

sudo docker rm -f $DOCKER_NAME
sudo docker run -v $PWD:/rom-discord-bot --name $DOCKER_NAME $DOCKER_NAME /bin/bash -c "source ~/.cargo/env && cd /rom-discord-bot/ && cargo build --release --target arm-unknown-linux-gnueabihf"

# Once release has been built, copy it to the delivery folder
mkdir -p abs/armv7h
cp ./target/arm-unknown-linux-gnueabihf/release/rom-discord-bot abs/armv7h/rom-discord-bot

#Build arm arch package
cd abs && makepkg -f CARCH=armv7h
