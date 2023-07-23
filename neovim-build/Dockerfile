ARG BASE_IMAGE
FROM $BASE_IMAGE
RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get upgrade -y
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y git curl
ARG BUILD_DEPS
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y $BUILD_DEPS
ARG TAG
ARG REPO_URL
RUN git clone -b $TAG $REPO_URL build
RUN cd build && make CMAKE_BUILD_TYPE=Release
RUN cd build/build && cpack -G DEB
