FROM ubuntu:latest

# Copy project files
COPY . /home/appuser/polkadot
WORKDIR /home/appuser/polkadot



RUN apt update -y && \
    apt install -y cmake protobuf-compiler clang supervisor build-essential gcc make curl bash

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
RUN ~/.cargo/bin/cargo install wasm-pack
RUN ~/.cargo/bin/rustup install nightly-2022-11-15-x86_64-unknown-linux-gnu
RUN ~/.cargo/bin/rustup override set nightly-2022-11-15
RUN ~/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain nightly-2022-11-15
RUN ~/.cargo/bin/cargo build --release

# Expose ports
EXPOSE 30333
EXPOSE 30334                
EXPOSE 30335
EXPOSE 9944
EXPOSE 9945
EXPOSE 9946

# Copy supervisord configuration
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
