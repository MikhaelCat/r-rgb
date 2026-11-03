use std::collections::HashMap;

pub const FEATURE_SIZE: usize = 524;
pub const OUTPUT_SIZE: usize = 64;
pub const KEYS_PER_PACKET: usize = 42;
pub const MAX_TRANSITIONS: usize = 16;
pub const MAX_EFFECT_SLOTS: usize = 9;

pub const MODE_EFFECT: u8 = 0;
pub const MODE_STATIC: u8 = 1;
pub const MODE_REACTIVE: u8 = 8;

pub const REGIONS: &[(u8, &[u8])] = &[
    (
        0x2A,
        &[
            4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
            27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 58, 59, 60, 61, 62, 63,
        ],
    ),
    (0x0B, &[40, 49, 50, 100, 135, 136, 137, 138, 139, 144, 145]),
    (
        0x18,
        &[
            41, 42, 43, 44, 45, 46, 47, 48, 51, 52, 53, 54, 55, 56, 57, 101, 224, 225, 226, 227,
            228, 229, 230, 240,
        ],
    ),
    (
        0x24,
        &[
            64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
            86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
        ],
    ),
];

pub const WAVE_X_MAX: u16 = 0x105C;
pub const WAVE_Y_MAX: u16 = 0x040D;
pub const WAVELENGTH_MIN: f32 = 0x1F as f32;
pub const WAVELENGTH_MAX: f32 = 0x3E9 as f32;

const KEY_PACKET_TAIL: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x08, 0x39];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key {
    pub color: (u8, u8, u8),
    pub mode: u8,
    pub effect: u8,
    pub reactive_color: (u8, u8, u8),
    pub reactive_ms: u16,
}

impl Default for Key {
    fn default() -> Self {
        Self {
            color: (0, 0, 0),
            mode: MODE_STATIC,
            effect: 0,
            reactive_color: (0, 0, 0),
            reactive_ms: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transition {
    pub slope: (i8, i8, i8),
    pub duration: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wave {
    pub x: f32,
    pub y: f32,
    pub along_x: bool,
    pub along_y: bool,
    pub wavelength: f32,
    pub inward: bool,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            along_x: true,
            along_y: false,
            wavelength: 0.5,
            inward: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Effect {
    pub start: (u8, u8, u8),
    pub transitions: Vec<Transition>,
    pub wave: Option<Wave>,
}

impl Effect {
    pub fn new(start: (u8, u8, u8), transitions: Vec<Transition>, wave: Option<Wave>) -> Self {
        Self {
            start,
            transitions,
            wave,
        }
    }
}

fn clamp<T: PartialOrd>(value: T, low: T, high: T) -> T {
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

pub fn key_packet(region: u8, keys: &HashMap<u8, Key>) -> Result<[u8; FEATURE_SIZE], &'static str> {
    if keys.len() > KEYS_PER_PACKET {
        return Err("too many keys for one packet");
    }
    let mut packet = [0u8; FEATURE_SIZE];
    packet[0] = 0x0E;
    packet[1] = 0x00;
    packet[2] = region;
    packet[3] = 0x00;

    let mut offset = 4;
    for (&code, key) in keys.iter() {
        packet[offset] = key.color.0;
        packet[offset + 1] = key.color.1;
        packet[offset + 2] = key.color.2;

        packet[offset + 3] = key.reactive_color.0;
        packet[offset + 4] = key.reactive_color.1;
        packet[offset + 5] = key.reactive_color.2;

        let ms_bytes = key.reactive_ms.to_le_bytes();
        packet[offset + 6] = ms_bytes[0];
        packet[offset + 7] = ms_bytes[1];

        packet[offset + 8] = key.effect;
        packet[offset + 9] = key.mode;
        packet[offset + 10] = 0x00;
        packet[offset + 11] = code;

        offset += 12;
    }

    // Tail begins at index 4 + KEYS_PER_PACKET * 12 = 4 + 504 = 508
    packet[508..524].copy_from_slice(&KEY_PACKET_TAIL);
    Ok(packet)
}

pub fn key_packets(keys: &HashMap<u8, Key>) -> Vec<[u8; FEATURE_SIZE]> {
    let mut packets = Vec::with_capacity(REGIONS.len());
    for &(region, codes) in REGIONS {
        let subset: HashMap<u8, Key> = codes
            .iter()
            .filter_map(|&code| keys.get(&code).map(|&k| (code, k)))
            .collect();
        if !subset.is_empty() {
            if let Ok(pkt) = key_packet(region, &subset) {
                packets.push(pkt);
            }
        }
    }
    packets
}

pub fn effect_packet(slot: u8, effect: &Effect) -> Result<[u8; FEATURE_SIZE], &'static str> {
    if effect.transitions.is_empty() || effect.transitions.len() > MAX_TRANSITIONS {
        return Err("an effect needs 1..16 transitions");
    }
    let mut packet = [0u8; FEATURE_SIZE];
    packet[0] = 0x0B;

    for (i, tr) in effect.transitions.iter().enumerate() {
        let offset = 2 + i * 8;
        packet[offset] = if i == 0 { slot } else { i as u8 };
        packet[offset + 2] = tr.slope.0 as u8;
        packet[offset + 3] = tr.slope.1 as u8;
        packet[offset + 4] = tr.slope.2 as u8;

        let dur = clamp(tr.duration, 1, 0xFFFF);
        let dur_bytes = dur.to_le_bytes();
        packet[offset + 6] = dur_bytes[0];
        packet[offset + 7] = dur_bytes[1];
    }

    let start_channels = [effect.start.0, effect.start.1, effect.start.2];
    for (c, &val) in start_channels.iter().enumerate() {
        let fixed = (val as u16) << 4;
        let bytes = fixed.to_le_bytes();
        packet[0x84 + 2 * c] = bytes[0];
        packet[0x85 + 2 * c] = bytes[1];
    }
    packet[0x8A] = 0xFF;

    if let Some(wave) = &effect.wave {
        let wx = (clamp(wave.x, 0.0, 1.0) * WAVE_X_MAX as f32).round() as u16;
        let wy = (clamp(wave.y, 0.0, 1.0) * WAVE_Y_MAX as f32).round() as u16;
        packet[0x8C..0x8E].copy_from_slice(&wx.to_le_bytes());
        packet[0x8E..0x90].copy_from_slice(&wy.to_le_bytes());
        packet[0x90..0x92].copy_from_slice(&(wave.along_x as u16).to_le_bytes());
        packet[0x92..0x94].copy_from_slice(&(wave.along_y as u16).to_le_bytes());

        let length =
            WAVELENGTH_MIN + clamp(wave.wavelength, 0.0, 1.0) * (WAVELENGTH_MAX - WAVELENGTH_MIN);
        let wlen = length.round() as u16;
        packet[0x94..0x96].copy_from_slice(&wlen.to_le_bytes());
        packet[0x9A] = wave.inward as u8;
    }

    packet[0x96] = effect.transitions.len() as u8;
    let total: u32 = effect.transitions.iter().map(|tr| tr.duration as u32).sum();
    let total_clamped = clamp(total, 0, 0xFFFF) as u16;
    packet[0x98..0x9A].copy_from_slice(&total_clamped.to_le_bytes());

    Ok(packet)
}

pub fn empty_effect_packet(slot: u8) -> [u8; FEATURE_SIZE] {
    let mut packet = [0u8; FEATURE_SIZE];
    packet[0] = 0x0B;
    packet[2] = slot;
    packet
}

pub fn refresh_packet() -> [u8; OUTPUT_SIZE] {
    let mut packet = [0u8; OUTPUT_SIZE];
    packet[0] = 0x09;
    packet
}

pub fn fade(
    start: (u8, u8, u8),
    keyframes: &[((u8, u8, u8), u16)],
) -> (Vec<Transition>, (u8, u8, u8)) {
    let mut current = [
        (start.0 as i32) << 4,
        (start.1 as i32) << 4,
        (start.2 as i32) << 4,
    ];
    let mut transitions = Vec::with_capacity(keyframes.len());

    for &(color, mut duration) in keyframes {
        let target = [
            (color.0 as i32) << 4,
            (color.1 as i32) << 4,
            (color.2 as i32) << 4,
        ];
        duration = duration.max(1);
        let need = [
            (target[0] - current[0]).abs(),
            (target[1] - current[1]).abs(),
            (target[2] - current[2]).abs(),
        ]
        .into_iter()
        .max()
        .unwrap_or(0);

        let min_duration = (need + 126) / 127;
        duration = duration.max(min_duration as u16);

        let s0 = (target[0] - current[0]) / (duration as i32);
        let s1 = (target[1] - current[1]) / (duration as i32);
        let s2 = (target[2] - current[2]) / (duration as i32);

        current[0] += s0 * (duration as i32);
        current[1] += s1 * (duration as i32);
        current[2] += s2 * (duration as i32);

        transitions.push(Transition {
            slope: (s0 as i8, s1 as i8, s2 as i8),
            duration,
        });
    }

    let end_color = (
        (current[0] >> 4) as u8,
        (current[1] >> 4) as u8,
        (current[2] >> 4) as u8,
    );
    (transitions, end_color)
}
