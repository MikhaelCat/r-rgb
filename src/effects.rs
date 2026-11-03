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
    pub description: &'static str,
    pub uses_color: bool,
    pub uses_highlight: bool,
    pub uses_speed: bool,
    pub uses_direction: bool,
}

pub const EFFECTS: &[EffectInfo] = &[
    EffectInfo {
        id: "static",
        name: "Static",
        description: "Solid static color across all keys and lightbars",
        uses_color: true,
        uses_highlight: false,
        uses_speed: false,
        uses_direction: false,
    },
    EffectInfo {
        id: "breathing",
        name: "Breathing",
        description: "Smooth breathing pulse of the chosen color",
        uses_color: true,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "spectrum",
        name: "Color Cycle",
        description: "Synchronized chromatic rainbow cycle across all LEDs",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "rainbow",
        name: "Rainbow Wave",
        description: "Continuous spectrum wave propagating across zones",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "wave",
        name: "Color Wave",
        description: "Unidirectional sweeping pulse of custom color",
        uses_color: true,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "starlight",
        name: "Starlight",
        description: "Gentle random twinkling stars in deep space",
        uses_color: true,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "reactive",
        name: "Reactive",
        description: "Keys illuminate dynamically upon keystrokes",
        uses_color: true,
        uses_highlight: true,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "fire",
        name: "Fireplace",
        description: "Warm flickering embers and rising fire flames",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "aurora",
        name: "Aurora Borealis",
        description: "Mystic ethereal northern lights in cyan, green and purple",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "neon",
        name: "Synthwave Neon",
        description: "Retro 80s outrun wave transitioning hot pink to cyan",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "matrix",
        name: "Matrix Rain",
        description: "Iconic digital cyber glyph rain descending downwards",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "cyberpunk",
        name: "Cyberpunk 2077",
        description: "Electrifying contrast of cyber yellow and neon cyan",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "vaporwave",
        name: "Vaporwave",
        description: "Dreamy nostalgic pastel gradients of violet and peach",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "ocean",
        name: "Deep Ocean",
        description: "Underwater sea depth swell transitioning aquamarine to sapphire",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "sunset",
        name: "Miami Sunset",
        description: "Vibrant tropical gradient of ruby red, tangerine and violet",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "magma",
        name: "Volcanic Magma",
        description: "Molten volcanic lava glow slowly rolling across the chassis",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: true,
    },
    EffectInfo {
        id: "glitch",
        name: "Glitch Shift",
        description: "Jittery chromatic cyber distortion across split clusters",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "police",
        name: "Emergency Strobe",
        description: "Tactical alternating high-frequency red/blue strobe",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "pulse",
        name: "Heartbeat Pulse",
        description: "Cardiogram double pulse with sharp decay",
        uses_color: true,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
    },
    EffectInfo {
        id: "disco",
        name: "Disco Party",
        description: "High-energy multi-zone club strobe dance floor",
        uses_color: false,
        uses_highlight: false,
        uses_speed: true,
        uses_direction: false,
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
            let step = units(p / 5.0);
            let hues = [
                scale_color((0, 255, 128), settings.brightness),
                scale_color((0, 200, 255), settings.brightness),
                scale_color((100, 0, 255), settings.brightness),
                scale_color((0, 255, 200), settings.brightness),
                scale_color((0, 255, 128), settings.brightness),
            ];
            let frames = &[
                (hues[1], step),
                (hues[2], step),
                (hues[3], step),
                (hues[4], step),
            ];
            let (transitions, _) = fade(hues[0], frames);
            let eff = Effect::new(hues[0], transitions, Some(get_wave(settings)));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "neon" => {
            let p = period(settings.speed, 8.0, 1.2);
            let step = units(p / 4.0);
            let c1 = scale_color((255, 0, 128), settings.brightness);
            let c2 = scale_color((0, 220, 255), settings.brightness);
            let c3 = scale_color((130, 0, 255), settings.brightness);
            let frames = &[(c2, step), (c3, step), (c2, step), (c1, step)];
            let (transitions, _) = fade(c1, frames);
            let eff = Effect::new(c1, transitions, Some(get_wave(settings)));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
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
        "cyberpunk" => {
            let p = period(settings.speed, 7.0, 1.0);
            let step = units(p / 4.0);
            let c_yellow = scale_color((255, 235, 0), settings.brightness);
            let c_cyan = scale_color((0, 240, 255), settings.brightness);
            let c_amber = scale_color((255, 90, 0), settings.brightness);
            let frames = &[
                (c_cyan, step),
                (c_amber, step),
                (c_cyan, step),
                (c_yellow, step),
            ];
            let (transitions, _) = fade(c_yellow, frames);
            let eff = Effect::new(c_yellow, transitions, Some(get_wave(settings)));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "vaporwave" => {
            let p = period(settings.speed, 10.0, 1.5);
            let step = units(p / 4.0);
            let c_pink = scale_color((255, 113, 206), settings.brightness);
            let c_blue = scale_color((1, 205, 254), settings.brightness);
            let c_green = scale_color((5, 255, 161), settings.brightness);
            let c_purple = scale_color((185, 103, 255), settings.brightness);
            let frames = &[
                (c_blue, step),
                (c_green, step),
                (c_purple, step),
                (c_pink, step),
            ];
            let (transitions, _) = fade(c_pink, frames);
            let eff = Effect::new(c_pink, transitions, Some(get_wave(settings)));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "ocean" => {
            let p = period(settings.speed, 9.0, 1.3);
            let step = units(p / 4.0);
            let c_teal = scale_color((0, 210, 190), settings.brightness);
            let c_deep = scale_color((0, 50, 180), settings.brightness);
            let c_aqua = scale_color((0, 160, 255), settings.brightness);
            let frames = &[
                (c_deep, step),
                (c_aqua, step),
                (c_deep, step),
                (c_teal, step),
            ];
            let (transitions, _) = fade(c_teal, frames);
            let eff = Effect::new(c_teal, transitions, Some(get_wave(settings)));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "sunset" => {
            let p = period(settings.speed, 8.0, 1.2);
            let step = units(p / 4.0);
            let c_orange = scale_color((255, 60, 0), settings.brightness);
            let c_rose = scale_color((255, 0, 120), settings.brightness);
            let c_gold = scale_color((255, 170, 0), settings.brightness);
            let c_purple = scale_color((120, 0, 160), settings.brightness);
            let frames = &[
                (c_rose, step),
                (c_purple, step),
                (c_gold, step),
                (c_orange, step),
            ];
            let (transitions, _) = fade(c_orange, frames);
            let eff = Effect::new(c_orange, transitions, Some(get_wave(settings)));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
            let keys = all_keys_matching(|| Key {
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            });
            Scene { effects, keys }
        }
        "magma" => {
            let p = period(settings.speed, 7.0, 1.1);
            let step = units(p / 4.0);
            let c_crust = scale_color((40, 0, 0), settings.brightness);
            let c_glow = scale_color((255, 40, 0), settings.brightness);
            let c_fire = scale_color((255, 140, 0), settings.brightness);
            let frames = &[
                (c_glow, step),
                (c_fire, step),
                (c_glow, step),
                (c_crust, step),
            ];
            let (transitions, _) = fade(c_crust, frames);
            let eff = Effect::new(c_crust, transitions, Some(get_wave(settings)));
            let mut effects = HashMap::new();
            effects.insert(0, eff);
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

            for slot in 0..4 {
                let col = palette[slot];
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
