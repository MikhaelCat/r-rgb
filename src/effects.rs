use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::collections::HashMap;

use crate::layout;
use crate::protocol::{self, fade, Effect, Key, Wave, MODE_EFFECT, MODE_REACTIVE};
use crate::settings::{parse_color, Settings};

pub const TIME_UNIT: f32 = 0.01;
pub const REACTIVE_UNIT: f32 = 0.001;
pub const BLACK: (u8, u8, u8) = (0, 0, 0);
pub const STAR_SLOTS: usize = 8;

#[derive(Debug, Clone)]
pub struct EffectInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub uses_color: bool,
    pub uses_highlight: bool,
    pub uses_speed: bool,
    pub uses_direction: bool,
}

pub const EFFECTS: &[EffectInfo] = &[
    // --- Classics & Basic (1..7) ---
    EffectInfo {
        id: "static",
        name: "Static",
        category: "Core",
        description: "Solid static color across all keys and lightbars",
        uses_color: true,
        uses_highlight: false,
        uses_speed: false,
        uses_direction: false,
    },
    EffectInfo {
        id: "breathing",
        name: "Breathing",
        category: "Core",
        description: "Smooth breathing pulse of the chosen color",
        uses_color: true,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "spectrum",
        name: "Color Cycle",
        category: "Core",
        description: "Synchronized chromatic rainbow cycle across all LEDs",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "rainbow",
        name: "Rainbow Wave",
        category: "Core",
        description: "Continuous spectrum wave propagating across zones",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "wave",
        name: "Color Wave",
        category: "Core",
        description: "Unidirectional sweeping pulse of custom color",
        uses_color: true,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "starlight",
        name: "Starlight",
        category: "Core",
        description: "Gentle random twinkling stars in deep space",
        uses_color: true,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "reactive",
        name: "Reactive",
        category: "Core",
        description: "Keys illuminate dynamically upon keystrokes",
        uses_color: true,
        uses_highlight: true,
        uses_speed: true,
        uses_direction: false,
    },
    // --- Cyberpunk, Sci-Fi & Digital (8..15) ---
    EffectInfo {
        id: "cyberpunk",
        name: "Cyberpunk 2077",
        category: "Sci-Fi",
        description: "Electrifying contrast of cyber yellow and neon cyan",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "matrix",
        name: "Matrix Rain",
        category: "Sci-Fi",
        description: "Iconic digital cyber glyph rain descending downwards",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "neon",
        name: "Synthwave Neon",
        category: "Sci-Fi",
        description: "Retro 80s outrun wave transitioning hot pink to cyan",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "vaporwave",
        name: "Vaporwave",
        category: "Sci-Fi",
        description: "Dreamy nostalgic pastel gradients of violet and peach",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "glitch",
        name: "Glitch Shift",
        category: "Sci-Fi",
        description: "Jittery chromatic cyber distortion across split clusters",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "tron",
        name: "Grid Legacy",
        category: "Sci-Fi",
        description: "Clean high-contrast digital laser grid in cyan and white",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "quantum",
        name: "Quantum Flux",
        category: "Sci-Fi",
        description: "Pulsating subatomic particles oscillating through indigo and teal",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "overclock",
        name: "Overclock",
        category: "Sci-Fi",
        description: "High-temperature digital redline glow racing through the traces",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    // --- Nature & Atmospheric Elements (16..24) ---
    EffectInfo {
        id: "fire",
        name: "Fireplace",
        category: "Nature",
        description: "Warm flickering embers and rising fire flames",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "aurora",
        name: "Aurora Borealis",
        category: "Nature",
        description: "Mystic ethereal northern lights in cyan, green and purple",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "ocean",
        name: "Deep Ocean",
        category: "Nature",
        description: "Underwater sea depth swell transitioning aquamarine to sapphire",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "magma",
        name: "Volcanic Magma",
        category: "Nature",
        description: "Molten volcanic lava glow slowly rolling across the chassis",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "blizzard",
        name: "Arctic Blizzard",
        category: "Nature",
        description: "Icy polar crystalline whites, pale cyans and frosty blues",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "toxic",
        name: "Toxic Biohazard",
        category: "Nature",
        description: "Radioactive acid lime green and sulfur yellow surging wave",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "sakura",
        name: "Sakura Blossom",
        category: "Nature",
        description: "Delicate spring cherry blossom petaline pinks and white breeze",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "lightning",
        name: "Thunderstorm",
        category: "Nature",
        description: "Electric high-voltage lightning flashes through dark stormy skies",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "forest",
        name: "Deep Forest",
        category: "Nature",
        description: "Organic lush woodland emerald, pine and mossy shadows",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    // --- Astronomy & Deep Cosmos (25..31) ---
    EffectInfo {
        id: "nebula",
        name: "Cosmic Nebula",
        category: "Space",
        description: "Deep galactic interstellar dust clouds of magenta and indigo",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "solar",
        name: "Solar Flare",
        category: "Space",
        description: "Coronal mass ejection of brilliant solar gold, amber and white",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "galaxy",
        name: "Spiral Galaxy",
        category: "Space",
        description: "Starry spiral arm revolving through deep midnight violet and cyan",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "supernova",
        name: "Supernova",
        category: "Space",
        description: "Blinding white cosmic burst decaying into radiant violet and orange",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "eclipse",
        name: "Solar Eclipse",
        category: "Space",
        description: "Pure dark totality ringed by a glowing golden corona wave",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "void",
        name: "Deep Void",
        category: "Space",
        description: "Abyssal blackness with distant, faint spectral purple shifts",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "meteor",
        name: "Meteor Shower",
        category: "Space",
        description: "Incandescent shooting stars streaking across a pitch-black matrix",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    // --- Mood, Chill & Ambient (32..38) ---
    EffectInfo {
        id: "sunset",
        name: "Miami Sunset",
        category: "Ambient",
        description: "Vibrant tropical gradient of ruby red, tangerine and violet",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "firefly",
        name: "Summer Fireflies",
        category: "Ambient",
        description: "Enchanted nocturnal fireflies gently drifting across dark grass",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "candle",
        name: "Candlelight",
        category: "Ambient",
        description: "Intimate and gentle warm golden candle flicker in the dark",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "dusk",
        name: "Evening Twilight",
        category: "Ambient",
        description: "Serene gradient of dusky periwinkle, lavender and deep navy",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "emerald",
        name: "Emerald Isle",
        category: "Ambient",
        description: "Deep oceanic jade flowing gently into sparkling seafoam",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "amethyst",
        name: "Amethyst Geode",
        category: "Ambient",
        description: "Crystalline royal violet shimmering into magenta facets",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "golden",
        name: "Golden Hour",
        category: "Ambient",
        description: "Warm amber sunbeams washing over the landscape at sunset",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    // --- Tactical, Action & Dynamic (39..44) ---
    EffectInfo {
        id: "stealth",
        name: "Stealth Tactical",
        category: "Tactical",
        description: "Ultra-low-observable dimmed crimson night-vision tactical glow",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "police",
        name: "Emergency Strobe",
        category: "Tactical",
        description: "Tactical alternating high-frequency red/blue strobe",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "pulse",
        name: "Heartbeat Pulse",
        category: "Tactical",
        description: "Cardiogram double pulse with sharp decay",
        uses_color: true,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "disco",
        name: "Disco Party",
        category: "Tactical",
        description: "High-energy multi-zone club strobe dance floor",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "siren",
        name: "Amber Beacon",
        category: "Tactical",
        description: "Rotating heavy hazard warning light across left and right clusters",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "sonar",
        name: "Submarine Sonar",
        category: "Tactical",
        description: "Concentric ping pulses expanding outwards from the center",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    // --- Exotic Color Harmonies (45..50) ---
    EffectInfo {
        id: "lavender",
        name: "Lavender Fields",
        category: "Harmonies",
        description: "Gentle swaying violet blooms under soft silver skies",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "inferno",
        name: "Hellfire Inferno",
        category: "Harmonies",
        description: "Raging aggressive wall of deep crimson, scarlet and gold flame",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "cotton",
        name: "Cotton Candy",
        category: "Harmonies",
        description: "Sweet pastel baby blue, powder pink and soft marshmallow white",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "citrus",
        name: "Citrus Burst",
        category: "Harmonies",
        description: "Zesty lemon yellow, lime green and juicy blood orange splash",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "monochrome",
        name: "Silver Moonlight",
        category: "Harmonies",
        description: "Silky monochrome ripple from pitch black through pure sterling white",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "mirage",
        name: "Desert Mirage",
        category: "Harmonies",
        description: "Shimmering desert sand dunes blending gold, copper and turquoise oasis",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
];

#[derive(Debug, Clone)]
pub struct Scene {
    pub effects: HashMap<u8, Effect>,
    pub keys: HashMap<u8, Key>,
}

fn scale_color(rgb: (u8, u8, u8), brightness: f32) -> (u8, u8, u8) {
    (
        ((rgb.0 as f32) * brightness).round() as u8,
        ((rgb.1 as f32) * brightness).round() as u8,
        ((rgb.2 as f32) * brightness).round() as u8,
    )
}

fn period(speed: f32, slow: f32, fast: f32) -> f32 {
    slow * (fast / slow).powf(speed)
}

fn units(seconds: f32) -> u16 {
    (seconds / TIME_UNIT).round().max(1.0) as u16
}

fn get_wave(settings: &Settings) -> Wave {
    let (x, y, along_x, along_y, inward) = match settings.direction.as_str() {
        "right" => (0.0, 0.5, true, false, false),
        "left" => (1.0, 0.5, true, false, false),
        "down" => (0.5, 0.0, false, true, false),
        "up" => (0.5, 1.0, false, true, false),
        "out" => (0.5, 0.5, true, true, false),
        "in" => (0.5, 0.5, true, true, true),
        _ => (0.0, 0.5, true, false, false),
    };
    Wave {
        x,
        y,
        along_x,
        along_y,
        wavelength: settings.wave_size,
        inward,
    }
}

fn all_keys_matching<F>(f: F) -> HashMap<u8, Key>
where
    F: Fn() -> Key,
{
    let mut map = HashMap::new();
    for k in layout::get_keys() {
        map.insert(k.code, f());
    }
    for zid in 0..6u8 {
        map.insert(zid, f());
    }
    map
}

fn make_rainbow(brightness: f32, p: f32) -> Effect {
    let hues: &[(u8, u8, u8)] = &[
        (255, 0, 0),
        (255, 255, 0),
        (0, 255, 0),
        (0, 255, 255),
        (0, 0, 255),
        (255, 0, 255),
        (255, 0, 0),
    ];
    let scaled: Vec<(u8, u8, u8)> = hues.iter().map(|&h| scale_color(h, brightness)).collect();
    let step = units(p / 6.0);
    let frames: Vec<((u8, u8, u8), u16)> = scaled[1..].iter().map(|&h| (h, step)).collect();
    let (transitions, _) = fade(scaled[0], &frames);
    Effect::new(scaled[0], transitions, None)
}

pub fn build_scene(settings: &Settings) -> Scene {
    if !settings.enabled {
        let keys = all_keys_matching(|| Key {
            color: BLACK,
            ..Default::default()
        });
        return Scene {
            effects: HashMap::new(),
            keys,
        };
    }

    let raw_color = parse_color(&settings.color).unwrap_or((0, 255, 0));
    let color = scale_color(raw_color, settings.brightness);

    match settings.effect.as_str() {
        "static" => {
            // Reliable solid color across all hardware revisions (KLC and ALC Lightbar/Logo)
            let (tr, _) = fade(color, &[(color, 1000)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(color, tr, None));
            let keys = all_keys_matching(|| Key {
                color,
                mode: protocol::MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "reactive" => {
            let raw_high = parse_color(&settings.highlight).unwrap_or((255, 255, 255));
            let highlight = scale_color(raw_high, settings.brightness);
            let fade_s = period(settings.speed, 1.5, 0.15);
            let reactive_ms = (fade_s / REACTIVE_UNIT).round() as u16;
            let keys = all_keys_matching(|| Key {
                color,
                mode: MODE_REACTIVE,
                reactive_color: highlight,
                reactive_ms,
                ..Default::default()
            });
            Scene {
                effects: HashMap::new(),
                keys,
            }
        }
        "breathing" => {
            let p = period(settings.speed, 8.0, 1.2);
            let frames = &[
                (BLACK, units(p * 0.45)),
                (color, units(p * 0.45)),
                (color, units(p * 0.10)),
            ];
            let (transitions, _) = fade(color, frames);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(color, transitions, None));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "spectrum" => {
            let eff = make_rainbow(settings.brightness, period(settings.speed, 30.0, 2.0));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "rainbow" => {
            let mut eff = make_rainbow(settings.brightness, period(settings.speed, 12.0, 1.5));
            eff.wave = Some(get_wave(settings));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "wave" => {
            let p = period(settings.speed, 6.0, 1.0);
            let frames = &[
                (BLACK, units(p * 0.4)),
                (BLACK, units(p * 0.2)),
                (color, units(p * 0.4)),
            ];
            let (transitions, _) = fade(color, frames);
            let eff = Effect::new(color, transitions, Some(get_wave(settings)));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "starlight" => {
            let p = period(settings.speed, 6.0, 1.0);
            let rise = p * 0.12;
            let fall = p * 0.28;
            let mut effects = HashMap::new();

            for slot in 0..STAR_SLOTS {
                let offset = (slot as f32) * (p - rise - fall) / ((STAR_SLOTS - 1) as f32);
                let mut frames = Vec::new();
                if offset >= TIME_UNIT {
                    frames.push((BLACK, units(offset)));
                }
                frames.push((color, units(rise)));
                frames.push((BLACK, units(fall)));
                let rest = p - offset - rise - fall;
                if rest >= TIME_UNIT {
                    frames.push((BLACK, units(rest)));
                }
                let (transitions, _) = fade(BLACK, &frames);
                effects.insert(slot as u8, Effect::new(BLACK, transitions, None));
            }

            let mut codes: Vec<u8> = layout::get_keys().iter().map(|k| k.code).collect();
            let mut rng = rand::rngs::StdRng::seed_from_u64(7);
            codes.shuffle(&mut rng);

            let mut keys = HashMap::new();
            for (i, code) in codes.into_iter().enumerate() {
                keys.insert(
                    code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: (i % STAR_SLOTS) as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "fire" => {
            let p = period(settings.speed, 3.5, 0.7);
            let colors = [
                (255, 20, 0),
                (255, 80, 0),
                (255, 160, 0),
                (255, 230, 20),
                (200, 10, 0),
                (255, 110, 0),
                (255, 200, 10),
                (180, 5, 0),
            ];
            let step = units(p / 4.0);
            let mut effects = HashMap::new();
            for slot in 0..STAR_SLOTS {
                let ca = scale_color(colors[slot % colors.len()], settings.brightness);
                let cb = scale_color(colors[(slot + 2) % colors.len()], settings.brightness);
                let cc = scale_color(colors[(slot + 4) % colors.len()], settings.brightness);
                let frames = &[(cb, step), (cc, step), (ca, step * 2)];
                let (transitions, _) = fade(ca, frames);
                effects.insert(slot as u8, Effect::new(ca, transitions, None));
            }

            let mut codes: Vec<u8> = layout::get_keys().iter().map(|k| k.code).collect();
            let mut rng = rand::rngs::StdRng::seed_from_u64(42);
            codes.shuffle(&mut rng);

            let mut keys = HashMap::new();
            for (i, code) in codes.into_iter().enumerate() {
                keys.insert(
                    code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: (i % STAR_SLOTS) as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "aurora" => {
            // Ethereal northern lights: slow drifting curtains of neon cyan, emerald green and violet
            let p = period(settings.speed, 8.0, 1.2);
            let c_green = scale_color((0, 255, 120), settings.brightness);
            let c_cyan = scale_color((0, 210, 255), settings.brightness);
            let c_purple = scale_color((130, 0, 255), settings.brightness);
            let step = units(p * 0.30);
            let (tr_g, _) = fade(
                c_green,
                &[(c_cyan, step), (c_purple, step), (c_green, step)],
            );
            let (tr_p, _) = fade(
                c_purple,
                &[(c_green, step), (c_cyan, step), (c_purple, step)],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(c_green, tr_g, None));
            effects.insert(1, Effect::new(c_purple, tr_p, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if ((k.x + k.y * 1.5) as usize).is_multiple_of(2) {
                    0
                } else {
                    1
                };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "neon" => {
            // Synthwave Outrun: hot magenta base with cyan digital scanlines
            let p = period(settings.speed, 4.0, 0.6);
            let hot_magenta = scale_color((255, 0, 128), settings.brightness);
            let outrun_cyan = scale_color((0, 240, 255), settings.brightness);
            let step = units(p * 0.20);
            let (tr0, _) = fade(hot_magenta, &[(outrun_cyan, step), (hot_magenta, step * 2)]);
            let (tr1, _) = fade(outrun_cyan, &[(hot_magenta, step), (outrun_cyan, step * 2)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(hot_magenta, tr0, None));
            effects.insert(1, Effect::new(outrun_cyan, tr1, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if (k.y as usize).is_multiple_of(2) {
                    0
                } else {
                    1
                };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "cyberpunk" => {
            // High-voltage cyber contrast: sharp pulsating yellow keys with cyan laser scanline
            let p = period(settings.speed, 3.5, 0.6);
            let cyber_yellow = scale_color((255, 235, 0), settings.brightness);
            let cyber_cyan = scale_color((0, 245, 255), settings.brightness);
            let cyber_dark = scale_color((20, 10, 0), settings.brightness);
            let t_pulse = units(p * 0.15);
            let t_hold = units(p * 0.40);
            let (tr_y, _) = fade(
                cyber_yellow,
                &[(cyber_dark, t_pulse), (cyber_yellow, t_hold)],
            );
            let (tr_c, _) = fade(cyber_cyan, &[(cyber_dark, t_pulse), (cyber_cyan, t_hold)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(cyber_yellow, tr_y, Some(get_wave(settings))));
            effects.insert(1, Effect::new(cyber_cyan, tr_c, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if (k.code % 4) == 0 { 1 } else { 0 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "vaporwave" => {
            // Dreamy 80s aesthetic: smooth color shift across pastel violet, cyan and peach
            let p = period(settings.speed, 9.0, 1.4);
            let v_pink = scale_color((255, 115, 205), settings.brightness);
            let v_cyan = scale_color((0, 210, 255), settings.brightness);
            let v_peach = scale_color((255, 180, 140), settings.brightness);
            let v_purple = scale_color((170, 90, 255), settings.brightness);
            let step = units(p * 0.25);
            let mut effects = HashMap::new();
            let pal = [v_pink, v_cyan, v_peach, v_purple];
            for (slot, &col) in pal.iter().enumerate() {
                let next = pal[(slot + 1) % 4];
                let (tr, _) = fade(col, &[(next, step), (col, step)]);
                effects.insert(slot as u8, Effect::new(col, tr, None));
            }
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = ((k.x as usize) + (k.y as usize)) % 4;
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "ocean" => {
            // Oceanic swell: deep abyssal navy baseline with rhythmic turquoise crest swells
            let p = period(settings.speed, 8.0, 1.2);
            let deep_navy = scale_color((0, 20, 70), settings.brightness);
            let ocean_teal = scale_color((0, 220, 200), settings.brightness);
            let ocean_cyan = scale_color((0, 140, 255), settings.brightness);
            let swell_up = units(p * 0.35);
            let swell_down = units(p * 0.45);
            let rest = units(p * 0.20);
            let (tr, _) = fade(
                deep_navy,
                &[
                    (ocean_teal, swell_up),
                    (ocean_cyan, swell_down),
                    (deep_navy, rest),
                ],
            );
            let mut effects = HashMap::new();
            let wave_out = Wave {
                x: 0.5,
                y: 1.0,
                along_x: false,
                along_y: true,
                wavelength: 0.45,
                inward: false,
            };
            effects.insert(0, Effect::new(deep_navy, tr, Some(wave_out)));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "sunset" => {
            // Miami horizon gradient: static vertical elevation gradient with gentle golden breeze
            let p = period(settings.speed, 7.0, 1.1);
            let ruby = scale_color((255, 30, 40), settings.brightness);
            let orange = scale_color((255, 130, 0), settings.brightness);
            let violet = scale_color((130, 0, 160), settings.brightness);
            let step = units(p * 0.30);
            let (tr0, _) = fade(ruby, &[(orange, step), (ruby, step)]);
            let (tr1, _) = fade(orange, &[(violet, step), (orange, step)]);
            let (tr2, _) = fade(violet, &[(ruby, step), (violet, step)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(ruby, tr0, None));
            effects.insert(1, Effect::new(orange, tr1, None));
            effects.insert(2, Effect::new(violet, tr2, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if k.y < 2.0 {
                    2
                } else if k.y < 4.0 {
                    1
                } else {
                    0
                };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "magma" => {
            // Molten volcanic flow: cracked dark volcanic rock surging with glowing orange magma
            let p = period(settings.speed, 6.0, 0.9);
            let rock_black = scale_color((20, 0, 0), settings.brightness);
            let magma_hot = scale_color((255, 60, 0), settings.brightness);
            let magma_yellow = scale_color((255, 200, 20), settings.brightness);
            let surge = units(p * 0.20);
            let cool = units(p * 0.40);
            let crust = units(p * 0.40);
            let (tr_rock, _) = fade(
                rock_black,
                &[
                    (magma_hot, surge),
                    (magma_yellow, surge),
                    (rock_black, cool + crust),
                ],
            );
            let (tr_core, _) = fade(magma_hot, &[(magma_yellow, surge), (magma_hot, cool)]);
            let mut effects = HashMap::new();
            effects.insert(
                0,
                Effect::new(rock_black, tr_rock, Some(get_wave(settings))),
            );
            effects.insert(1, Effect::new(magma_hot, tr_core, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if (k.code % 3) == 0 { 1 } else { 0 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "blizzard" => {
            // Arctic blizzard: icy howling polar whites and cyan frost gusts across keys
            let p = period(settings.speed, 4.0, 0.6);
            let ice_white = scale_color((255, 255, 255), settings.brightness);
            let ice_cyan = scale_color((130, 230, 255), settings.brightness);
            let ice_blue = scale_color((0, 60, 160), settings.brightness);
            let gust = units(p * 0.12);
            let linger = units(p * 0.28);
            let calm = units(p * 0.60);
            let mut effects = HashMap::new();
            for slot in 0..STAR_SLOTS {
                let delay = units(slot as f32 * (p * 0.60 / (STAR_SLOTS as f32)));
                let frames = &[
                    (ice_blue, delay),
                    (ice_white, gust),
                    (ice_cyan, linger),
                    (ice_blue, calm),
                ];
                let (tr, _) = fade(ice_blue, frames);
                effects.insert(slot as u8, Effect::new(ice_blue, tr, None));
            }
            let mut codes: Vec<u8> = layout::get_keys().iter().map(|k| k.code).collect();
            let mut rng = rand::rngs::StdRng::seed_from_u64(24);
            codes.shuffle(&mut rng);
            let mut keys = HashMap::new();
            for (i, code) in codes.into_iter().enumerate() {
                keys.insert(
                    code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: (i % STAR_SLOTS) as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "nebula" => {
            // Cosmic interstellar cloud: rotating clouds of electric magenta, deep indigo and starburst white
            let p = period(settings.speed, 6.0, 1.0);
            let n_magenta = scale_color((240, 0, 160), settings.brightness);
            let n_indigo = scale_color((50, 0, 180), settings.brightness);
            let n_star = scale_color((230, 220, 255), settings.brightness);
            let step = units(p * 0.25);
            let (tr0, _) = fade(
                n_magenta,
                &[(n_indigo, step), (n_star, step), (n_magenta, step)],
            );
            let (tr1, _) = fade(
                n_indigo,
                &[(n_star, step), (n_magenta, step), (n_indigo, step)],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(n_magenta, tr0, None));
            effects.insert(1, Effect::new(n_indigo, tr1, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = (((k.x * 2.0) as usize) ^ ((k.y * 2.0) as usize)) % 2;
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "toxic" => {
            // Radioactive biohazard: pulsing acid lime waves with sharp yellow warning strobes
            let p = period(settings.speed, 3.5, 0.5);
            let acid_lime = scale_color((40, 255, 0), settings.brightness);
            let hazard_yellow = scale_color((240, 255, 0), settings.brightness);
            let toxic_dim = scale_color((0, 40, 0), settings.brightness);
            let step = units(p * 0.15);
            let pause = units(p * 0.50);
            let (tr0, _) = fade(
                toxic_dim,
                &[(acid_lime, step), (hazard_yellow, step), (toxic_dim, pause)],
            );
            let (tr1, _) = fade(acid_lime, &[(hazard_yellow, step), (acid_lime, step)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(toxic_dim, tr0, Some(get_wave(settings))));
            effects.insert(1, Effect::new(acid_lime, tr1, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if k.x < 5.0 || k.x > 14.0 { 1 } else { 0 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "sakura" => {
            // Spring blossom: soft petaline pinks gently falling with floating white highlights
            let p = period(settings.speed, 8.0, 1.2);
            let sakura_pink = scale_color((255, 175, 195), settings.brightness);
            let sakura_petal = scale_color((255, 235, 245), settings.brightness);
            let sakura_deep = scale_color((220, 90, 150), settings.brightness);
            let step = units(p * 0.30);
            let mut effects = HashMap::new();
            for slot in 0..4 {
                let col1 = if slot % 2 == 0 {
                    sakura_pink
                } else {
                    sakura_deep
                };
                let col2 = if slot % 2 == 0 {
                    sakura_petal
                } else {
                    sakura_pink
                };
                let (tr, _) = fade(col1, &[(col2, step), (col1, step)]);
                effects.insert(slot as u8, Effect::new(col1, tr, None));
            }
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = (((k.x * 1.5) as usize) + ((k.y * 2.0) as usize)) % 4;
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "solar" => {
            // Coronal solar flare: radiating blinding gold and deep fiery core
            let p = period(settings.speed, 5.0, 0.8);
            let solar_core = scale_color((255, 90, 0), settings.brightness);
            let solar_flare = scale_color((255, 230, 40), settings.brightness);
            let solar_burst = scale_color((255, 255, 255), settings.brightness);
            let wave_radial = Wave {
                x: 0.5,
                y: 0.5,
                along_x: true,
                along_y: true,
                wavelength: 0.30,
                inward: false,
            };
            let step = units(p * 0.15);
            let linger = units(p * 0.35);
            let (tr, _) = fade(
                solar_core,
                &[
                    (solar_burst, step),
                    (solar_flare, linger),
                    (solar_core, linger),
                ],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(solar_core, tr, Some(wave_radial)));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "forest" => {
            // Deep woodland canopy: emerald leaves with dappled golden sunbeams
            let p = period(settings.speed, 7.0, 1.1);
            let forest_emerald = scale_color((0, 160, 60), settings.brightness);
            let forest_moss = scale_color((20, 80, 25), settings.brightness);
            let sunbeam = scale_color((240, 230, 100), settings.brightness);
            let step = units(p * 0.25);
            let (tr0, _) = fade(
                forest_emerald,
                &[(forest_moss, step), (forest_emerald, step)],
            );
            let (tr1, _) = fade(forest_moss, &[(sunbeam, step), (forest_moss, step)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(forest_emerald, tr0, None));
            effects.insert(1, Effect::new(forest_moss, tr1, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if (k.code % 6) == 0 { 1 } else { 0 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "galaxy" => {
            // Spiral galaxy: deep midnight core with cyan arm rotating around center
            let p = period(settings.speed, 9.0, 1.4);
            let galaxy_dark = scale_color((15, 0, 50), settings.brightness);
            let galaxy_cyan = scale_color((0, 200, 240), settings.brightness);
            let galaxy_star = scale_color((255, 255, 255), settings.brightness);
            let galaxy_purple = scale_color((170, 0, 255), settings.brightness);
            let wave_spiral = Wave {
                x: 0.5,
                y: 0.5,
                along_x: true,
                along_y: false,
                wavelength: 0.40,
                inward: false,
            };
            let step = units(p * 0.20);
            let (tr, _) = fade(
                galaxy_dark,
                &[
                    (galaxy_purple, step),
                    (galaxy_cyan, step),
                    (galaxy_star, step),
                    (galaxy_dark, step),
                ],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(galaxy_dark, tr, Some(wave_spiral)));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "tron" => {
            // High-contrast cyber grid: blue borders/numpad, white laser runner
            let p = period(settings.speed, 3.5, 0.5);
            let laser_white = scale_color((255, 255, 255), settings.brightness);
            let grid_cyan = scale_color((0, 230, 255), settings.brightness);
            let grid_dim = scale_color((0, 25, 60), settings.brightness);
            let step = units(p * 0.15);
            let hold = units(p * 0.70);
            let (tr0, _) = fade(
                grid_dim,
                &[(grid_cyan, step), (laser_white, step), (grid_dim, hold)],
            );
            let (tr1, _) = fade(
                laser_white,
                &[(grid_dim, hold), (grid_cyan, step), (laser_white, step)],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(grid_dim, tr0, Some(get_wave(settings))));
            effects.insert(1, Effect::new(laser_white, tr1, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if k.x > 14.5 || k.y < 1.0 || k.y > 5.0 {
                    1
                } else {
                    0
                };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "quantum" => {
            // Quantum Flux: oscillating subatomic energy particles across staggered spatial hashes
            let p = period(settings.speed, 5.0, 0.7);
            let step = units(p * 0.20);
            let mut effects = HashMap::new();
            let q_indigo = scale_color((60, 0, 160), settings.brightness);
            let q_teal = scale_color((0, 255, 180), settings.brightness);
            let q_white = scale_color((200, 255, 255), settings.brightness);
            let palette = [q_indigo, q_teal, q_white, BLACK];
            for (slot, &col) in palette.iter().enumerate() {
                let next_col = palette[(slot + 1) % 4];
                let (tr, _) = fade(col, &[(next_col, step), (col, step * 2)]);
                effects.insert(slot as u8, Effect::new(col, tr, None));
            }
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = (((k.x * 2.0) as usize) ^ ((k.y * 3.0) as usize)) % 4;
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "overclock" => {
            // Overclock: intense crimson core with rapid orange/white spark pulses
            let p = period(settings.speed, 2.5, 0.4);
            let hot_red = scale_color((255, 0, 10), settings.brightness);
            let hot_orange = scale_color((255, 120, 0), settings.brightness);
            let white_spark = scale_color((255, 255, 255), settings.brightness);
            let step = units(p * 0.12);
            let (tr0, _) = fade(hot_red, &[(hot_orange, step), (hot_red, step * 2)]);
            let (tr1, _) = fade(
                white_spark,
                &[(hot_orange, step), (hot_red, step), (white_spark, step)],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(hot_red, tr0, None));
            effects.insert(1, Effect::new(white_spark, tr1, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if (k.code % 5) == 0 { 1 } else { 0 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "meteor" => {
            // High-speed incandescent shooting stars on dark backdrop
            let p = period(settings.speed, 2.5, 0.4);
            let flash = units(p * 0.08);
            let tail = units(p * 0.22);
            let rest = units(p * 0.70);
            let meteor_head = scale_color((255, 255, 230), settings.brightness);
            let meteor_tail = scale_color((255, 140, 20), settings.brightness);
            let mut effects = HashMap::new();
            for slot in 0..STAR_SLOTS {
                let delay = units(slot as f32 * (p * 0.70 / (STAR_SLOTS as f32)));
                let frames = &[
                    (BLACK, delay),
                    (meteor_head, flash),
                    (meteor_tail, tail),
                    (BLACK, rest),
                ];
                let (transitions, _) = fade(BLACK, frames);
                effects.insert(slot as u8, Effect::new(BLACK, transitions, None));
            }
            let mut codes: Vec<u8> = layout::get_keys().iter().map(|k| k.code).collect();
            let mut rng = rand::rngs::StdRng::seed_from_u64(88);
            codes.shuffle(&mut rng);
            let mut keys = HashMap::new();
            for (i, code) in codes.into_iter().enumerate() {
                keys.insert(
                    code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: (i % STAR_SLOTS) as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "supernova" => {
            // Blinding explosion flash decaying outward through violet and amber
            let p = period(settings.speed, 4.0, 0.7);
            let wave_out = Wave {
                x: 0.5,
                y: 0.5,
                along_x: true,
                along_y: true,
                wavelength: 0.35,
                inward: false,
            };
            let flash = units(p * 0.10);
            let exp_violet = scale_color((180, 0, 255), settings.brightness);
            let exp_amber = scale_color((255, 120, 0), settings.brightness);
            let c_white = scale_color((255, 255, 255), settings.brightness);
            let decay = units(p * 0.35);
            let after = units(p * 0.40);
            let pause = units(p * 0.15);
            let (tr, _) = fade(
                BLACK,
                &[
                    (c_white, flash),
                    (exp_violet, decay),
                    (exp_amber, after),
                    (BLACK, pause),
                ],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(BLACK, tr, Some(wave_out)));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "eclipse" => {
            // Dark totality disk with bright golden corona ring expanding from center
            let p = period(settings.speed, 6.0, 1.0);
            let corona_gold = scale_color((255, 200, 40), settings.brightness);
            let corona_white = scale_color((255, 245, 220), settings.brightness);
            let deep_shadow = scale_color((10, 0, 25), settings.brightness);
            let ring_wave = Wave {
                x: 0.5,
                y: 0.5,
                along_x: true,
                along_y: true,
                wavelength: 0.25,
                inward: false,
            };
            let step = units(p * 0.18);
            let linger = units(p * 0.24);
            let shadow_time = units(p * 0.40);
            let (tr, _) = fade(
                deep_shadow,
                &[
                    (corona_gold, step),
                    (corona_white, step),
                    (corona_gold, linger),
                    (deep_shadow, shadow_time),
                ],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(deep_shadow, tr, Some(ring_wave)));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "dusk" => {
            // Twilight gradient: soothing lavender wash with deep navy shadows
            let p = period(settings.speed, 8.0, 1.2);
            let dusk_lav = scale_color((130, 80, 180), settings.brightness);
            let dusk_navy = scale_color((20, 30, 80), settings.brightness);
            let step = units(p * 0.40);
            let (tr0, _) = fade(dusk_lav, &[(dusk_navy, step), (dusk_lav, step)]);
            let (tr1, _) = fade(dusk_navy, &[(dusk_lav, step), (dusk_navy, step)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(dusk_lav, tr0, None));
            effects.insert(1, Effect::new(dusk_navy, tr1, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if (k.code % 2) == 0 { 0 } else { 1 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "emerald" => {
            // Emerald Isle: deep green field with sparkling crystalline seafoam ripples
            let p = period(settings.speed, 6.0, 0.9);
            let em_deep = scale_color((0, 120, 50), settings.brightness);
            let em_bright = scale_color((0, 240, 120), settings.brightness);
            let seafoam = scale_color((180, 255, 220), settings.brightness);
            let wave_diag = Wave {
                x: 0.0,
                y: 0.0,
                along_x: true,
                along_y: true,
                wavelength: 0.35,
                inward: false,
            };
            let step = units(p * 0.25);
            let (tr, _) = fade(
                em_deep,
                &[(em_bright, step), (seafoam, step), (em_deep, step)],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(em_deep, tr, Some(wave_diag)));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "amethyst" => {
            // Amethyst geode: deep royal violet core with diamond facet shimmers
            let p = period(settings.speed, 5.0, 0.8);
            let violet_deep = scale_color((70, 0, 140), settings.brightness);
            let magenta_bright = scale_color((220, 40, 220), settings.brightness);
            let crystal_white = scale_color((250, 230, 255), settings.brightness);
            let shimmer = units(p * 0.12);
            let glow = units(p * 0.38);
            let rest = units(p * 0.50);
            let mut effects = HashMap::new();
            for slot in 0..STAR_SLOTS {
                let delay = units(slot as f32 * (p * 0.50 / (STAR_SLOTS as f32)));
                let frames = &[
                    (violet_deep, delay),
                    (crystal_white, shimmer),
                    (magenta_bright, glow),
                    (violet_deep, rest),
                ];
                let (tr, _) = fade(violet_deep, frames);
                effects.insert(slot as u8, Effect::new(violet_deep, tr, None));
            }
            let mut codes: Vec<u8> = layout::get_keys().iter().map(|k| k.code).collect();
            let mut rng = rand::rngs::StdRng::seed_from_u64(57);
            codes.shuffle(&mut rng);
            let mut keys = HashMap::new();
            for (i, code) in codes.into_iter().enumerate() {
                keys.insert(
                    code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: (i % STAR_SLOTS) as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "golden" => {
            // Golden hour: warm amber wash propagating inward with soft orange glow
            let p = period(settings.speed, 7.0, 1.1);
            let gold_core = scale_color((255, 190, 0), settings.brightness);
            let gold_amber = scale_color((230, 90, 0), settings.brightness);
            let wave_in = Wave {
                x: 0.5,
                y: 0.5,
                along_x: true,
                along_y: true,
                wavelength: 0.50,
                inward: true,
            };
            let step = units(p * 0.40);
            let (tr, _) = fade(gold_core, &[(gold_amber, step), (gold_core, step)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(gold_core, tr, Some(wave_in)));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "sonar" => {
            // Concentric sonar ping expanding outwards from center
            let p = period(settings.speed, 3.5, 0.5);
            let ping_green = scale_color((0, 255, 120), settings.brightness);
            let ping_dark = scale_color((0, 20, 10), settings.brightness);
            let wave_out = Wave {
                x: 0.5,
                y: 0.5,
                along_x: true,
                along_y: true,
                wavelength: 0.20,
                inward: false,
            };
            let flash = units(p * 0.15);
            let decay = units(p * 0.35);
            let rest = units(p * 0.50);
            let (tr, _) = fade(ping_dark, &[(ping_green, flash), (ping_dark, decay + rest)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(ping_dark, tr, Some(wave_out)));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "lavender" => {
            // Lavender fields: gentle swaying breeze across vertical columns
            let p = period(settings.speed, 6.0, 1.0);
            let lav_soft = scale_color((210, 180, 255), settings.brightness);
            let lav_deep = scale_color((120, 60, 200), settings.brightness);
            let step = units(p * 0.30);
            let (tr0, _) = fade(lav_soft, &[(lav_deep, step), (lav_soft, step)]);
            let (tr1, _) = fade(lav_deep, &[(lav_soft, step), (lav_deep, step)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(lav_soft, tr0, None));
            effects.insert(1, Effect::new(lav_deep, tr1, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = ((k.x as usize) / 3) % 2;
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "inferno" => {
            // Raging hellfire: fast ascending wall of fire surging from spacebar upwards
            let p = period(settings.speed, 2.5, 0.4);
            let flame_red = scale_color((255, 0, 10), settings.brightness);
            let flame_orange = scale_color((255, 100, 0), settings.brightness);
            let flame_yellow = scale_color((255, 230, 0), settings.brightness);
            let wave_up = Wave {
                x: 0.5,
                y: 1.0,
                along_x: false,
                along_y: true,
                wavelength: 0.30,
                inward: false,
            };
            let step = units(p * 0.15);
            let (tr, _) = fade(
                flame_red,
                &[
                    (flame_orange, step),
                    (flame_yellow, step),
                    (flame_red, step * 2),
                ],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(flame_red, tr, Some(wave_up)));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "cotton" => {
            // Cotton candy: alternating baby blue and bubblegum pink zones
            let p = period(settings.speed, 6.0, 1.0);
            let cotton_pink = scale_color((255, 140, 190), settings.brightness);
            let cotton_blue = scale_color((120, 200, 255), settings.brightness);
            let step = units(p * 0.40);
            let (tr_p, _) = fade(cotton_pink, &[(cotton_blue, step), (cotton_pink, step)]);
            let (tr_b, _) = fade(cotton_blue, &[(cotton_pink, step), (cotton_blue, step)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(cotton_pink, tr_p, None));
            effects.insert(1, Effect::new(cotton_blue, tr_b, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if k.x < 10.0 { 0 } else { 1 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "citrus" => {
            // Zesty splash: sparkling lime and orange lemon bursts across numpad and letters
            let p = period(settings.speed, 3.5, 0.5);
            let c_lime = scale_color((120, 255, 0), settings.brightness);
            let c_orange = scale_color((255, 110, 0), settings.brightness);
            let c_lemon = scale_color((255, 235, 0), settings.brightness);
            let step = units(p * 0.20);
            let (tr0, _) = fade(c_lime, &[(c_lemon, step), (c_orange, step), (c_lime, step)]);
            let (tr1, _) = fade(
                c_orange,
                &[(c_lime, step), (c_lemon, step), (c_orange, step)],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(c_lime, tr0, None));
            effects.insert(1, Effect::new(c_orange, tr1, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if (k.code % 3) == 0 { 1 } else { 0 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "monochrome" => {
            // Pure silver ripple: high-contrast monochrome sweep from pitch black to brilliant white
            let p = period(settings.speed, 5.0, 0.8);
            let silver = scale_color((255, 255, 255), settings.brightness);
            let grey = scale_color((60, 60, 60), settings.brightness);
            let step = units(p * 0.25);
            let rest = units(p * 0.50);
            let (tr, _) = fade(BLACK, &[(silver, step), (grey, step), (BLACK, rest)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(BLACK, tr, Some(get_wave(settings))));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "mirage" => {
            // Desert mirage: warm sand dunes with turquoise oasis pool in the center
            let p = period(settings.speed, 7.0, 1.1);
            let sand_gold = scale_color((255, 175, 50), settings.brightness);
            let oasis_teal = scale_color((0, 230, 210), settings.brightness);
            let step = units(p * 0.35);
            let (tr_sand, _) = fade(
                sand_gold,
                &[
                    (scale_color((200, 100, 20), settings.brightness), step),
                    (sand_gold, step),
                ],
            );
            let (tr_oasis, _) = fade(
                oasis_teal,
                &[
                    (scale_color((0, 160, 240), settings.brightness), step),
                    (oasis_teal, step),
                ],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(sand_gold, tr_sand, None));
            effects.insert(1, Effect::new(oasis_teal, tr_oasis, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if k.x >= 6.0 && k.x <= 13.0 && k.y >= 2.0 && k.y <= 4.0 {
                    1
                } else {
                    0
                };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "matrix" => {
            let p = period(settings.speed, 6.0, 1.0);
            let wave_down = Wave {
                x: 0.5,
                y: 0.0,
                along_x: false,
                along_y: true,
                wavelength: settings.wave_size,
                inward: false,
            };
            let g_bright = scale_color((0, 255, 70), settings.brightness);
            let g_dark = scale_color((0, 40, 10), settings.brightness);
            let frames = &[
                (g_bright, units(p * 0.15)),
                (g_dark, units(p * 0.45)),
                (BLACK, units(p * 0.40)),
            ];
            let (transitions, _) = fade(BLACK, frames);
            let eff = Effect::new(BLACK, transitions, Some(wave_down));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "lightning" => {
            let p = period(settings.speed, 3.5, 0.5);
            let t_flash = units(p * 0.06);
            let t_gap = units(p * 0.04);
            let t_wait = units(p * 0.70);
            let c_bolt = scale_color((220, 240, 255), settings.brightness);
            let c_dark = scale_color((10, 15, 30), settings.brightness);
            let frames = &[
                (c_bolt, t_flash),
                (c_dark, t_gap),
                (c_bolt, t_flash),
                (c_dark, t_wait),
            ];
            let (transitions, _) = fade(c_dark, frames);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(c_dark, transitions, None));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "glitch" => {
            let p = period(settings.speed, 3.0, 0.4);
            let step = units(p * 0.12);
            let pause = units(p * 0.4);
            let mut effects = HashMap::new();

            let palette = [
                scale_color((255, 0, 80), settings.brightness),
                scale_color((0, 255, 240), settings.brightness),
                scale_color((255, 255, 255), settings.brightness),
                BLACK,
            ];

            for (slot, &col) in palette.iter().enumerate() {
                let frames = &[(col, step), (BLACK, step), (col, step), (BLACK, pause)];
                let (transitions, _) = fade(BLACK, frames);
                effects.insert(slot as u8, Effect::new(BLACK, transitions, None));
            }

            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = ((k.code as usize) ^ (k.x as usize)) % 4;
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "police" => {
            let p = period(settings.speed, 2.0, 0.4);
            let t_flash = units(p * 0.1);
            let t_gap = units(p * 0.05);
            let t_wait = units(p * 0.35);

            let red = scale_color((255, 0, 0), settings.brightness);
            let blue = scale_color((0, 0, 255), settings.brightness);

            let (tr0, _) = fade(
                BLACK,
                &[
                    (red, t_flash),
                    (BLACK, t_gap),
                    (red, t_flash),
                    (BLACK, t_gap),
                    (BLACK, t_wait * 2),
                ],
            );
            let (tr1, _) = fade(
                BLACK,
                &[
                    (BLACK, t_wait),
                    (blue, t_flash),
                    (BLACK, t_gap),
                    (blue, t_flash),
                    (BLACK, t_gap + t_wait),
                ],
            );

            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(BLACK, tr0, None));
            effects.insert(1, Effect::new(BLACK, tr1, None));

            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if k.x < (layout::WIDTH / 2.0) { 0 } else { 1 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "siren" => {
            let p = period(settings.speed, 2.0, 0.4);
            let t_flash = units(p * 0.15);
            let t_wait = units(p * 0.35);
            let amber = scale_color((255, 140, 0), settings.brightness);
            let (tr0, _) = fade(BLACK, &[(amber, t_flash), (BLACK, t_wait + t_wait)]);
            let (tr1, _) = fade(BLACK, &[(BLACK, t_wait), (amber, t_flash), (BLACK, t_wait)]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(BLACK, tr0, None));
            effects.insert(1, Effect::new(BLACK, tr1, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = if k.x < (layout::WIDTH / 2.0) { 0 } else { 1 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "pulse" => {
            let p = period(settings.speed, 4.0, 0.8);
            let up1 = units(p * 0.12);
            let down1 = units(p * 0.15);
            let up2 = units(p * 0.10);
            let down2 = units(p * 0.20);
            let rest = units(p * 0.43);
            let frames = &[
                (color, up1),
                (BLACK, down1),
                (color, up2),
                (BLACK, down2),
                (BLACK, rest),
            ];
            let (transitions, _) = fade(BLACK, frames);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(BLACK, transitions, None));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "disco" => {
            let p = period(settings.speed, 4.0, 0.8);
            let palette = [
                (255, 0, 100),
                (0, 255, 150),
                (255, 220, 0),
                (0, 150, 255),
                (255, 0, 255),
                (0, 255, 255),
                (255, 120, 0),
                (120, 0, 255),
            ];
            let step = units(p / 4.0);
            let mut effects = HashMap::new();
            for slot in 0..STAR_SLOTS {
                let ca = scale_color(palette[slot % palette.len()], settings.brightness);
                let cb = scale_color(palette[(slot + 3) % palette.len()], settings.brightness);
                let cc = scale_color(palette[(slot + 5) % palette.len()], settings.brightness);
                let frames = &[(cb, step), (cc, step), (ca, step * 2)];
                let (transitions, _) = fade(ca, frames);
                effects.insert(slot as u8, Effect::new(ca, transitions, None));
            }

            let mut codes: Vec<u8> = layout::get_keys().iter().map(|k| k.code).collect();
            let mut rng = rand::rngs::StdRng::seed_from_u64(13);
            codes.shuffle(&mut rng);

            let mut keys = HashMap::new();
            for (i, code) in codes.into_iter().enumerate() {
                keys.insert(
                    code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: (i % STAR_SLOTS) as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "firefly" => {
            let p = period(settings.speed, 5.0, 0.8);
            let rise = p * 0.18;
            let fall = p * 0.32;
            let fly_yellow = scale_color((180, 255, 20), settings.brightness);
            let mut effects = HashMap::new();

            for slot in 0..STAR_SLOTS {
                let offset = (slot as f32) * (p - rise - fall) / ((STAR_SLOTS - 1) as f32);
                let mut frames = Vec::new();
                if offset >= TIME_UNIT {
                    frames.push((BLACK, units(offset)));
                }
                frames.push((fly_yellow, units(rise)));
                frames.push((BLACK, units(fall)));
                let rest = p - offset - rise - fall;
                if rest >= TIME_UNIT {
                    frames.push((BLACK, units(rest)));
                }
                let (transitions, _) = fade(BLACK, &frames);
                effects.insert(slot as u8, Effect::new(BLACK, transitions, None));
            }

            let mut codes: Vec<u8> = layout::get_keys().iter().map(|k| k.code).collect();
            let mut rng = rand::rngs::StdRng::seed_from_u64(99);
            codes.shuffle(&mut rng);

            let mut keys = HashMap::new();
            for (i, code) in codes.into_iter().enumerate() {
                keys.insert(
                    code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: (i % STAR_SLOTS) as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "candle" => {
            let p = period(settings.speed, 3.0, 0.6);
            let colors = [(255, 140, 20), (255, 90, 10), (230, 70, 5), (255, 160, 30)];
            let step = units(p / 4.0);
            let mut effects = HashMap::new();
            for slot in 0..4 {
                let ca = scale_color(colors[slot], settings.brightness);
                let cb = scale_color(colors[(slot + 1) % 4], settings.brightness);
                let (transitions, _) = fade(ca, &[(cb, step), (ca, step)]);
                effects.insert(slot as u8, Effect::new(ca, transitions, None));
            }
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                let slot = ((k.code as usize) ^ 0x05) % 4;
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot as u8,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        "void" => {
            // Cosmic Deep Void: hypnotic vortex spiral twisting inward into a pitch-black abyss
            let p = period(settings.speed, 6.0, 0.9);
            let vortex_neon = scale_color((180, 0, 255), settings.brightness);
            let vortex_cyan = scale_color((0, 190, 255), settings.brightness);
            let vortex_dim = scale_color((25, 0, 50), settings.brightness);
            let step = units(p * 0.25);
            let (tr, _) = fade(
                vortex_dim,
                &[
                    (vortex_neon, step),
                    (vortex_cyan, step),
                    (vortex_dim, step * 2),
                ],
            );
            let wave_spiral = Wave {
                x: 0.5,
                y: 0.5,
                along_x: true,
                along_y: true,
                wavelength: 0.28,
                inward: true,
            };
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(vortex_dim, tr, Some(wave_spiral)));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "stealth" => {
            // Stealth Tactical: covert military night-vision HUD (dark olive tactical background with scanning NVG green crosshairs on WASD & arrows)
            let p = period(settings.speed, 4.0, 0.6);
            let nvg_green = scale_color((0, 255, 60), settings.brightness);
            let nvg_target = scale_color((180, 255, 0), settings.brightness);
            let covert_dim = scale_color((0, 30, 8), settings.brightness);
            let step = units(p * 0.30);
            let (tr_target, _) = fade(nvg_green, &[(nvg_target, step), (nvg_green, step)]);
            let (tr_covert, _) = fade(
                covert_dim,
                &[
                    (scale_color((0, 55, 15), settings.brightness), step),
                    (covert_dim, step),
                ],
            );
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(nvg_green, tr_target, None));
            effects.insert(1, Effect::new(covert_dim, tr_covert, None));
            let mut keys = HashMap::new();
            for k in layout::get_keys() {
                // WASD (0x1a, 0x04, 0x16, 0x07) and Arrows (0x4f, 0x50, 0x51, 0x52) stand out as glowing tactical crosshair targets
                let is_tactical_key = matches!(
                    k.code,
                    0x1a | 0x04 | 0x16 | 0x07 | 0x4f | 0x50 | 0x51 | 0x52
                );
                let slot = if is_tactical_key { 0 } else { 1 };
                keys.insert(
                    k.code,
                    Key {
                        mode: MODE_EFFECT,
                        effect: slot,
                        ..Default::default()
                    },
                );
            }
            Scene { effects, keys }
        }
        _ => {
            let keys = all_keys_matching(|| Key {
                color,
                mode: protocol::MODE_STATIC,
                ..Default::default()
            });
            Scene {
                effects: HashMap::new(),
                keys,
            }
        }
    }
}

pub type EffectSegment = (u16, u16, (i32, i32, i32), (i8, i8, i8));

pub struct CompiledEffect {
    pub segments: Vec<EffectSegment>,
    pub total: u16,
}

pub struct Simulator {
    pub scene: Scene,
    compiled_effects: HashMap<u8, CompiledEffect>,
    delays: HashMap<u8, f32>,
}

impl Simulator {
    pub fn new(scene: Scene) -> Self {
        let mut compiled = HashMap::new();
        for (&slot, effect) in &scene.effects {
            let mut segments = Vec::new();
            let mut t = 0u16;
            let mut value = (
                (effect.start.0 as i32) << 4,
                (effect.start.1 as i32) << 4,
                (effect.start.2 as i32) << 4,
            );
            for tr in &effect.transitions {
                segments.push((t, tr.duration, value, tr.slope));
                value = (
                    value.0 + (tr.slope.0 as i32) * (tr.duration as i32),
                    value.1 + (tr.slope.1 as i32) * (tr.duration as i32),
                    value.2 + (tr.slope.2 as i32) * (tr.duration as i32),
                );
                t = t.saturating_add(tr.duration);
            }
            compiled.insert(
                slot,
                CompiledEffect {
                    segments,
                    total: t.max(1),
                },
            );
        }

        let mut delays = HashMap::new();
        for key in layout::get_keys() {
            if let Some(k) = scene.keys.get(&key.code) {
                if k.mode == protocol::MODE_EFFECT {
                    if let Some(effect) = scene.effects.get(&k.effect) {
                        delays.insert(key.code, Self::calc_delay(&key, effect));
                    }
                }
            }
        }

        Self {
            scene,
            compiled_effects: compiled,
            delays,
        }
    }

    fn calc_delay(key: &layout::KeySpec, effect: &Effect) -> f32 {
        let wave = match &effect.wave {
            Some(w) if w.along_x || w.along_y => w,
            _ => return 0.0,
        };
        let (kx, ky) = layout::normalized_center(key);
        let dx = if wave.along_x {
            (kx - wave.x) * protocol::WAVE_X_MAX as f32
        } else {
            0.0
        };
        let dy = if wave.along_y {
            (ky - wave.y) * protocol::WAVE_Y_MAX as f32
        } else {
            0.0
        };
        let length = protocol::WAVELENGTH_MIN
            + wave.wavelength * (protocol::WAVELENGTH_MAX - protocol::WAVELENGTH_MIN);
        let total: u32 = effect.transitions.iter().map(|tr| tr.duration as u32).sum();
        let phase = dx.hypot(dy) / (length * 4.0);
        let signed_phase = if wave.inward { -phase } else { phase };
        signed_phase * (total as f32)
    }

    pub fn color(&self, code: u8, seconds: f32) -> (u8, u8, u8) {
        let key = match self.scene.keys.get(&code) {
            Some(k) => k,
            None => return BLACK,
        };

        if key.mode == protocol::MODE_EFFECT {
            if let Some(ce) = self.compiled_effects.get(&key.effect) {
                let delay = self.delays.get(&code).copied().unwrap_or(0.0);
                let units = (seconds / TIME_UNIT) - delay;
                let total = ce.total as f32;
                let u = ((units % total) + total) % total;
                let u_u16 = u as u16;

                for &(t0, duration, value, slope) in &ce.segments {
                    if u_u16 < t0.saturating_add(duration) {
                        let dt = (u_u16.saturating_sub(t0)) as i32;
                        let r = (((value.0 + (slope.0 as i32) * dt) / 16).clamp(0, 255)) as u8;
                        let g = (((value.1 + (slope.1 as i32) * dt) / 16).clamp(0, 255)) as u8;
                        let b = (((value.2 + (slope.2 as i32) * dt) / 16).clamp(0, 255)) as u8;
                        return (r, g, b);
                    }
                }
                if let Some(&(_, _, val, _)) = ce.segments.last() {
                    let r = ((val.0 / 16).clamp(0, 255)) as u8;
                    let g = ((val.1 / 16).clamp(0, 255)) as u8;
                    let b = ((val.2 / 16).clamp(0, 255)) as u8;
                    return (r, g, b);
                }
            }
        }
        key.color
    }
}
