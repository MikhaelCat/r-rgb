use std::fs::{self, File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use nix::ioctl_readwrite_bad;

use crate::effects::Scene;
use crate::protocol::{self, empty_effect_packet, refresh_packet, Key, MAX_EFFECT_SLOTS};

pub const REPORT_GAP: Duration = Duration::from_millis(5);

// Known Vendor IDs:
// 0x1038: Hardware Controller VID
// 0x1770: MSI Micro Star International
pub const KNOWN_VENDOR_IDS: &[u16] = &[0x1038, 0x1770];

// HID IOC SFEATURE ioctl
// Python definition: (3 << 30) | (length << 16) | (ord('H') << 8) | 0x06
// For length 524 (0x020c): (3 << 30) | (0x020c << 16) | (0x48 << 8) | 0x06
const HIDIOCSFEATURE_524: u64 =
    (3u64 << 30) | ((protocol::FEATURE_SIZE as u64) << 16) | (0x48 << 8) | 0x06;

ioctl_readwrite_bad!(
    hidiocsfeature,
    HIDIOCSFEATURE_524,
    [u8; protocol::FEATURE_SIZE]
);

pub const SUPPORTED_PIDS: &[(u16, u16, &str)] = &[
    (0x1038, 0x113A, "Per-Key RGB Keyboard"),
    (0x1038, 0x1122, "Per-Key RGB Keyboard (MSI)"),
    (0x1038, 0x1161, "Front Lightbar / Logo (MSI)"),
    (0x1770, 0xFF00, "MSI 3-Zone Gaming Keyboard"),
];

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub path: PathBuf,
    pub vendor_id: u16,
    pub product_id: u16,
    pub name: &'static str,
}

pub fn list_all_hid_candidates() -> Vec<(String, u16, u16, String, PathBuf)> {
    let mut results = Vec::new();
    let entries = match fs::read_dir("/sys/class/hidraw") {
        Ok(e) => e,
        Err(_) => return results,
    };

    let mut nodes: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    nodes.sort();

    for node in nodes {
        let uevent_path = node.join("device").join("uevent");
        let content = match fs::read_to_string(&uevent_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let mut hid_id = None;
        let mut name = "Unknown HID Device".to_string();
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("HID_ID=") {
                hid_id = Some(val.trim());
            } else if let Some(n) = line.strip_prefix("HID_NAME=") {
                name = n.trim().to_string();
            }
        }

        if let Some(id_str) = hid_id {
            let parts: Vec<&str> = id_str.split(':').collect();
            if parts.len() == 3 {
                let vid = u16::from_str_radix(parts[1], 16).unwrap_or(0);
                let pid = u16::from_str_radix(parts[2], 16).unwrap_or(0);
                let file_name = node.file_name().unwrap_or_default();
                let dev_path = PathBuf::from("/dev").join(file_name);
                results.push((name, vid, pid, id_str.to_string(), dev_path));
            }
        }
    }
    results
}

pub fn find_all_devices() -> Vec<DeviceInfo> {
    let mut devices = Vec::new();

    let hidraw_entries = match fs::read_dir("/sys/class/hidraw") {
        Ok(entries) => entries,
        Err(_) => return devices,
    };

    let mut nodes: Vec<PathBuf> = hidraw_entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    nodes.sort();

    for node in nodes {
        let uevent_path = node.join("device").join("uevent");
        let content = match fs::read_to_string(&uevent_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let mut hid_id = None;
        let mut dev_name = "Generic HID Device".to_string();
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("HID_ID=") {
                hid_id = Some(val.trim());
            } else if let Some(n) = line.strip_prefix("HID_NAME=") {
                dev_name = n.trim().to_string();
            }
        }

        let (bus, vid, pid) = match hid_id {
            Some(id_str) => {
                let parts: Vec<&str> = id_str.split(':').collect();
                if parts.len() != 3 {
                    continue;
                }
                let bus = u16::from_str_radix(parts[0], 16).unwrap_or(0);
                let vid = u16::from_str_radix(parts[1], 16).unwrap_or(0);
                let pid = u16::from_str_radix(parts[2], 16).unwrap_or(0);
                (bus, vid, pid)
            }
            None => continue,
        };

        if bus != 0x03 {
            continue;
        }

        let is_known = SUPPORTED_PIDS
            .iter()
            .find(|(v, p, _)| *v == vid && *p == pid);
        let name_lower = dev_name.to_lowercase();
        let is_rgb_name = name_lower.contains("keyboard")
            || name_lower.contains("klc")
            || name_lower.contains("alc")
            || name_lower.contains("rgb");

        if is_known.is_none() && !KNOWN_VENDOR_IDS.contains(&vid) && !is_rgb_name {
            continue;
        }

        let resolved_name = if let Some((_, _, label)) = is_known {
            *label
        } else {
            "RGB Compatible Controller"
        };

        let real_dev = match fs::canonicalize(node.join("device")) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let dev_str = real_dev.to_string_lossy();
        let parent_ends_with_1_0 = dev_str
            .split('/')
            .rev()
            .nth(1)
            .map(|p| p.ends_with(":1.0"))
            .unwrap_or(false);

        if parent_ends_with_1_0 {
            let file_name = node.file_name().unwrap_or_default();
            let dev_path = PathBuf::from("/dev").join(file_name);
            devices.push(DeviceInfo {
                path: dev_path,
                vendor_id: vid,
                product_id: pid,
                name: resolved_name,
            });
        }
    }

    devices
}

pub struct Controller {
    file: File,
    pub info: DeviceInfo,
}

impl Controller {
    pub fn open(info: DeviceInfo) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_CLOEXEC)
            .open(&info.path)?;
        Ok(Self { file, info })
    }

    pub fn send_feature(
        &mut self,
        mut packet: [u8; protocol::FEATURE_SIZE],
    ) -> std::io::Result<()> {
        let fd = self.file.as_raw_fd();
        let res = unsafe { hidiocsfeature(fd, &mut packet) };
        if res.is_err() {
            return Err(std::io::Error::last_os_error());
        }
        thread::sleep(REPORT_GAP);
        Ok(())
    }

    pub fn send_output(&mut self, packet: &[u8]) -> std::io::Result<()> {
        use std::io::Write;
        self.file.write_all(packet)?;
        thread::sleep(REPORT_GAP);
        Ok(())
    }

    pub fn is_alc(&self) -> bool {
        self.info.product_id == 0x1161
            || self.info.name.to_lowercase().contains("lightbar")
            || self.info.name.to_lowercase().contains("logo")
            || self.info.name.to_lowercase().contains("alc")
    }

    pub fn apply_scene(&mut self, scene: &Scene) -> std::io::Result<()> {
        if self.is_alc() {
            self.apply_scene_alc(scene)
        } else {
            self.apply_scene_klc(scene)
        }
    }

    pub fn apply_scene_klc(&mut self, scene: &Scene) -> std::io::Result<()> {
        for slot in 0..MAX_EFFECT_SLOTS as u8 {
            let packet = match scene.effects.get(&slot) {
                Some(effect) => protocol::effect_packet(slot, effect)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?,
                None => empty_effect_packet(slot),
            };
            self.send_feature(packet)?;
        }

        for packet in protocol::key_packets(&scene.keys) {
            self.send_feature(packet)?;
        }

        self.send_output(&refresh_packet())?;
        Ok(())
    }

    pub fn apply_scene_alc(&mut self, scene: &Scene) -> std::io::Result<()> {
        // Send effect packets if present
        for slot in 0..MAX_EFFECT_SLOTS as u8 {
            let packet = match scene.effects.get(&slot) {
                Some(effect) => protocol::effect_packet(slot, effect)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?,
                None => empty_effect_packet(slot),
            };
            self.send_feature(packet)?;
        }

        // Determine ALC zone colors
        let default_color = scene
            .effects
            .get(&0)
            .map(|e| e.start)
            .or_else(|| scene.keys.values().next().map(|k| k.color))
            .unwrap_or((0, 255, 0));

        let left_color = scene
            .keys
            .get(&0x1D)
            .or_else(|| scene.keys.get(&0x04))
            .map(|k| k.color)
            .unwrap_or(default_color);
        let center_color = scene
            .keys
            .get(&0x06)
            .or_else(|| scene.keys.get(&0x2C))
            .map(|k| k.color)
            .unwrap_or(default_color);
        let right_color = scene
            .keys
            .get(&0x38)
            .or_else(|| scene.keys.get(&0x28))
            .map(|k| k.color)
            .unwrap_or(default_color);
        let dragon_color = scene
            .keys
            .get(&0x2C)
            .or_else(|| scene.keys.get(&0x0B))
            .map(|k| k.color)
            .unwrap_or(default_color);

        let z0 = scene.keys.get(&0).map(|k| k.color).unwrap_or(left_color);
        let z1 = scene.keys.get(&1).map(|k| k.color).unwrap_or(center_color);
        let z2 = scene.keys.get(&2).map(|k| k.color).unwrap_or(right_color);
        let z3 = scene.keys.get(&3).map(|k| k.color).unwrap_or(dragon_color);
        let z4 = scene.keys.get(&4).map(|k| k.color).unwrap_or(right_color);
        let z5 = scene.keys.get(&5).map(|k| k.color).unwrap_or(dragon_color);

        let alc_zones = [
            (0u8, z0),
            (1u8, z1),
            (2u8, z2),
            (3u8, z3),
            (4u8, z4),
            (5u8, z5),
        ];

        // Send Report 0x0E (effect bindings)
        let mut alc_keys = scene.keys.clone();
        for &(zid, col) in &alc_zones {
            alc_keys.entry(zid).or_insert(Key {
                color: col,
                mode: if scene.effects.is_empty() {
                    protocol::MODE_STATIC
                } else {
                    protocol::MODE_EFFECT
                },
                effect: 0,
                ..Default::default()
            });
        }
        let pkt_e = protocol::alc_key_packet(&alc_keys);
        self.send_feature(pkt_e)?;

        // Send Report 0x0C (direct hardware RGB feature report) for instant illumination
        let pkt_c = protocol::alc_report_0c(&alc_zones);
        self.send_feature(pkt_c)?;

        // Commit output
        let _ = self.send_output(&refresh_packet());

        Ok(())
    }
}

pub struct MultiDeviceManager {
    controllers: Vec<Controller>,
}

fn acquire_device_lock() -> Option<File> {
    let candidates: [Option<PathBuf>; 3] = [
        std::env::var("XDG_RUNTIME_DIR")
            .ok()
            .map(|dir| PathBuf::from(dir).join("rrgb.lock")),
        Some(PathBuf::from("/tmp/rrgb.lock")),
        Some(PathBuf::from("/run/lock/rrgb.lock")),
    ];

    for path in candidates.into_iter().flatten() {
        if let Ok(file) = OpenOptions::new().read(true).open(&path) {
            #[allow(deprecated)]
            if nix::fcntl::flock(file.as_raw_fd(), nix::fcntl::FlockArg::LockExclusive).is_ok() {
                return Some(file);
            }
        }

        if let Ok(file) = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
        {
            #[allow(deprecated)]
            if nix::fcntl::flock(file.as_raw_fd(), nix::fcntl::FlockArg::LockExclusive).is_ok() {
                return Some(file);
            }
        }
    }

    None
}

impl MultiDeviceManager {
    pub fn load_all() -> std::io::Result<Self> {
        let found = find_all_devices();
        if found.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No RGB controllers found",
            ));
        }

        let mut controllers = Vec::new();
        for dev in found {
            if let Ok(ctrl) = Controller::open(dev) {
                controllers.push(ctrl);
            }
        }

        if controllers.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Permission denied or unable to open any RGB HID nodes",
            ));
        }

        Ok(Self { controllers })
    }

    pub fn apply_scene(&mut self, scene: &Scene) -> std::io::Result<()> {
        let lock_file = acquire_device_lock();

        let mut success = false;
        let mut last_err = None;
        for ctrl in &mut self.controllers {
            match ctrl.apply_scene(scene) {
                Ok(_) => success = true,
                Err(e) => last_err = Some(e),
            }
        }

        if let Some(f) = lock_file {
            #[allow(deprecated)]
            let _ = nix::fcntl::flock(f.as_raw_fd(), nix::fcntl::FlockArg::Unlock);
        }

        if !success {
            if let Some(err) = last_err {
                return Err(err);
            }
        }

        Ok(())
    }

    pub fn apply_scenes_zoned(
        &mut self,
        kb_scene: &Scene,
        bar_scene: &Scene,
    ) -> std::io::Result<()> {
        let lock_file = acquire_device_lock();

        let mut success = false;
        let mut last_err = None;
        for ctrl in &mut self.controllers {
            let is_bar = ctrl.is_alc();
            let target_scene = if is_bar { bar_scene } else { kb_scene };
            match ctrl.apply_scene(target_scene) {
                Ok(_) => success = true,
                Err(e) => last_err = Some(e),
            }
        }

        if let Some(f) = lock_file {
            #[allow(deprecated)]
            let _ = nix::fcntl::flock(f.as_raw_fd(), nix::fcntl::FlockArg::Unlock);
        }

        if !success {
            if let Some(err) = last_err {
                return Err(err);
            }
        }

        Ok(())
    }

    pub fn devices(&self) -> &[Controller] {
        &self.controllers
    }
}
