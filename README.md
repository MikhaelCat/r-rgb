<div align="center">

# ⚡ rrgb

**Ultra-fast, memory-safe Linux RGB driver & Adwaita GUI written in 100% safe Rust for MSI & SteelSeries gaming laptops.**

[![CI](https://github.com/MikhaelCat/r-steelseries/actions/workflows/ci.yml/badge.svg)](https://github.com/MikhaelCat/r-steelseries/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux-lightgrey.svg)]()
[![Presets](https://img.shields.io/badge/effects-50%20hardware%20presets-purple.svg)]()
[![Performance](https://img.shields.io/badge/latency-3.8µs%20generation-brightgreen.svg)]()
[![GTK4 / Libadwaita](https://img.shields.io/badge/GUI-Native%20GTK4%20%2B%20Adwaita-blue.svg)]()

*Engineered specifically for MSI laptops equipped with SteelSeries Per-Key RGB keyboards, front panoramic Aurora Lightbars, and illuminated Dragon logos.*

[Features](#-key-features) • [Installation](#-multi-distribution-installation) • [Hardware Diagnostics](#-hardware-diagnostics) • [50 Lighting Presets](#-complete-50-hardware-presets-catalog) • [GUI Control Center](#-native-adwaita-gui) • [CLI Reference](#-command-line-reference) • [Architecture](#-architecture--benchmarks)

---

</div>

## 🌟 Key Features

- **🚀 Ultra-Low Latency & Zero Overhead:** Written in pure, memory-safe Rust with zero Python runtime dependencies. Scene compilation executes in **~3.8 microseconds**; commands latch to hardware in **2–5 milliseconds** (50x faster than legacy Python tools).
- **🔒 Zero Background CPU Usage:** Animations compile directly into native hardware firmware bytecode and are offloaded onto the onboard SteelSeries microcontroller. Once applied, `rrgb` immediately terminates with **0% background CPU consumption**.
- **🌈 Dual-Controller Multi-Zone Sync:** Synchronizes **both** the Per-Key RGB keyboard (`KLC 1038:1122`) and the chassis Aurora Lightbar & Dragon logo (`ALC 1038:1161`) into one unified aesthetic theme.
- **🎨 50 Categorized Hardware Presets:** The complete original SteelSeries catalog plus modern atmospheric, cyberpunk, nature, cosmic, and tactical profiles.
- **🖥️ Native Libadwaita GUI (`rrgb gui`):** Built directly in Rust with GTK4 and Libadwaita. Instant startup, categorized preset browser, live brightness/speed/direction sliders, and color palette.
- **🔍 Automated Device Probing (`rrgb check`):** Deep inspection of all kernel `/dev/hidraw*` endpoints to diagnose custom VIDs/PIDs across different laptop revisions.
- **🛡️ Battle-Tested Reliability:** Verified with Criterion micro-benchmarks, unit tests, and continuous 20,000-cycle memory stability test suites in GitHub Actions CI with AddressSanitizer (ASan).

---

## 💻 Supported Hardware

`rrgb` communicates directly with Linux kernel HID subsystems (`/dev/hidraw` via `ioctl(HIDIOCSFEATURE)`).

| Controller Type | Vendor ID | Product ID | Description |
| :--- | :--- | :--- | :--- |
| **SteelSeries KLC** | `0x1038` | `0x1122` | Per-Key RGB Keyboard (MSI Raider GE75, GE76, GE78, 18 HX AI, Titan 18 HX series) |
| **SteelSeries KLC (Gen 2)** | `0x1038` | `0x113A` | Modern SteelSeries Per-Key RGB Keyboard revisions |
| **SteelSeries ALC** | `0x1038` | `0x1161` | Front Panoramic Lightbar (Aurora) & Rear Dragon Shield Logo |
| **MSI 3-Zone Controller** | `0x1770` | `0xFF00` | Classic 3-Zone backlit gaming keyboards |

*Have a custom or unlisted revision? Run `rrgb check` to display all detected endpoints on your system.*

---

## 📦 Multi-Distribution Installation

### Arch Linux / CachyOS / Manjaro
Install via `makepkg` using the included [`PKGBUILD`](PKGBUILD):
```bash
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
makepkg -si
```
*(Or via your preferred AUR helper: `yay -S rrgb-bin`)*

### Ubuntu / Debian / Pop!_OS / Linux Mint
```bash
sudo apt update && sudo apt install -y git build-essential curl libgtk-4-dev libadwaita-1-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
sudo ./install.sh
```

### Fedora / RHEL / AlmaLinux
```bash
sudo dnf install -y git gcc cargo rust gtk4-devel libadwaita-devel
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
sudo ./install.sh
```

### openSUSE (Tumbleweed / Leap)
```bash
sudo zypper install -y git gcc cargo rust gtk4-devel libadwaita-devel
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
sudo ./install.sh
```

### Universal Build & Direct Binary Installation
```bash
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
cargo build --release
sudo install -m 755 target/release/rrgb /usr/local/bin/rrgb
sudo install -m 644 packaging/50-rrgb.rules /etc/udev/rules.d/50-rrgb.rules
sudo udevadm control --reload && sudo udevadm trigger
```

---

## 🖥️ Native Adwaita GUI

To launch the native graphical control center:
```bash
rrgb gui
```

- **Instant Zero-Delay Launch:** Native Rust + GTK4 / Libadwaita without slow interpreters or Python runtimes.
- **Categorized Presets:** Browse all 50 lighting presets neatly grouped into Core, Sci-Fi, Nature, Space, Ambient, Tactical, and Harmonies.
- **Dynamic Tuning:** Adjust brightness, animation speed, and spatial wave direction in real-time.
- **Color Palette:** Quick-apply curated color swatches or custom hex accents.

---

## 🎨 Complete 50 Hardware Presets Catalog

`rrgb` provides **50 meticulously crafted hardware animation presets** offloaded onto the SteelSeries microcontroller:

### 1. Core & Classics (7 Presets)
| Preset | Description | Customizable Options |
| :--- | :--- | :--- |
| `static` | Solid static color across all keys and lightbars | Color, Brightness |
| `breathing` | Smooth breathing pulse of the chosen color | Color, Speed, Brightness |
| `spectrum` | Synchronized chromatic rainbow cycle across all LEDs | Speed, Brightness |
| `rainbow` | Continuous spectrum wave propagating across zones | Speed, Brightness, Direction |
| `wave` | Unidirectional sweeping pulse of custom color | Color, Speed, Brightness, Direction |
| `starlight` | Gentle random twinkling stars in deep space | Color, Speed, Brightness |
| `reactive` | Keys illuminate dynamically upon keystrokes | Color, Highlight, Speed, Brightness |

### 2. Cyberpunk, Sci-Fi & Digital (8 Presets)
| Preset | Description | Default Direction |
| :--- | :--- | :--- |
| `cyberpunk` | Electrifying contrast of cyber yellow and neon cyan | `right` |
| `matrix` | Iconic digital cyber glyph rain descending downwards | `down` |
| `neon` | Retro 80s outrun wave transitioning hot pink to cyan | `right` |
| `vaporwave` | Dreamy nostalgic pastel gradients of violet and peach | `right` |
| `glitch` | Jittery chromatic cyber distortion across split clusters | Strobe |
| `tron` | Clean high-contrast digital laser grid in cyan and white | `right` |
| `quantum` | Pulsating subatomic particles oscillating through indigo and teal | `out` |
| `overclock` | High-temperature digital redline glow racing through the traces | `right` |

### 3. Nature & Atmospheric Elements (9 Presets)
| Preset | Description | Default Dynamics |
| :--- | :--- | :--- |
| `fire` | Warm flickering embers and rising fire flames | Ember flicker |
| `aurora` | Mystic ethereal northern lights in cyan, green and purple | `right` wave |
| `ocean` | Underwater sea depth swell transitioning aquamarine to sapphire | `out` wave |
| `magma` | Molten volcanic lava glow slowly rolling across the chassis | `down` wave |
| `blizzard` | Icy polar crystalline whites, pale cyans and frosty blues | `down` wave |
| `toxic` | Radioactive acid lime green and sulfur yellow surging wave | `out` surge |
| `sakura` | Delicate spring cherry blossom petaline pinks and white breeze | `right` breeze |
| `lightning` | Electric high-voltage lightning flashes through dark stormy skies | Lightning strobe |
| `forest` | Organic lush woodland emerald, pine and mossy shadows | `in` canopy |

### 4. Astronomy & Deep Cosmos (7 Presets)
| Preset | Description | Dynamics |
| :--- | :--- | :--- |
| `nebula` | Deep galactic interstellar dust clouds of magenta and indigo | `right` flow |
| `solar` | Coronal mass ejection of brilliant solar gold, amber and white | `out` blast |
| `galaxy` | Starry spiral arm revolving through deep midnight violet and cyan | `out` spiral |
| `supernova` | Blinding white cosmic burst decaying into radiant violet and orange | `out` burst |
| `eclipse` | Pure dark totality ringed by a glowing golden corona wave | `out` corona |
| `void` | Abyssal blackness with distant, faint spectral purple shifts | Ambient pulse |
| `meteor` | Incandescent shooting stars streaking across a pitch-black matrix | `down` streak |

### 5. Mood, Chill & Ambient (7 Presets)
| Preset | Description | Dynamics |
| :--- | :--- | :--- |
| `sunset` | Vibrant tropical gradient of ruby red, tangerine and violet | `left` swell |
| `firefly` | Enchanted nocturnal fireflies gently drifting across dark grass | Random drift |
| `candle` | Intimate and gentle warm golden candle flicker in the dark | Candle flicker |
| `dusk` | Serene gradient of dusky periwinkle, lavender and deep navy | `right` drift |
| `emerald` | Deep oceanic jade flowing gently into sparkling seafoam | `right` flow |
| `amethyst` | Crystalline royal violet shimmering into magenta facets | `right` shimmer |
| `golden` | Warm amber sunbeams washing over the landscape at sunset | `right` wash |

### 6. Tactical, Action & Dynamic (6 Presets)
| Preset | Description | Dynamics |
| :--- | :--- | :--- |
| `stealth` | Ultra-low-observable dimmed crimson night-vision tactical glow | Static Low |
| `police` | Tactical alternating high-frequency red/blue strobe | Dual cluster |
| `pulse` | Cardiogram double pulse with sharp decay | Heartbeat |
| `disco` | High-energy multi-zone club strobe dance floor | Strobe matrix |
| `siren` | Rotating heavy hazard warning light across left and right clusters | Rotating beacon |
| `sonar` | Concentric ping pulses expanding outwards from the center | `out` pulse |

### 7. Exotic Chromatic Harmonies (6 Presets)
| Preset | Description | Dynamics |
| :--- | :--- | :--- |
| `lavender` | Gentle swaying violet blooms under soft silver skies | `right` wave |
| `inferno` | Raging aggressive wall of deep crimson, scarlet and gold flame | `up` wave |
| `cotton` | Sweet pastel baby blue, powder pink and soft marshmallow white | `right` wave |
| `citrus` | Zesty lemon yellow, lime green and juicy blood orange splash | `right` splash |
| `monochrome`| Silky monochrome ripple from pitch black through pure sterling white | `right` wave |
| `mirage` | Shimmering desert sand dunes blending gold, copper and turquoise oasis | `right` wave |

---

## ⌨️ Command-Line Reference

```bash
# Basic Power & Control
rrgb on                                  # Enable lighting
rrgb off                                 # Disable lighting
rrgb status                              # Print active mode, brightness and hardware paths
rrgb effects                             # List all 50 available hardware presets
rrgb gui                                 # Launch the native GTK4/Adwaita GUI

# High-Performance Hardware Effects
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

# Solid Colors & Custom Wave
rrgb set static -c ff0000                # Pure Red
rrgb set static -c 00ffff                # Cyber Cyan
rrgb set static -c 8000ff                # Deep Purple
rrgb set wave -c 00ffa0 -s 50 -d right   # Custom teal wave
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

## ⚡ Architecture & Benchmarks

```
   ┌────────────────────────────────────────────────────────┐
   │                  rrgb CLI & Native GUI                 │
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

at your option (standard Rust community dual-licensing).

---

<div align="center">
<sub>Engineered with precision for the Linux gaming community. Maintained by <a href="https://github.com/MikhaelCat">MikhaelCat</a>.</sub>
</div>
