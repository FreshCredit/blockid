FROM rust:1.65

COPY . /blockid
WORKDIR /blockid

# Install WebAssembly tools
RUN rustup target add wasm32-unknown-unknown
RUN rustup override set nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

#Install dependenties
RUN apt-get update
RUN apt install cmake -y
RUN apt install -y protobuf-compiler
RUN apt-get -y install clang

#Run build
RUN cargo +nightly build --release
RUN ./target/release/parachain-blockid-node build-spec --disable-default-bootnode > plain-parachain-chainspec.json
RUN ./target/release/parachain-blockid-node build-spec --chain plain-parachain-chainspec.json --disable-default-bootnode --raw > raw-parachain-chainspec.json
RUN ./target/release/parachain-blockid-node export-genesis-state --chain raw-parachain-chainspec.json para-2002-genesis-stat
RUN ./target/release/parachain-blockid-node export-genesis-wasm --chain raw-parachain-chainspec.json para-2002-wasm

#Start the app
CMD ./target/release/parachain-blockid-node --charlie --collator --force-authoring --chain raw-parachain-chainspec.json --base-path /tmp/parachain/charlie --port 40335 --ws-port 8846 -- --execution wasm --chain rococo-custom-3-raw.json --port 30345 --ws-port 9979

#Expose ports
EXPOSE 40335
EXPOSE 8846
EXPOSE 30345
EXPOSE 9979
