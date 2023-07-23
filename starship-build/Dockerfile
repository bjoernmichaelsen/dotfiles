ARG RUST_BASE_IMAGE
FROM ${RUST_BASE_IMAGE}
RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get upgrade -y
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y git curl build-essential cmake
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN git clone -b v1.15.0 https://github.com/starship/starship
RUN . "${HOME}/.cargo/env" && cd starship && cargo build --release
