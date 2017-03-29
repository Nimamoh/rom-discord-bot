from ubuntu:latest

ENV DEBIAN_FRONTEND noninteractive

#Replace default interpreter with bash
RUN ln -snf /bin/bash /bin/sh

#Install cross compiler for armhf
RUN apt-get update &&\
    apt-get -y install gcc-arm-linux-gnueabihf \
    wget \
    curl \
    vim \
    tree

#rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source ~/.cargo/env &&\
    echo "source ~/.cargo/env" >> ~/.bashrc &&\
    rustup install nightly &&\
    rustup default nightly-x86_64-unknown-linux-gnu &&\
    rustup target add arm-unknown-linux-gnueabihf

#openssl. Since some deps are manipulated we restore it afterwards
COPY openssl.sh /
RUN /openssl.sh 1.0.2j linux-armv4 arm-linux-gnueabihf-

RUN apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev \
    make \
    pkg-config

ENV PKG_CONFIG_PATH=/openssl/lib/pkgconfig/:$PKG_CONFIG_PATH

#libsodium
RUN mkdir /CROSS && cd /CROSS && \
    wget --no-check-certificate https://github.com/jedisct1/libsodium/releases/download/1.0.11/libsodium-1.0.11.tar.gz

RUN cd /CROSS && \
    tar xvf libsodium-1.0.11.tar.gz 

RUN cd /CROSS/libsodium-1.0.11 && \
    ./configure --host=arm-linux-gnueabihf --prefix=/libsodium && \
    make install

ENV PKG_CONFIG_PATH=/libsodium/lib/pkgconfig:$PKG_CONFIG_PATH

#libopus
RUN cd /CROSS && \
    wget --no-check-certificate http://downloads.xiph.org/releases/opus/opus-1.1.3.tar.gz

RUN cd /CROSS && \
    tar xvf opus-1.1.3.tar.gz

RUN mkdir /libopus && cd /CROSS/opus-1.1.3 && \
    ./configure --prefix=/libopus --exec-prefix=/libopus --host=arm-linux-gnueabihf && \
    make install

ENV PKG_CONFIG_PATH=/libopus/lib/pkgconfig/:$PKG_CONFIG_PATH

#Enable cross compile
ENV PKG_CONFIG_ALLOW_CROSS=1

COPY init-cargo-deps.sh /
RUN bash -c "source ~/.cargo/env && export USER=root && ./init-cargo-deps.sh"
