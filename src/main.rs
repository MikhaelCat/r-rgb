use clap::{Parser, Subcommand};
use std::process;

use r_rgb::device::{find_all_devices, list_all_hid_candidates, MultiDeviceManager};
use r_rgb::effects::{build_scene, EFFECTS};
use r_rgb::settings::{self, default_config_path, format_color, parse_color, DIRECTIONS};

#[derive(Parser, Debug)]
#[command(name = "rrgb", author = "MikhaelCat", version = "0.1.0")]
#[command(
    about = "Blazing-fast Rust controller for Linux RGB keyboards, lightbars, and dragon logos"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show the detected devices and the saved settings
    Status,
    /// Probe and diagnose all system HID devices to test keyboard detection
    Check,
    /// List all available lighting effects
    Effects,
    /// Turn the lighting on
    On,
    /// Turn the lighting off
    Off,
    /// Apply the currently saved settings
    Apply {
        #[arg(short, long)]
        quiet: bool,
        /// Maximum seconds to wait/retry if device is reconnecting
        #[arg(short, long, default_value_t = 0)]
        wait: u64,
    },
    /// Apply newest saved settings of any user (used by systemd on boot and resume)
    Restore {
        #[arg(short, long)]
        quiet: bool,
        /// Maximum seconds to wait/retry while device connects (default 10s)
        #[arg(short, long, default_value_t = 10)]
        wait: u64,
    },
    /// Launch the graphical interface (GUI)
    Gui,
    /// Choose an effect and its options
    Set {
        /// Effect name (run 'rrgb effects' to see all 50 available hardware presets)
        effect: String,

        /// Main color in hex (e.g. ff8000, 00ffff)
        #[arg(short, long)]
        color: Option<String>,

        /// Key press color for reactive mode
        #[arg(long)]
        highlight: Option<String>,

        /// Animation speed (0-100)
        #[arg(short, long)]
        speed: Option<f32>,

        /// Brightness (0-100)
        #[arg(short, long)]
        brightness: Option<f32>,

        /// Wave direction (right, left, down, up, out, in)
        #[arg(short, long)]
        direction: Option<String>,

        /// Wave size (0-100)
        #[arg(long)]
        wave_size: Option<f32>,
    },
}

fn describe(s: &settings::Settings) {
    if !s.enabled {
        println!("Lighting: off");
        return;
    }
    let eff = EFFECTS.iter().find(|e| e.id == s.effect);
    let name = eff.map(|e| e.name).unwrap_or(&s.effect);
    let mut parts = vec![name.to_string()];

    if eff.map(|e| e.uses_color).unwrap_or(true) {
        parts.push(s.color.clone());
    }
    if eff.map(|e| e.uses_highlight).unwrap_or(false) {
        parts.push(format!("highlight {}", s.highlight));
    }
    if eff.map(|e| e.uses_speed).unwrap_or(true) {
        parts.push(format!("speed {}%", (s.speed * 100.0).round()));
    }
    if eff.map(|e| e.uses_direction).unwrap_or(false) {
        parts.push(format!(
            "direction {}, size {}%",
            s.direction,
            (s.wave_size * 100.0).round()
        ));
    }
    parts.push(format!("brightness {}%", (s.brightness * 100.0).round()));
    println!("Lighting: {}", parts.join(", "));
}

fn apply_settings(
    settings: &settings::Settings,
    quiet: bool,
    wait_secs: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let max_wait = std::time::Duration::from_secs(wait_secs);

    loop {
        match MultiDeviceManager::load_all() {
            Ok(mut manager) => {
                let scene = build_scene(settings);
                manager.apply_scene(&scene)?;
                if !quiet {
                    describe(settings);
                }
                return Ok(());
            }
            Err(e) => {
                if start.elapsed() >= max_wait {
                    return Err(Box::new(e));
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let config_path = default_config_path();
    let mut current = settings::load(&config_path);

    match cli.command {
        None | Some(Commands::Status) => {
            let devs = find_all_devices();
            if devs.is_empty() {
                println!("No RGB controllers detected!");
                println!("Hint: run 'rrgb check' to inspect all attached HID devices.");
            } else {
                for d in devs {
                    println!(
                        "Device: {} [{:04x}:{:04x}] at {}",
                        d.name,
                        d.vendor_id,
                        d.product_id,
                        d.path.display()
                    );
                }
            }
            describe(&current);
            if cli.command.is_none() {
                println!("\nRun 'rrgb --help' for commands, or 'rrgb check' for diagnostics.");
            }
        }
        Some(Commands::Check) => {
            println!("=== rrgb Hardware Diagnostics ===");
            let devs = find_all_devices();
            if devs.is_empty() {
                println!("[!] No active RGB controllers matched.");
            } else {
                println!("[+] Successfully matched {} RGB controller(s):", devs.len());
                for d in &devs {
                    println!(
                        "    -> {} | VID: 0x{:04x}, PID: 0x{:04x} | Path: {}",
                        d.name,
                        d.vendor_id,
                        d.product_id,
                        d.path.display()
                    );
                }
            }

            println!("\n=== All System HID Candidates (/dev/hidraw*) ===");
            let all = list_all_hid_candidates();
            for (name, vid, pid, raw_id, path) in all {
                let status = if devs.iter().any(|d| d.path == path) {
                    "[MATCHED]"
                } else {
                    "[  SKIP  ]"
                };
                println!(
                    "{} {} | ID: {:04x}:{:04x} ({}) at {}",
                    status,
                    name,
                    vid,
                    pid,
                    raw_id,
                    path.display()
                );
            }
        }
        Some(Commands::Effects) => {
            println!("{:<12} DESCRIPTION", "EFFECT");
            println!("{:-<12} {:-<40}", "", "");
            for e in EFFECTS {
                println!("{:<12} {}", e.id, e.description);
            }
        }
        Some(Commands::On) => {
            current.enabled = true;
            let _ = settings::save(&current, &config_path);
            if let Err(e) = apply_settings(&current, false, 0) {
                eprintln!("Error applying settings: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Off) => {
            current.enabled = false;
            let _ = settings::save(&current, &config_path);
            if let Err(e) = apply_settings(&current, false, 0) {
                eprintln!("Error applying settings: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Apply { quiet, wait }) => {
            if let Err(e) = apply_settings(&current, quiet, wait) {
                eprintln!("Error applying settings: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Restore { quiet, wait }) => {
            if let Some(latest) = settings::latest_user_settings() {
                if let Err(e) = apply_settings(&latest, quiet, wait) {
                    if !quiet {
                        eprintln!("Error restoring settings: {}", e);
                    }
                    process::exit(1);
                }
            }
        }
        Some(Commands::Gui) => {
            let exit_code = r_rgb::gui::run_gui();
            process::exit(i32::from(exit_code));
        }
        Some(Commands::Set {
            effect,
            color,
            highlight,
            speed,
            brightness,
            direction,
            wave_size,
        }) => {
            let eff_match = EFFECTS.iter().find(|e| e.id == effect);
            if eff_match.is_none() {
                eprintln!(
                    "Unknown effect '{}'. Run 'rrgb effects' for a list.",
                    effect
                );
                process::exit(1);
            }

            current.enabled = true;
            current.effect = effect;

            if let Some(c) = color {
                match parse_color(&c) {
                    Ok(rgb) => current.color = format_color(rgb),
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
            }
            if let Some(h) = highlight {
                match parse_color(&h) {
                    Ok(rgb) => current.highlight = format_color(rgb),
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
            }
            if let Some(s) = speed {
                current.speed = (s / 100.0).clamp(0.0, 1.0);
            }
            if let Some(b) = brightness {
                current.brightness = (b / 100.0).clamp(0.0, 1.0);
            }
            if let Some(d) = direction {
                if !DIRECTIONS.contains(&d.as_str()) {
                    eprintln!("Invalid direction '{}'. Allowed: {:?}", d, DIRECTIONS);
                    process::exit(1);
                }
                current.direction = d;
            }
            if let Some(w) = wave_size {
                current.wave_size = (w / 100.0).clamp(0.0, 1.0);
            }

            let _ = settings::save(&current, &config_path);
            if let Err(e) = apply_settings(&current, false, 0) {
                eprintln!("Error applying settings: {}", e);
                process::exit(1);
            }
        }
    }
}
