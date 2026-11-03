<div align="center">

# ⚡ rrgb

**High-performance, memory-safe Linux RGB driver & CLI written in Rust for MSI & SteelSeries gaming laptops.**

[![CI](https://github.com/MikhaelCat/r-steelseries/actions/workflows/ci.yml/badge.svg)](https://github.com/MikhaelCat/r-steelseries/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux-lightgrey.svg)]()
[![Presets](https://img.shields.io/badge/effects-30%20hardware%20presets-purple.svg)]()
[![Performance](https://img.shields.io/badge/latency-3.8µs%20generation-brightgreen.svg)]()

*Engineered specifically for MSI laptops equipped with SteelSeries Per-Key RGB keyboards, front panoramic Aurora Lightbars, and illuminated Dragon logos.*

[Features](#-key-features) • [Installation](#-installation) • [Hardware Diagnostics](#-hardware-diagnostics) • [Lighting Presets (30)](#-curated-lighting-presets-30) • [CLI Reference](#-command-line-reference) • [Architecture](#-architecture--benchmarks)

---

</div>

## 🌟 Key Features

- **🚀 Ultra-Low Latency:** Written in 100% safe Rust. Scene generation executes in **~3.8 microseconds**; commands latch to hardware in **2–5 milliseconds** (30x–50x faster than legacy Python scripts).
- **🔒 Zero CPU & Zero Memory Overhead:** Animations are compiled directly into native hardware firmware bytecode and offloaded onto the onboard SteelSeries microcontroller. Once applied, `rrgb` exits with **0% background CPU usage**.
- **🌈 Dual-Controller Multi-Zone Sync:** Synchronizes **both** the Per-Key RGB keyboard (`KLC 1038:1122`) and the chassis Aurora Lightbar & Dragon logo (`ALC 1038:1161`) into one cohesive aesthetic theme.
- **🎨 30 Curated Hardware Effects:** The complete original SteelSeries catalog plus modern atmospheric presets (Cyberpunk, Vaporwave, Volcanic Magma, Cosmic Nebula, Deep Ocean, Arctic Blizzard, Matrix Rain, Sakura, Fire, Aurora, and more).
- **🔍 Automated Device Probing (`rrgb check`):** Deep inspection of all kernel `/dev/hidraw*` endpoints to diagnose custom VIDs/PIDs across different laptop revisions.
- **🛡️ Battle-Tested Reliability:** Verified with Criterion micro-benchmarks, unit tests, and continuous 20,000-cycle memory stability test suites in GitHub Actions CI.

---

## 💻 Supported Hardware

`rrgb` communicates directly with Linux kernel HID subsystems (`/dev/hidraw` via `ioctl(HIDIOCSFEATURE)`).

| Controller Type | Vendor ID | Product ID | Description |
| :--- | :--- | :--- | :--- |
| **SteelSeries KLC** | `0x1038` | `0x1122` | Per-Key RGB Keyboard (MSI Raider GE75, GE76, GE78, 18 HX AI, Titan series) |
| **SteelSeries KLC (Gen 2)** | `0x1038` | `0x113A` | Modern SteelSeries Per-Key RGB Keyboard revisions |
| **SteelSeries ALC** | `0x1038` | `0x1161` | Front Panoramic Lightbar (Aurora) & Rear Dragon Shield Logo |
| **MSI 3-Zone Controller** | `0x1770` | `0xFF00` | Classic 3-Zone backlit gaming keyboards |

*Have a different ID? Run `rrgb check` to instantly display all candidate endpoints.*

---

## 📦 Installation

### 1. Arch Linux / CachyOS / Manjaro
Install via `makepkg` using the included [`PKGBUILD`](PKGBUILD):
```bash
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
makepkg -si
```
*(Or via your preferred AUR helper: `yay -S rrgb-bin`)*

### 2. Ubuntu / Debian / Pop!_OS / Linux Mint
```bash
sudo apt update && sudo apt install -y git build-essential curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
sudo ./install.sh
```

### 3. Fedora / RHEL / AlmaLinux
```bash
sudo dnf install -y git gcc cargo rust
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
sudo ./install.sh
```

### 4. openSUSE (Tumbleweed / Leap)
```bash
sudo zypper install -y git gcc cargo rust
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
sudo ./install.sh
```

### 5. Universal Source Build
```bash
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
cargo build --release
sudo cp target/release/rrgb /usr/local/bin/
sudo cp packaging/50-rrgb.rules /etc/udev/rules.d/
sudo udevadm control --reload && sudo udevadm trigger
```

---

## 🔍 Hardware Diagnostics

To verify your controllers and check hardware visibility:

```bash
rrgb check
```

```text
=== rrgb Hardware Diagnostics ===
[+] Successfully matched 2 RGB controller(s):
    -> SteelSeries KLC (MSI Keyboard)        | VID: 0x1038, PID: 0x1122 | Path: /dev/hidraw3
    -> SteelSeries ALC (MSI Lightbar / Logo) | VID: 0x1038, PID: 0x1161 | Path: /dev/hidraw5

=== All System HID Candidates (/dev/hidraw*) ===
[MATCHED] SteelSeries SteelSeries KLC | ID: 1038:1122 at /dev/hidraw3
[MATCHED] SteelSeries SteelSeries ALC | ID: 1038:1161 at /dev/hidraw5
[  SKIP  ] Logitech M350              | ID: 046d:4080 at /dev/hidraw2
```

---

## 🎨 Curated Lighting Presets (30)

`rrgb` includes **30 hardware animation presets** tuned specifically for per-key switch matrices and panoramic lightbars:

```
PRESET        DESCRIPTION
------------  -------------------------------------------------------------
cyberpunk     Electrifying contrast of cyber yellow and neon cyan
neon          Retro 80s outrun wave transitioning hot pink to cyan
vaporwave     Dreamy nostalgic pastel gradients of violet and peach
ocean         Underwater sea depth swell transitioning aquamarine to sapphire
sunset        Vibrant tropical gradient of ruby red, tangerine and violet
magma         Molten volcanic lava glow slowly rolling across the chassis
nebula        Deep galactic interstellar dust clouds of magenta and indigo
blizzard      Icy polar crystalline whites, pale cyans and frosty blues
toxic         Radioactive acid lime green and sulfur yellow surging wave
sakura        Delicate spring cherry blossom petaline pinks and white breeze
lightning     Electric high-voltage lightning flashes through dark stormy skies
solar         Coronal mass ejection of brilliant solar gold, amber and white
forest        Organic lush woodland emerald, pine and mossy shadows
galaxy        Starry spiral arm revolving through deep midnight violet and cyan
firefly       Enchanted nocturnal fireflies gently drifting across dark grass
stealth       Ultra-low-observable dimmed crimson night-vision tactical glow
matrix        Iconic digital cyber glyph rain descending downwards
fire          Warm flickering embers and rising fire flames
aurora        Mystic ethereal northern lights in cyan, green and purple
rainbow       Continuous spectrum wave propagating across zones
spectrum      Synchronized chromatic rainbow cycle across all LEDs
breathing     Smooth breathing pulse of the chosen color
starlight     Gentle random twinkling stars in deep space
glitch        Jittery chromatic cyber distortion across split clusters
pulse         Cardiogram double pulse with sharp decay
disco         High-energy multi-zone club strobe dance floor
police        Tactical alternating high-frequency red/blue strobe
reactive      Keys illuminate dynamically upon keystrokes
wave          Unidirectional sweeping pulse of custom color
static        Solid static color across all keys and lightbars
```

---

## ⌨️ Command-Line Reference

```bash
# Basic Power & Status
rrgb on                                  # Enable lighting
rrgb off                                 # Disable lighting
rrgb status                              # Print active mode, brightness and device paths
rrgb effects                             # List all 30 available presets
rrgb gui                                 # Launch the graphical control panel

# Atmospheric & Futuristic Presets
rrgb set cyberpunk -s 50 -d right        # Cyberpunk 2077 neon yellow/cyan flow
rrgb set nebula -s 40 -d right           # Deep galactic cosmic nebula
rrgb set blizzard -s 50 -d down          # Polar crystalline ice storm
rrgb set toxic -s 60 -d out              # Acidic biohazard surge
rrgb set sakura -s 35 -d right           # Japanese cherry blossom petaline flow
rrgb set lightning -s 70                 # Electric thunderbolt flashing
rrgb set solar -s 50 -d out              # Solar flare coronal blast
rrgb set forest -s 40 -d in              # Ancient woodland emerald canopy
rrgb set galaxy -s 35 -d out             # Revolving starry spiral galaxy
rrgb set firefly -s 45                   # Drifting nocturnal summer fireflies
rrgb set stealth                         # Dimmed crimson covert tactical glow

# Retro & Vaporwave
rrgb set neon -s 60 -d right             # Synthwave Outrun pink & cyan
rrgb set vaporwave -s 40                 # Pastel aesthetic violet & teal
rrgb set ocean -s 35 -d out              # Oceanic deep-sea swell
rrgb set sunset -s 45 -d left            # Miami sunset tropical gradient
rrgb set magma -s 50 -d down             # Volcanic magma wave descending
rrgb set matrix -s 65                    # Matrix green digital glyph rain
rrgb set fire -s 50                      # Natural flame flicker

# Classic Solid Colors
rrgb set static -c ff0000                # Pure Red
rrgb set static -c 00ffff                # Cyber Cyan
rrgb set static -c 8000ff                # Deep Purple
rrgb set static -c ffd700                # Gold
```

### Options & Parameter Flags

| Flag | Parameter | Values | Description |
| :--- | :--- | :--- | :--- |
| `-c`, `--color` | Hex Color | `ff0000`, `00ffff` | Primary accent color |
| `--highlight` | Hex Color | `ffffff`, `00ff00` | Keystroke reaction color (for `reactive`) |
| `-s`, `--speed` | Integer | `0` to `100` | Hardware transition speed percentage |
| `-b`, `--brightness` | Integer | `0` to `100` | Overall illumination brightness percentage |
| `-d`, `--direction` | String | `right`, `left`, `up`, `down`, `out`, `in` | Direction of spatial wave propagation |
| `--wave-size` | Integer | `0` to `100` | Spatial wavelength scale |

---

## ⚡ Architecture & Benchmarks

```
   ┌────────────────────────────────────────────────────────┐
   │                       rrgb CLI                         │
   │           (Command Parser, Color Interpolator)         │
   └───────────────┬────────────────────────┬───────────────┘
                   │                        │
       524-byte Feature Reports  64-byte Output Reports
                   │                        │
                   ▼                        ▼
       ┌──────────────────────┐  ┌──────────────────────┐
       │   SteelSeries KLC    │  │   SteelSeries ALC    │
       │    (Keyboard MCU)    │  │   (Lightbar & Logo)  │
       │     VID: 0x1038      │  │     VID: 0x1038      │
       │     PID: 0x1122      │  │     PID: 0x1161      │
       └──────────────────────┘  └──────────────────────┘
                   │                        │
                   └───────────┬────────────┘
                               ▼
                    Hardware LED Rendering
                 (0% CPU overhead on the host)
```

Micro-benchmarked using **Criterion.rs** on AMD / Intel Linux platforms:
- **Scene Assembly Latency:** `3.89 µs` (static) to `19.2 µs` (interpolated dynamic wave).
- **Process Memory RSS:** `< 3.2 MB`.
- **System Call Latency:** ~`180 ms` (dominated solely by USB HID bus transfer rates; CPU computation < `0.01 ms`).

Run tests and benchmarks locally:
```bash
cargo test
cargo test --test memory_leak_test   # 20,000 continuous cycle stress test
cargo bench                          # Criterion micro-benchmarks
```

---

## 🛡️ License

Licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

---

<div align="center">
<sub>Engineered with precision for the Linux gaming community. Maintained by <a href="https://github.com/MikhaelCat">MikhaelCat</a>.</sub>
</div>
