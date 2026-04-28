<div align="center">

# Now Playing Notifier

A lightweight Rust CLI tool that listens to the currently active media player via MPRIS (D-Bus) and sends desktop notifications whenever the track changes.

<img src="resources/screenshot.png" alt="Screenshot of the Now Playing Notifier">
</div>

---

## Overview

This project is a small systems-level utility written in Rust that integrates with the Linux desktop environment.

It monitors the active media player through the MPRIS D-Bus interface and displays real-time “Now Playing” notifications using the system notification service.

---

## Features

* Detects the currently active media player via MPRIS
* Retrieves track metadata (title, artist)
* Sends desktop notifications on track change
* Super lightweight CLI-based design (consumes less than half a megabyte)
* Works with common Linux media players (Spotify, VLC, browser media sessions, etc.)
* Can download album art based on taste (configuration file) on by default

---
## Requirements
* * A Linux distribution running a notification daemon (tested on Pop!_OS with COSMIC)
---

## Dependencies

* `mpris`
* `notify-rust`

System-level dependencies (Linux):

* D-Bus (`libdbus-1`)

---

## Usage
If you would like to disable the album art feature, then go ahead and create a folder inside ./config and create a file called config.toml with the following content:
album_art = false
This will disable the album art feature in Music Info.
```bash
mkdir -p ~/.config/music_info && echo "album_art = false" > ~/.config/music_info/config.toml
```


### Installing

### using the make file
```bash
make install          # native
make install-flatpak  # flatpak version
make install-bin      # download and install release binary
```

### uninstalling
```bash
make uninstall          # native
make uninstall-flatpak  # flatpak version
make uninstall-bin      # uninstall release binary
```

### Run in development mode

```bash
cargo run
```

### Build release binary

```bash
cargo build --release
```

### Run binary

```bash
./target/release/<binary_name>
```

### building from source

```bash
cargo install --path .

mkdir -p ~/.config/systemd/user
cp music_info.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable music_info
systemctl --user start music_info
systemctl --user status music_info
```

### if downloading the release version from github
```bash
cp music_info  /usr/local/bin
chmod +x /usr/local/bin/music_info
```
repeat the steps for creating a service entry, but change the path to the binary.

---

## How it works

1. Connects to the D-Bus session bus
2. Finds the currently active MPRIS-compatible media player
3. Reads metadata (title, artist)
4. Formats a human-readable string
5. Sends a desktop notification when track changes

---


## Future Improvements

Planned or possible enhancements:
* System tray integration