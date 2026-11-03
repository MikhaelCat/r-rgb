<div align="center">

# ⚡ rrgb

**High-performance, memory-safe Linux RGB driver & modern Libadwaita GUI engineered in 100% Rust for MSI & gaming laptops.**

[![CI](https://github.com/MikhaelCat/r-rgb/actions/workflows/ci.yml/badge.svg)](https://github.com/MikhaelCat/r-rgb/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux-lightgrey.svg)]()
[![Presets](https://img.shields.io/badge/effects-50%20hardware%20presets-purple.svg)]()
[![Latency](https://img.shields.io/badge/scene%20gen-3.6µs-brightgreen.svg)]()
[![GTK4 / Libadwaita](https://img.shields.io/badge/GUI-Native%20GTK4%20%2B%20Adwaita-blue.svg)]()
[![Security Audit](https://img.shields.io/badge/security%20audit-0%20vulnerabilities-brightgreen.svg)]()

*Engineered specifically for MSI laptops equipped with Per-Key RGB keyboards, front panoramic Aurora Lightbars, and illuminated Dragon logos.*

[Features](#-key-features) • [Installation](#-multi-distribution-installation) • [Hardware Diagnostics](#-hardware-diagnostics) • [50 Lighting Presets](#-complete-50-hardware-presets-catalog) • [GUI Control Center](#-native-adwaita-gui) • [CLI Reference](#-command-line-reference) • [Architecture](#-architecture--benchmarks) • [Russian Guide (Руководство)](MSI_Подсветка_Команды.md)

---

</div>

## 🌟 Key Features

- **🚀 Ultra-Low Latency & Microsecond Precision:** Written in pure, memory-safe Rust with zero interpreter or Python overhead. Scene compilation executes in **~3.6 microseconds**; packet serialization to hardware in **~6.0 microseconds** (up to 50x faster than legacy tools).
- **🔒 Zero Background Host CPU Usage:** Pre-interpolated transitions are compiled into native hardware feature packets and committed directly onto the onboard hardware microcontroller. Once uploaded, `rrgb` immediately terminates with **0% background CPU consumption**.
- **🌈 Dual-Controller Multi-Zone Architecture:** Seamlessly controls **both** the Per-Key RGB keyboard (`KLC 1038:1122`) and the chassis Aurora Lightbar & Dragon shield logo (`ALC 1038:1161`). Supports both synchronized mirroring and independent custom color zoning.
- **🎨 50 Completely Unique Hardware Presets:** Meticulously designed lighting profiles across Core, Sci-Fi, Nature, Space, Ambient, Tactical, and Chromatic Harmonies — every effect features an individual color palette, spatial vectors, or zone-mapped key clusters.
- **🖥️ Native Libadwaita GUI (`rrgb gui`):**
  - **Live Animated Canvas Preview:** Real-time 30 FPS rendering engine with physical wave propagation, keycap borders, and multi-segment Lightbar representation.
  - **Dedicated Lock-Free Worker Thread:** Non-blocking hardware dispatch queue with intelligent 30ms debouncing eliminates bus lock contention (`flock`) and guarantees fluid, freeze-free rapid preset switching.
  - **Runtime Localization (RU/EN):** Instant dynamic language switching saved persistently to your configuration.
  - **User Profiles & Pinned Favorites:** Save custom presets to `~/.config/rrgb/profiles.json` and pin frequent effects with one-click star icons.
  - **Thermal-Reactive Mode:** Dynamically synchronizes real-time CPU/GPU temperatures into color heatmaps.
- **🛡️ Battle-Tested Reliability & Safety:** Audited with `cargo audit` (0 vulnerabilities), Criterion micro-benchmarks, and continuous 20,000-cycle memory stability stress tests in GitHub Actions CI with AddressSanitizer (ASan).

---

## 💻 Supported Hardware

`rrgb` communicates directly with Linux kernel HID endpoints (`/dev/hidraw*` via standard `ioctl(HIDIOCSFEATURE)`):

| Controller Type | Vendor ID | Product ID | Description / Hardware Scope |
| :--- | :--- | :--- | :--- |
| **KLC Keyboard** | `0x1038` | `0x1122` | Per-Key RGB Keyboard (MSI Raider GE75, GE76, GE78, 18 HX AI, Titan 18 HX, Stealth series) |
| **KLC Keyboard (Gen 2)** | `0x1038` | `0x113A` | Modern Per-Key RGB Keyboard revisions |
| **ALC Lightbar / Logo** | `0x1038` | `0x1161` | Front Panoramic Lightbar (Aurora) & Rear Illuminated Dragon Shield Logo |
| **MSI 3-Zone Controller** | `0x1770` | `0xFF00` | Classic 3-Zone backlit gaming keyboards |

*Have a custom or unlisted device revision? Run `rrgb check` to perform automated probing of all connected hardware endpoints.*

---

## 📦 Multi-Distribution Installation

### Arch Linux / CachyOS / Manjaro
Install via `makepkg` using the repository [`PKGBUILD`](PKGBUILD):
```bash
git clone https://github.com/MikhaelCat/r-rgb.git
cd r-rgb
makepkg -si
```
*(Or build directly with `cargo build --release`)*

### Ubuntu / Debian / Pop!_OS / Linux Mint
```bash
sudo apt update && sudo apt install -y git build-essential curl libgtk-4-dev libadwaita-1-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

git clone https://github.com/MikhaelCat/r-rgb.git
cd r-rgb
sudo ./install.sh
```

### Fedora / RHEL / AlmaLinux
```bash
sudo dnf install -y git gcc cargo rust gtk4-devel libadwaita-devel
git clone https://github.com/MikhaelCat/r-rgb.git
cd r-rgb
sudo ./install.sh
```

### openSUSE (Tumbleweed / Leap)
```bash
sudo zypper install -y git gcc cargo rust gtk4-devel libadwaita-devel
git clone https://github.com/MikhaelCat/r-rgb.git
cd r-rgb
sudo ./install.sh
```

### Universal Build & Direct Installation
```bash
git clone https://github.com/MikhaelCat/r-rgb.git
cd r-rgb
cargo build --release
sudo install -m 755 target/release/rrgb /usr/local/bin/rrgb
sudo install -m 644 packaging/50-rrgb.rules /etc/udev/rules.d/50-rrgb.rules
sudo udevadm control --reload && sudo udevadm trigger
```

---

## 🖥️ Native Adwaita GUI

Launch the native graphical control center:
```bash
rrgb gui
```

- **Zero-Latency Launch:** Native compiled Rust + GTK4 / Libadwaita without slow runtimes.
- **Real-Time Visualizer:** Interactive canvas displaying animated keycaps, ScrollLock, and 24-segment panoramic Lightbar.
- **Multi-Zone Decoupling:** Toggle independent zone control to isolate the front lightbar & dragon logo with custom hues.
- **Dynamic Localization:** Switch interface language between Russian (RU) and English (EN) at runtime.
- **Thermal Sync:** Real-time polling of hardware thermal sensors for gradient temperature visualization.

---

## 🎨 Complete 50 Hardware Presets Catalog

`rrgb` provides **50 unique hardware animation presets** offloaded onto the hardware microcontroller:

### 1. Core & Classics (7 Presets)
| Preset | Description | Customizable Options |
| :--- | :--- | :--- |
| `static` | Solid static color across all keys, lightbar, and dragon logo | Color, Brightness |
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
| `void` | Hypnotic inward cosmic vortex spiral twisting into a pitch-black abyss | `in` vortex |
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
| `stealth` | Covert military night-vision HUD (olive background, illuminated WASD/arrows) | Tactical HUD |
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
rrgb set void -s 45 -d in                # Cosmic deep void inward vortex
rrgb set stealth                         # Covert military night-vision HUD

# Solid Colors & Custom Wave
rrgb set static -c 00ffff                # Cyber Cyan
rrgb set static -c ff0055                # Hot Pink
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
    -> Per-Key RGB Keyboard (MSI) [1038:1122] at /dev/hidraw3
    -> Front Lightbar / Logo (MSI) [1038:1161] at /dev/hidraw5

=== All System HID Candidates (/dev/hidraw*) ===
[MATCHED] Per-Key RGB Keyboard | ID: 1038:1122 at /dev/hidraw3
[MATCHED] Front Lightbar / Logo | ID: 1038:1161 at /dev/hidraw5
[  SKIP  ] Logitech M350        | ID: 046d:4080 at /dev/hidraw2
```

---

## ⚡ Architecture & Micro-Benchmarks

```
   ┌────────────────────────────────────────────────────────┐
   │                  rrgb CLI & Native GUI                 │
   │      (Real-Time Simulator, Threaded Worker Queue)      │
   └───────────────┬────────────────────────┬───────────────┘
                   │                        │
       524-byte Feature Reports  64-byte Output Reports
                   │                        │
                   ▼                        ▼
       ┌──────────────────────┐  ┌──────────────────────┐
       │   KLC Keyboard MCU   │  │   ALC Lightbar & Logo│
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
- **Scene Assembly Latency:** `3.64 µs` (static) to `4.47 µs` (aurora/complex wave).
- **HID Packet Serialization:** `6.00 µs`.
- **Memory Consumption:** `< 3.2 MB RSS` with zero memory leaks across 20,000 continuous switching iterations.

Run verification suite locally:
```bash
cargo test
cargo test --test memory_leak_test   # 20,000 continuous cycle stress test
cargo bench                          # Criterion micro-benchmarks
cargo audit                          # RustSec security advisory vulnerability scan
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
