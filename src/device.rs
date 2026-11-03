use std::fs::{self, File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use nix::ioctl_readwrite_bad;

use crate::effects::Scene;
use crate::protocol::{self, empty_effect_packet, refresh_packet, MAX_EFFECT_SLOTS};

pub const VENDOR_ID: u16 = 0x1038;
pub const REPORT_GAP: Duration = Duration::from_millis(5);

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

pub const SUPPORTED_PIDS: &[(u16, &str)] = &[
    (0x113A, "SteelSeries KLC"),
    (0x1122, "SteelSeries KLC (MSI Keyboard)"),
    (0x1161, "SteelSeries ALC (MSI Lightbar / Logo)"),
];

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub path: PathBuf,
    pub vendor_id: u16,
    pub product_id: u16,
    pub name: &'static str,
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
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("HID_ID=") {
                hid_id = Some(val.trim());
                break;
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

        if bus != 0x03 || vid != VENDOR_ID {
            continue;
        }

        let matched = SUPPORTED_PIDS.iter().find(|(p, _)| *p == pid);
        if let Some((_, name)) = matched {
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
                    name,
                });
            }
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

    pub fn apply_scene(&mut self, scene: &Scene) -> std::io::Result<()> {
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
}

pub struct MultiDeviceManager {
    controllers: Vec<Controller>,
}

impl MultiDeviceManager {
    pub fn load_all() -> std::io::Result<Self> {
        let found = find_all_devices();
        if found.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No SteelSeries KLC/ALC controllers found",
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
                "Permission denied or unable to open any SteelSeries HID nodes",
            ));
        }

        Ok(Self { controllers })
    }

    pub fn apply_scene(&mut self, scene: &Scene) -> std::io::Result<()> {
        let lock_dir = if Path::new("/run/lock").is_dir() {
            "/run/lock"
        } else {
            "/tmp"
        };
        let lock_path = Path::new(lock_dir).join("keyglow.lock");
        // Open read-only if it exists (sufficient for flock), or create if not present
        #[allow(deprecated)]
        let lock_file = OpenOptions::new()
            .read(true)
            .open(&lock_path)
            .or_else(|_| {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(&lock_path)
            })?;

        #[allow(deprecated)]
        nix::fcntl::flock(lock_file.as_raw_fd(), nix::fcntl::FlockArg::LockExclusive)
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        for ctrl in &mut self.controllers {
            let _ = ctrl.apply_scene(scene);
        }

        #[allow(deprecated)]
        let _ = nix::fcntl::flock(lock_file.as_raw_fd(), nix::fcntl::FlockArg::Unlock);
        Ok(())
    }

    pub fn devices(&self) -> &[Controller] {
        &self.controllers
    }
}
