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

fn make_wave_palette(
    colors: &[(u8, u8, u8)],
    brightness: f32,
    p: f32,
    wave: Wave,
) -> (HashMap<u8, Effect>, HashMap<u8, Key>) {
    let step = units(p / colors.len() as f32);
    let scaled: Vec<(u8, u8, u8)> = colors.iter().map(|&c| scale_color(c, brightness)).collect();
    let mut frames = Vec::new();
    for &col in scaled.iter().skip(1) {
        frames.push((col, step));
    }
    frames.push((scaled[0], step));
    let (transitions, _) = fade(scaled[0], &frames);
    let eff = Effect::new(scaled[0], transitions, Some(wave));
    let mut effects = HashMap::new();
    effects.insert(0, eff);
    let keys = all_keys_matching(|| Key {
        mode: MODE_EFFECT,
        effect: 0,
        ..Default::default()
    });
    (effects, keys)
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
            let p = period(settings.speed, 16.0, 2.5);
            let (effects, keys) = make_wave_palette(
                &[(0, 255, 128), (0, 200, 255), (100, 0, 255), (0, 255, 200)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "neon" => {
            let p = period(settings.speed, 8.0, 1.2);
            let (effects, keys) = make_wave_palette(
                &[(255, 0, 128), (0, 220, 255), (130, 0, 255)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "cyberpunk" => {
            let p = period(settings.speed, 7.0, 1.0);
            let (effects, keys) = make_wave_palette(
                &[(255, 235, 0), (0, 240, 255), (255, 90, 0)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "vaporwave" => {
            let p = period(settings.speed, 10.0, 1.5);
            let (effects, keys) = make_wave_palette(
                &[
                    (255, 113, 206),
                    (1, 205, 254),
                    (5, 255, 161),
                    (185, 103, 255),
                ],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "ocean" => {
            let p = period(settings.speed, 9.0, 1.3);
            let (effects, keys) = make_wave_palette(
                &[(0, 210, 190), (0, 50, 180), (0, 160, 255)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "sunset" => {
            let p = period(settings.speed, 8.0, 1.2);
            let (effects, keys) = make_wave_palette(
                &[(255, 60, 0), (255, 0, 120), (120, 0, 160), (255, 170, 0)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "magma" => {
            let p = period(settings.speed, 7.0, 1.1);
            let (effects, keys) = make_wave_palette(
                &[(40, 0, 0), (255, 40, 0), (255, 140, 0)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "blizzard" => {
            let p = period(settings.speed, 8.0, 1.2);
            let (effects, keys) = make_wave_palette(
                &[(220, 245, 255), (130, 220, 255), (0, 140, 255)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "nebula" => {
            let p = period(settings.speed, 11.0, 1.6);
            let (effects, keys) = make_wave_palette(
                &[(230, 0, 150), (50, 0, 180), (140, 0, 255)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "toxic" => {
            let p = period(settings.speed, 6.0, 0.9);
            let (effects, keys) = make_wave_palette(
                &[(57, 255, 20), (210, 255, 0), (0, 180, 20)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "sakura" => {
            let p = period(settings.speed, 9.0, 1.4);
            let (effects, keys) = make_wave_palette(
                &[(255, 183, 197), (255, 240, 245), (255, 105, 180)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "solar" => {
            let p = period(settings.speed, 6.5, 1.0);
            let (effects, keys) = make_wave_palette(
                &[(255, 255, 230), (255, 190, 0), (255, 70, 0)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "forest" => {
            let p = period(settings.speed, 10.0, 1.5);
            let (effects, keys) = make_wave_palette(
                &[(0, 160, 60), (20, 70, 20), (40, 210, 80)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "galaxy" => {
            let p = period(settings.speed, 12.0, 1.8);
            let (effects, keys) = make_wave_palette(
                &[(25, 0, 75), (0, 180, 220), (160, 0, 230)],
                settings.brightness,
                p,
                get_wave(settings),
            );
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
            let p = period(settings.speed, 9.0, 1.3);
            let (effects, keys) = make_wave_palette(
                &[(40, 20, 80), (130, 80, 180), (20, 40, 100)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "emerald" => {
            let p = period(settings.speed, 8.0, 1.2);
            let (effects, keys) = make_wave_palette(
                &[(0, 200, 100), (0, 60, 40), (120, 255, 180)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "amethyst" => {
            let p = period(settings.speed, 8.5, 1.2);
            let (effects, keys) = make_wave_palette(
                &[(153, 50, 204), (75, 0, 130), (218, 112, 214)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "golden" => {
            let p = period(settings.speed, 7.5, 1.1);
            let (effects, keys) = make_wave_palette(
                &[(255, 215, 0), (255, 140, 0), (180, 90, 0)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "sonar" => {
            let p = period(settings.speed, 5.0, 0.8);
            let wave_out = Wave {
                x: 0.5,
                y: 0.5,
                along_x: true,
                along_y: true,
                wavelength: settings.wave_size,
                inward: false,
            };
            let (effects, keys) = make_wave_palette(
                &[(0, 255, 150), (0, 20, 10), (0, 20, 10)],
                settings.brightness,
                p,
                wave_out,
            );
            Scene { effects, keys }
        }
        "lavender" => {
            let p = period(settings.speed, 9.0, 1.4);
            let (effects, keys) = make_wave_palette(
                &[(230, 230, 250), (147, 112, 219), (75, 0, 130)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "inferno" => {
            let p = period(settings.speed, 4.0, 0.6);
            let (effects, keys) = make_wave_palette(
                &[(255, 0, 40), (255, 90, 0), (80, 0, 0)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "cotton" => {
            let p = period(settings.speed, 8.0, 1.2);
            let (effects, keys) = make_wave_palette(
                &[(255, 182, 193), (173, 216, 230), (255, 255, 255)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "citrus" => {
            let p = period(settings.speed, 6.0, 0.9);
            let (effects, keys) = make_wave_palette(
                &[(255, 255, 0), (50, 205, 50), (255, 69, 0)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "monochrome" => {
            let p = period(settings.speed, 7.0, 1.0);
            let (effects, keys) = make_wave_palette(
                &[(255, 255, 255), (100, 100, 100), (0, 0, 0)],
                settings.brightness,
                p,
                get_wave(settings),
            );
            Scene { effects, keys }
        }
        "mirage" => {
            let p = period(settings.speed, 8.5, 1.3);
            let (effects, keys) = make_wave_palette(
                &[(237, 201, 175), (255, 160, 40), (64, 224, 208)],
                settings.brightness,
                p,
                get_wave(settings),
            );
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
            let p = period(settings.speed, 12.0, 2.0);
            let v_deep = scale_color((20, 0, 45), settings.brightness);
            let (transitions, _) =
                fade(BLACK, &[(v_deep, units(p * 0.5)), (BLACK, units(p * 0.5))]);
            let mut effects = HashMap::new();
            effects.insert(0, Effect::new(BLACK, transitions, None));
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "stealth" => {
            let dimmed_red = scale_color((70, 0, 0), settings.brightness);
            let keys = all_keys_matching(|| Key {
                color: dimmed_red,
                mode: protocol::MODE_STATIC,
                ..Default::default()
            });
            Scene {
                effects: HashMap::new(),
                keys,
            }
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
