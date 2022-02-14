<h1 align="center">
mediarepo
</h1>
<p align="center">
<img src="https://github.com/Trivernis/mediarepo/raw/main/mediarepo-ui/src-tauri/icons/64x64.png?raw=true"/>
</p>
<p align="center">
    <a href="https://github.com/Trivernis/mediarepo/actions/workflows/build.yml">
        <img src="https://img.shields.io/github/workflow/status/trivernis/mediarepo/Build%20and%20test?style=for-the-badge">
    </a>
    <a href="https://mediarepo.trivernis.dev">
        <img src="https://img.shields.io/website?style=for-the-badge&url=https%3A%2F%2Fmediarepo.trivernis.dev">
    </a>
    <a href="https://aur.archlinux.org/packages/mediarepo">
        <img src="https://img.shields.io/aur/version/mediarepo?style=for-the-badge">
    </a>
    <img src="https://img.shields.io/aur/license/mediarepo?style=for-the-badge">
</p>

- - -

> Mediarepo is a tool for managing media files.
It works similar to image boards (boorus) as it allows one to assign tags to media entries and
search for entries by using those tags.

![](https://mediarepo.trivernis.dev/assets/images/screenshot-1.png)

## Features

### Implemented

- management of multiple repositories
- running repository daemons on startup or in the background
- importing files from the file system
- assigning tags to files
- searching for files using tags and properties
- sorting files by properties and tag namespaces

### Planned

- tag aliases and implications
- file collections
- importing files from URLs
- tag lookup using SauceNao and IQDB
- synchronisation between clients

## Installation

In order to use mediarepo, the mediarepo daemon and UI application need to be installed.
Both can be downloaded from the [Releases](https://github.com/Trivernis/mediarepo/releases) page or the AUR.

Arch Linux:
```sh
$ yay -S mediarepo-daemon mediarepo
```

When installing manually the `mediarepo-daemon` binary needs to be accessible in the `PATH` variable.


## Building

### Prerequisites

You need to have a working rust toolchain (e.g. via [rustup](https://rustup.rs/)) and  [node.js](https://nodejs.org) installed.
For building the UI the required tauri build tooling needs to be installed as well. Please follow [their documentation](https://tauri.studio/docs/getting-started/prerequisites) for setup information.
You also need to have a working [python](https://www.python.org/) installation on your system.

### Building mediarepo

After all required dependencies are installed and tools are accessible in the `PATH`, you can build the project like follows:

> Note: You might need to make the `build.py` file executable with `chmod +x build.py`.

All Componens:
```sh
$ ./build.py build --ffmpeg
```

Daemon only:
```sh
$ ./build.py build --daemon --ffmpeg
```

If you don't want to build with ffmpeg support omit the `--ffmpeg` flag.

UI only:
```sh
$ ./build.py build --ui
```

After building the `out` directory contains all the built binaries and bundles.

### Test Builds

For test builds the `Dockerfile` in this repository can be used. This way no build dependencies need to be installed on the system. The dockerfile doesn't provide any artifacts and can only be used for validation.

## Usage and Further Information

Please consult the [official website](https://mediarepo.trivernis.dev) for more information. 


## License

GPL-3