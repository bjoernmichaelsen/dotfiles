NERDFONTS_BASEURL=https://github.com/ryanoasis/nerd-fonts/releases/download/
NERDFONTS_VERSION=v3.0.2
NERDFONTS_NAMES=3270 Hack Monoid

all: nerdfonts-install

starship-install: starship-build
	cp workdir/starship ~/.local/bin/
	~/.local/bin/starship init nu > ~/.config/nushell/starship.nu

starship-build:
	podman build --tag $@ $@
	podman run --mount type=bind,source=workdir/,destination=/out $@ cp starship/target/release/starship /out

nushell-install: nushell-build
	cp workdir/starship ~/.local/bin/

nushell-build:
	podman build --tag $@ $@
	podman run --mount type=bind,source=workdir/,destination=/out $@ cp nushell/target/release/nu /out

nerdfonts-install: $(foreach n,$(NERDFONTS_NAMES),workdir/nerdfonts/$n.zip)
	for n in $^; do unzip $$n -d ~/.local/share/fonts; done
	fc-cache -f -v

nerdfonts-dl: $(foreach n,$(NERDFONTS_NAMES),workdir/nerdfonts/$n.zip)

workdir/nerdfonts/%: workdir
	curl --proto '=https' --tlsv1.2 -Sf -L --remote-name --output-dir workdir/nerdfonts $(NERDFONTS_BASEURL)/$(NERDFONTS_VERSION)/$*

workdir:
	mkdir $@
	mkdir $@/nerdfonts

clean:
	rm -rf workdir

.PHONY: all nerdfonts-dl nerdfonts-install nushell-build nushell-install starship-install starship-build clean
