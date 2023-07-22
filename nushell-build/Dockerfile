FROM debian:bookworm
RUN apt-get update
RUN apt-get upgrade
RUN apt-get install -y git curl build-essential openssl pkg-config libssl-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN git clone -b 0.82.0 https://github.com/nushell/nushell
RUN . "${HOME}/.cargo/env" && cd nushell && cargo build --release
