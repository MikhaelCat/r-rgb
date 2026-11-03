#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeySpec {
    pub code: u8,
    pub label: &'static str,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub iso_enter: bool,
}

impl KeySpec {
    pub const fn new(code: u8, label: &'static str, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            code,
            label,
            x,
            y,
            w,
            h,
            iso_enter: false,
        }
    }
}

pub const FN_ROW_HEIGHT: f32 = 0.75;
pub const NUMPAD_X: f32 = 15.5;
pub const WIDTH: f32 = 19.5;
pub const HEIGHT: f32 = FN_ROW_HEIGHT + 5.0;

pub fn get_keys() -> Vec<KeySpec> {
    let mut keys = Vec::with_capacity(115);

    let top: &[(u8, &'static str)] = &[
        (0x29, "Esc"),
        (0x3A, "F1"),
        (0x3B, "F2"),
        (0x3C, "F3"),
        (0x3D, "F4"),
        (0x3E, "F5"),
        (0x3F, "F6"),
        (0x40, "F7"),
        (0x41, "F8"),
        (0x42, "F9"),
        (0x43, "F10"),
        (0x44, "F11"),
        (0x45, "F12"),
        (0x46, "PrtSc"),
        (0x47, "ScrLk"),
        (0x48, "Pause"),
        (0x49, "Ins"),
        (0x4C, "Del"),
        (0x4A, "Home"),
        (0x4B, "PgUp"),
        (0x4E, "PgDn"),
        (0x4D, "End"),
    ];
    let step = WIDTH / (top.len() as f32);
    for (i, &(code, label)) in top.iter().enumerate() {
        keys.push(KeySpec::new(
            code,
            label,
            (i as f32) * step,
            0.0,
            step,
            FN_ROW_HEIGHT,
        ));
    }

    let y = FN_ROW_HEIGHT;
    // Row 1
    let row1: &[(u8, &'static str, f32)] = &[
        (0x35, "`", 1.0),
        (0x1E, "1", 1.0),
        (0x1F, "2", 1.0),
        (0x20, "3", 1.0),
        (0x21, "4", 1.0),
        (0x22, "5", 1.0),
        (0x23, "6", 1.0),
        (0x24, "7", 1.0),
        (0x25, "8", 1.0),
        (0x26, "9", 1.0),
        (0x27, "0", 1.0),
        (0x2D, "-", 1.0),
        (0x2E, "=", 1.0),
        (0x2A, "Backspace", 2.0),
    ];
    let mut cur_x = 0.0;
    for &(code, label, w) in row1 {
        keys.push(KeySpec::new(code, label, cur_x, y, w, 1.0));
        cur_x += w;
    }

    // Row 2
    let row2: &[(u8, &'static str, f32)] = &[
        (0x2B, "Tab", 1.5),
        (0x14, "Q", 1.0),
        (0x1A, "W", 1.0),
        (0x08, "E", 1.0),
        (0x15, "R", 1.0),
        (0x17, "T", 1.0),
        (0x1C, "Y", 1.0),
        (0x18, "U", 1.0),
        (0x0C, "I", 1.0),
        (0x12, "O", 1.0),
        (0x13, "P", 1.0),
        (0x2F, "[", 1.0),
        (0x30, "]", 1.0),
    ];
    cur_x = 0.0;
    for &(code, label, w) in row2 {
        keys.push(KeySpec::new(code, label, cur_x, y + 1.0, w, 1.0));
        cur_x += w;
    }
    keys.push(KeySpec {
        code: 0x28,
        label: "Enter",
        x: 13.5,
        y: y + 1.0,
        w: 1.5,
        h: 2.0,
        iso_enter: true,
    });

    // Row 3
    let row3: &[(u8, &'static str, f32)] = &[
        (0x39, "Caps", 1.75),
        (0x04, "A", 1.0),
        (0x16, "S", 1.0),
        (0x07, "D", 1.0),
        (0x09, "F", 1.0),
        (0x0A, "G", 1.0),
        (0x0B, "H", 1.0),
        (0x0D, "J", 1.0),
        (0x0E, "K", 1.0),
        (0x0F, "L", 1.0),
        (0x33, ";", 1.0),
        (0x34, "'", 1.0),
        (0x32, "#", 1.0),
    ];
    cur_x = 0.0;
    for &(code, label, w) in row3 {
        keys.push(KeySpec::new(code, label, cur_x, y + 2.0, w, 1.0));
        cur_x += w;
    }

    // Row 4
    let row4: &[(u8, &'static str, f32)] = &[
        (0xE1, "Shift", 1.25),
        (0x64, "\\", 1.0),
        (0x1D, "Z", 1.0),
        (0x1B, "X", 1.0),
        (0x06, "C", 1.0),
        (0x19, "V", 1.0),
        (0x05, "B", 1.0),
        (0x11, "N", 1.0),
        (0x10, "M", 1.0),
        (0x36, ",", 1.0),
        (0x37, ".", 1.0),
        (0x38, "/", 1.0),
        (0xE5, "Shift", 1.75),
        (0x52, "↑", 1.0),
    ];
    cur_x = 0.0;
    for &(code, label, w) in row4 {
        keys.push(KeySpec::new(code, label, cur_x, y + 3.0, w, 1.0));
        cur_x += w;
    }

    // Row 5
    let row5: &[(u8, &'static str, f32)] = &[
        (0xE0, "Ctrl", 1.25),
        (0xF0, "Fn", 1.0),
        (0xE3, "Super", 1.0),
        (0xE2, "Alt", 1.0),
        (0x2C, "Space", 4.75),
        (0xE6, "AltGr", 1.0),
        (0x65, "Menu", 1.0),
        (0xE4, "Ctrl", 1.0),
        (0x50, "←", 1.0),
        (0x51, "↓", 1.0),
        (0x4F, "→", 1.0),
    ];
    cur_x = 0.0;
    for &(code, label, w) in row5 {
        keys.push(KeySpec::new(code, label, cur_x, y + 4.0, w, 1.0));
        cur_x += w;
    }

    // Numpad
    let nx = NUMPAD_X;
    let num1: &[(u8, &'static str)] = &[(0x53, "Num"), (0x54, "/"), (0x55, "*"), (0x56, "-")];
    for (i, &(code, label)) in num1.iter().enumerate() {
        keys.push(KeySpec::new(code, label, nx + (i as f32), y, 1.0, 1.0));
    }

    let num2: &[(u8, &'static str)] = &[(0x5F, "7"), (0x60, "8"), (0x61, "9")];
    for (i, &(code, label)) in num2.iter().enumerate() {
        keys.push(KeySpec::new(
            code,
            label,
            nx + (i as f32),
            y + 1.0,
            1.0,
            1.0,
        ));
    }
    keys.push(KeySpec::new(0x57, "+", nx + 3.0, y + 1.0, 1.0, 2.0));

    let num3: &[(u8, &'static str)] = &[(0x5C, "4"), (0x5D, "5"), (0x5E, "6")];
    for (i, &(code, label)) in num3.iter().enumerate() {
        keys.push(KeySpec::new(
            code,
            label,
            nx + (i as f32),
            y + 2.0,
            1.0,
            1.0,
        ));
    }

    let num4: &[(u8, &'static str)] = &[(0x59, "1"), (0x5A, "2"), (0x5B, "3")];
    for (i, &(code, label)) in num4.iter().enumerate() {
        keys.push(KeySpec::new(
            code,
            label,
            nx + (i as f32),
            y + 3.0,
            1.0,
            1.0,
        ));
    }
    keys.push(KeySpec::new(0x58, "Enter", nx + 3.0, y + 3.0, 1.0, 2.0));

    keys.push(KeySpec::new(0x62, "0", nx, y + 4.0, 2.0, 1.0));
    keys.push(KeySpec::new(0x63, ".", nx + 2.0, y + 4.0, 1.0, 1.0));

    keys
}

pub fn normalized_center(key: &KeySpec) -> (f32, f32) {
    let cx = key.x + key.w / 2.0;
    let cy = key.y + key.h / 2.0;
    (cx / WIDTH, cy / HEIGHT)
}
