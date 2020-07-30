# [panbuild](https://github.com/louib/panbuild)
![Functional tests status](https://github.com/louib/panbuild/workflows/tests/badge.svg)
![Flatpak-build CI job status](https://github.com/louib/panbuild/workflows/flatpak-build/badge.svg)
![Flatpak-install CI job status](https://github.com/louib/panbuild/workflows/flatpak-install/badge.svg)
[![Crates.io version](https://img.shields.io/crates/v/panbuild?style=flat-square)](https://crates.io/crates/panbuild)
[![License file](https://img.shields.io/github/license/louib/panbuild)](https://github.com/louib/panbuild/blob/master/LICENSE)
<!-- uncomment when there is a release available -->
<!-- [![GitHub release](https://img.shields.io/github/release/louib/panbuild)](https://github.com/louib/panbuild/releases/) -->

The universal build manifest converter.

> **This repo is a work-in-progress and is not ready for general use.
  The command-line options, command names and file formats might change
  at any time before the project reaches version 1.0.0.**

The supported packaging systems are:
* flatpak;
* snap;
* debian packages (via debian `control` files);

Panbuild aims to make Unix system package managers inter-operable, whether they are distribution
agnostic (snap, flatpak) or distribution based (deb, rpm, pacman, Homebrew). The executable is
portable and comes with an internal database of projects that can be installed through
various build systems.

## Install

### Using cargo
```
git clone git@github.com:louib/panbuild.git
cd panbuild/
cargo install --path .
```

You might need to adjust your `PATH` variable to find the binary:
```
export PATH="$PATH:~/.cargo/bin/"
```

### Using flatpak
```
# Make sure you have flathub installed.
# This is not working yet.
flatpak install net.louib.panbuild
```

### Install the binary
TODO

## Other related tools
* https://github.com/flatpak/flatpak-builder-tools

## License

BSD-3
