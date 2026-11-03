use std::collections::HashMap;
use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::device::MultiDeviceManager;
use crate::effects::Scene;
use crate::layout::{self, HEIGHT};
use crate::protocol::{self, Key, MODE_EFFECT};

pub struct AudioVisualizer {
    running: Arc<AtomicBool>,
    bands: Arc<Mutex<[f32; 16]>>,
    worker: Option<thread::JoinHandle<()>>,
}

impl Default for AudioVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioVisualizer {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            bands: Arc::new(Mutex::new([0.0; 16])),
            worker: None,
        }
    }

    pub fn bands(&self) -> [f32; 16] {
        *self.bands.lock().unwrap()
    }

    pub fn start(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let bands = self.bands.clone();

        self.worker = Some(thread::spawn(move || {
            Self::capture_loop(running, bands);
        }));
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(w) = self.worker.take() {
            let _ = w.join();
        }
        let mut b = self.bands.lock().unwrap();
        *b = [0.0; 16];
    }

    fn get_monitor_source() -> Option<String> {
        let output = Command::new("pactl")
            .arg("get-default-sink")
            .output()
            .ok()?;
        if output.status.success() {
            let sink = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !sink.is_empty() {
                return Some(format!("{}.monitor", sink));
            }
        }
        None
    }

    fn spawn_recorder(source: Option<&str>) -> Option<Child> {
        // Try pw-record first
        let mut cmd = Command::new("pw-record");
        if let Some(src) = source {
            cmd.args(["--target", src]);
        }
        cmd.args(["--rate", "22050", "--channels", "1", "--format", "s16", "-"]);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());

        if let Ok(child) = cmd.spawn() {
            return Some(child);
        }

        // Fallback to parec
        let mut cmd = Command::new("parec");
        if let Some(src) = source {
            cmd.args(["-d", src]);
        } else {
            cmd.args(["-d", "@DEFAULT_MONITOR@"]);
        }
        cmd.args(["--rate=22050", "--channels=1", "--format=s16le"]);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());

        cmd.spawn().ok()
    }

    fn capture_loop(running: Arc<AtomicBool>, bands_lock: Arc<Mutex<[f32; 16]>>) {
        let monitor = Self::get_monitor_source();
        let mut child = match Self::spawn_recorder(monitor.as_deref()) {
            Some(c) => c,
            None => {
                // If neither pw-record nor parec available, exit thread
                return;
            }
        };

        let mut stdout = match child.stdout.take() {
            Some(s) => s,
            None => return,
        };

        // 512 samples of 16-bit PCM (1024 bytes) = ~23ms at 22050 Hz (~43 FPS)
        const SAMPLES: usize = 512;
        let mut buf = [0u8; SAMPLES * 2];
        let mut decay_bands = [0.0f32; 16];

        while running.load(Ordering::SeqCst) {
            if stdout.read_exact(&mut buf).is_err() {
                thread::sleep(Duration::from_millis(50));
                continue;
            }

            // Convert raw PCM s16le to f32 samples
            let mut raw_samples = [0.0f32; SAMPLES];
            for i in 0..SAMPLES {
                let sample_i16 = i16::from_le_bytes([buf[i * 2], buf[i * 2 + 1]]);
                raw_samples[i] = (sample_i16 as f32) / 32768.0;
            }

            // Simple Discrete Spectral Energy Distribution (16 frequency buckets)
            let mut current_bands = [0.0f32; 16];
            let chunk_size = SAMPLES / 16;
            for (b, band) in current_bands.iter_mut().enumerate() {
                let mut energy = 0.0f32;
                let start = b * chunk_size;
                let end = start + chunk_size;
                for s in &raw_samples[start..end] {
                    energy += s.abs();
                }
                let avg = (energy / (chunk_size as f32)) * 6.5; // Gain factor
                let freq_boost = 1.0 + (b as f32) * 0.12;
                *band = (avg * freq_boost).clamp(0.0, 1.0);
            }

            // Smooth falloff / decay
            for (b, &cur) in current_bands.iter().enumerate() {
                if cur >= decay_bands[b] {
                    decay_bands[b] = cur;
                } else {
                    decay_bands[b] = (decay_bands[b] * 0.82 - 0.02).max(0.0);
                }
            }

            {
                let mut b = bands_lock.lock().unwrap();
                *b = decay_bands;
            }
        }

        let _ = child.kill();
        let _ = child.wait();
    }
}

pub fn build_audio_scene(bands: &[f32; 16], brightness: f32) -> Scene {
    let mut scene = Scene {
        effects: HashMap::new(),
        keys: HashMap::new(),
    };

    // Slot 0: Static Effect base
    let dummy_fade = protocol::fade((255, 255, 255), &[((255, 255, 255), 1000)]).0;
    scene
        .effects
        .insert(0, protocol::Effect::new((255, 255, 255), dummy_fade, None));

    let keys = layout::get_keys();
    for k in keys {
        let (cx, cy) = layout::normalized_center(&k);
        let band_idx = ((cx * 16.0).floor() as usize).min(15);
        let level = bands[band_idx];

        // Inverse height: higher rows light up only when volume/energy is high
        let height_threshold = 1.0 - (cy * HEIGHT / HEIGHT);
        let is_lit = level >= height_threshold * 0.85;

        let color = if is_lit {
            // Dynamic chromatic gradient from Cyan/Green at base to Magenta/Red at peak
            let heat = level.clamp(0.0, 1.0);
            let r = ((heat * 255.0) as u8).saturating_add(30);
            let g = ((1.0 - heat * 0.6) * 220.0) as u8;
            let b = ((1.0 - heat) * 255.0) as u8;
            (
                ((r as f32) * brightness) as u8,
                ((g as f32) * brightness) as u8,
                ((b as f32) * brightness) as u8,
            )
        } else {
            (0, 5, 15) // Deep ambient glow
        };

        scene.keys.insert(
            k.code,
            Key {
                color,
                mode: MODE_EFFECT,
                effect: 0,
                ..Default::default()
            },
        );
    }

    scene
}

pub fn run_visualizer_cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Starting rrgb Live Audio Spectrum Visualizer...");
    println!("Connecting to PipeWire/PulseAudio monitor sink...");
    println!("Press Ctrl+C to stop.");

    let mut vis = AudioVisualizer::new();
    vis.start();

    let mut manager = MultiDeviceManager::load_all()?;

    loop {
        let bands = vis.bands();
        let scene = build_audio_scene(&bands, 1.0);
        let _ = manager.apply_scene(&scene);
        thread::sleep(Duration::from_millis(30));
    }
}
