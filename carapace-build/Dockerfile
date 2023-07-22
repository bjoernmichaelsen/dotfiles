FROM docker.io/golang:1.20.6-bookworm
RUN apt-get update
RUN apt-get upgrade -y
RUN git clone -b v0.25.1 https://github.com/rsteube/carapace-bin.git
RUN cd carapace-bin && go mod download && go mod verify
RUN cd carapace-bin/cmd/carapace && go generate ./... && go build -ldflags="-s -w" .
