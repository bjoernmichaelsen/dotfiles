NERDFONTS_BASEURL:=https://github.com/ryanoasis/nerd-fonts/releases/download/
NERDFONTS_VERSION:=v3.0.2
NERDFONTS_NAMES:=3270 Hack Monoid

NUSHELL_REPO_URL:=https://github.com/nushell/nushell
NUSHELL_TAG:=0.82.0
NUSHELL_BUILD_DEPS:=build-essential openssl pkg-config libssl-dev

STARSHIP_REPO_URL:=https://github.com/starship/starship
STARSHIP_TAG:=v1.15.0
STARSHIP_BUILD_DEPS:=build-essential cmake

CARAPACE_REPO_URL:=https://github.com/rsteube/carapace-bin.git
CARAPACE_TAG:=v0.25.1
CARAPACE_BUILD_DEPS:=

# rust is mostly statically linked, but you want a Debian or Ubuntu base image with the same glibc of your target machine.
RUST_BASE_IMAGE:=debian:bookworm

