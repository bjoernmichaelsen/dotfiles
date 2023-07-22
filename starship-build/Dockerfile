FROM debian:bookworm
RUN apt-get update
RUN apt-get upgrade
RUN apt-get install -y git curl build-essential cmake
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN git clone -b v1.15.0 https://github.com/starship/starship
RUN . "${HOME}/.cargo/env" && cd starship && cargo build -j1 --release
