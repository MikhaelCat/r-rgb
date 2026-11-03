use clap::{Parser, Subcommand};
use std::process;

use r_steelseries::device::{find_all_devices, MultiDeviceManager};
use r_steelseries::effects::{build_scene, EFFECTS};
use r_steelseries::settings::{self, default_config_path, format_color, parse_color, DIRECTIONS};

#[derive(Parser, Debug)]
#[command(name = "r-steelseries", author = "MikhaelCat", version = "0.1.0")]
#[command(
    about = "Blazing-fast Rust controller for MSI SteelSeries keyboards, lightbars, and dragon logos"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show the detected devices and the saved settings
    Status,
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
    },
    /// Choose an effect and its options
    Set {
        /// Effect name (static, breathing, spectrum, rainbow, wave, starlight, reactive, disco, fire, aurora, neon, police, pulse, matrix)
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
) -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = MultiDeviceManager::load_all()?;
    let scene = build_scene(settings);
    manager.apply_scene(&scene)?;
    if !quiet {
        describe(settings);
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let config_path = default_config_path();
    let mut current = settings::load(&config_path);

    match cli.command {
        None | Some(Commands::Status) => {
            let devs = find_all_devices();
            if devs.is_empty() {
                println!("No SteelSeries KLC/ALC devices found!");
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
                println!("\nRun 'r-steelseries --help' for commands and options.");
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
            if let Err(e) = apply_settings(&current, false) {
                eprintln!("Error applying settings: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Off) => {
            current.enabled = false;
            let _ = settings::save(&current, &config_path);
            if let Err(e) = apply_settings(&current, false) {
                eprintln!("Error applying settings: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Apply { quiet }) => {
            if let Err(e) = apply_settings(&current, quiet) {
                eprintln!("Error applying settings: {}", e);
                process::exit(1);
            }
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
                    "Unknown effect '{}'. Run 'r-steelseries effects' for a list.",
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
            if let Err(e) = apply_settings(&current, false) {
                eprintln!("Error applying settings: {}", e);
                process::exit(1);
            }
        }
    }
}
