
dotfiles
========

Some scripting to set up nushell and neovim ~anywhere with fonts, theming and
autocompletion. 

Dependencies
------------

* podman for containerized local builds
* curl 7.68 or later for downloads
* a POSIX shell
* and GNU make

Installing
----------

Run `make` in the project root. Note this installs into ~/.config and ~/.local.

On gentoo, you might prefer to use `make gentoo` -- this will leave out to
build stuff that is already packaged on that distro.

Configuring
-----------

You might want to change some parameters of the build in `config.mk`. The
current settings should be reasonable for any Debian/Ubuntu-based distro.

Related Themes
--------------

* gnome: https://github.com/arc-design/arc-theme
* vs code: https://marketplace.visualstudio.com/items?itemName=enkia.tokyo-night
* IDEA: https://plugins.jetbrains.com/plugin/15662-tokyo-night-color-scheme
