# Now Playing Notifier

A lightweight Rust CLI tool that listens to the currently active media player via MPRIS (D-Bus) and sends desktop notifications whenever the track changes.

---

## Overview

This project is a small systems-level utility written in Rust that integrates with the Linux desktop environment.

It monitors the active media player through the MPRIS D-Bus interface and displays real-time “Now Playing” notifications using the system notification service.

---

## Features

* Detects the currently active media player via MPRIS
* Retrieves track metadata (title, artist)
* Sends desktop notifications on track change
* Super lightweight CLI-based design (consumes only half a megabyte)
* Works with common Linux media players (Spotify, VLC, browser media sessions, etc.)

---

## Dependencies

* `mpris`
* `notify-rust`

System-level dependencies (Linux):

* D-Bus (`libdbus-1`)

---

## Usage

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
* Album art support (when supported by notification daemon)