# One Package

## Description

Package fragmentation is rampant.

Instead of making yet another new format ([relevant XKCD](https://xkcd.com/927/)), here's a tool that combines several.

THIS PROJECT HAS NOT BEEN RELEASED YET. THE BASIC FEATURE SET IS NOT IMPLEMENTED!!!!

## Supported Formats

Currently, these are planned package formats supported:
- dnf (Fedora)
- apt (Debian)
- pacman (Arch)
- pacman AUR (also Arch)
- flatpak (general)
- AppImage (general)
- GitHub source which requires more info (general)

Please contribute and add to this list!

A list of supported packages are stored remotely in the pkg-ls.json file in the following format:

```
{
    'pkg_name': {
        'dnf': 'name in Fedora repos',
        'apt': 'name in Debian repos',
        'pacman': 'name in pacman repos',
        'aur': 'name in AUR',
        'flatpak': 'name in flatpak',
        'appimage': 'link to appimage',
        'github': {
            'repo': 'link to GitHub repo',
            'steps': [
                'first shell step to run (expects POSIX compliant shell)',
                '...',
                ...
            ], 'deps': [
                'other one-pkg dependency',
                '...',
                ...
            ]
        }
    }
}
```

Again, I'd love for you to contribute as not all packages will have all installs set up.

## Usage

The basic use case is `onepkg install <package>` or `onepkg uninstall <package>`

When installing, you will be prompted with a prompt based on which package format you'd like to use:

```
The package '<package>' is available in the following formats:
(1). dnf (Fedora)
(2). Flatpak
(3). AppImage
Please enter a number for which format you'd like to use: _
```

You'll then pick your desired format, and it will run the command using the corresponding package manager.

Internally, one-pkg keeps track of which packages are installed and what format they were installed with.
If they are only a dependency for a source package, they will be uninstalled.

There is also `onepkg auto-uninstall` which will tell certain package managers to remove unused packages, specifically:
- Pacman: `pacman -r $(Qtdq)`
- Debian: `sudo apt auto-remove`

