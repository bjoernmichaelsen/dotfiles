include ./config.mk

SHELL=sh

all: nerdfonts-install nushell-install starship-install carapace-install $(if $(FNM_ENABLED),fnm-install)
	@echo "Remaining manual steps:"
	@echo "You might want to add the following lines to ~/.config/nushell/config.nu, as I dont dare:"
	@echo "cat <<EOF >> ~/.config/nushell/config.nu"
	@echo "source ~/.config/nushell/starship.nu"
	@echo "source ~/.config/nushell/carapace.nu"
	@echo "EOF"

define container-install
$1-install: $1-build
	cp workdir/$(strip $2) ~/.local/bin/
	$3

.PHONY: $1-install
endef

$(eval $(call container-install, carapace, carapace, carapace _carapace nushell > ~/.config/nushell/carapace.nu))
$(eval $(call container-install, starship, starship, ~/.local/bin/starship init nu > ~/.config/nushell/starship.nu))
$(eval $(call container-install, nushell, nu,))
$(eval $(call container-install, fnm, fnm,))

nerdfonts-install: $(foreach n,$(NERDFONTS_NAMES),workdir/nerdfonts/$n.zip)
	for n in $^; do unzip -u $$n -d ~/.local/share/fonts; done
	fc-cache -f -v

define container-build
$1-build:
	podman build --tag $$@ $$@\
		--build-arg=RUST_BASE_IMAGE="$(strip $(RUST_BASE_IMAGE))"\
		--build-arg=TAG="$(strip $2)"\
		--build-arg=REPO_URL="$(strip $3)"\
	       	--build-arg=BUILD_DEPS="$(strip $4)"
	podman run --mount type=bind,source=workdir/,destination=/out $$@ cp $5 /out

.PHONY: $1-build
endef

$(eval $(call container-build, carapace,\
	$(CARAPACE_TAG),\
	$(CARAPACE_REPO_URL),\
	$(CARAPACE_BUILD_DEPS),\
	build/cmd/carapace/carapace))
$(eval $(call container-build, starship,\
	$(STARSHIP_TAG),\
	$(STARSHIP_REPO_URL),\
	$(STARSHIP_BUILD_DEPS),\
	build/target/release/starship))
$(eval $(call container-build, nushell,\
	$(NUSHELL_TAG),\
	$(NUSHELL_REPO_URL),\
	$(NUSHELL_BUILD_DEPS),\
	build/target/release/nu))
$(eval $(call container-build, fnm,\
	$(FNM_TAG),\
	$(FNM_REPO_URL),\
	$(FNM_BUILD_DEPS),\
	build/target/release/fnm))

nerdfonts-dl: $(foreach n,$(NERDFONTS_NAMES),workdir/nerdfonts/$n.zip)

workdir/nerdfonts/%: workdir
	curl --proto '=https' --tlsv1.2 -Sf -L -o workdir/nerdfonts/$* $(NERDFONTS_BASEURL)/$(NERDFONTS_VERSION)/$*

workdir:
	mkdir $@
	mkdir $@/nerdfonts

clean:
	rm -rf workdir

.PHONY: all nerdfonts-dl nerdfonts-install clean
