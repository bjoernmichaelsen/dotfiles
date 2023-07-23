ARG BASE_IMAGE
FROM $BASE_IMAGE
RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get upgrade -y
ARG TAG
ARG REPO_URL
RUN git clone -b $TAG $REPO_URL build
RUN cd build && go mod download && go mod verify
RUN cd build/cmd/carapace && go generate ./... && CGO_ENABLED=0 go build -ldflags="-s -w" .
