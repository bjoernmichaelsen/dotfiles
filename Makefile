NERDFONTS_BASEURL=https://github.com/ryanoasis/nerd-fonts/releases/download/
NERDFONTS_VERSION=v3.0.2
NERDFONTS_NAMES=3270 Hack Monoid

all: nerdfonts-install nushell-install starship-install carapace-install
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

nerdfonts-install: $(foreach n,$(NERDFONTS_NAMES),workdir/nerdfonts/$n.zip)
	for n in $^; do unzip $$n -d ~/.local/share/fonts; done
	fc-cache -f -v

define container-build
$1-build:
	podman build --tag $$@ $$@
	podman run --mount type=bind,source=workdir/,destination=/out $$@ cp $2 /out

.PHONY: $1-build
endef

$(eval $(call container-build, carapace, carapace-bin/cmd/carapace/carapace))
$(eval $(call container-build, starship, starship/target/release/starship))
$(eval $(call container-build, nushell, nushell/target/release/nu))

nerdfonts-dl: $(foreach n,$(NERDFONTS_NAMES),workdir/nerdfonts/$n.zip)

workdir/nerdfonts/%: workdir
	curl --proto '=https' --tlsv1.2 -Sf -L -o workdir/nerdfonts/$* $(NERDFONTS_BASEURL)/$(NERDFONTS_VERSION)/$*

workdir:
	mkdir $@
	mkdir $@/nerdfonts

clean:
	rm -rf workdir

.PHONY: all nerdfonts-dl nerdfonts-install clean
