ARG RUST_BASE_IMAGE
FROM ${RUST_BASE_IMAGE}
RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get upgrade -y
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y git curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ARG BUILD_DEPS
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y $BUILD_DEPS
ARG TAG
ARG REPO_URL
RUN git clone -b $TAG $REPO_URL build
RUN . "${HOME}/.cargo/env" && cd build && cargo build --release
