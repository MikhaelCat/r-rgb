use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Instant;

use gtk4::glib;
use gtk4::prelude::*;
use libadwaita::prelude::*;
use libadwaita::{
    ActionRow, Application, ApplicationWindow, ExpanderRow, HeaderBar, PreferencesGroup, SwitchRow,
};

use crate::audio::{build_audio_scene, AudioVisualizer};
use crate::device::MultiDeviceManager;
use crate::effects::{build_scene, Simulator, EFFECTS};
use crate::layout::{get_keys, HEIGHT, WIDTH};
use crate::settings::{
    self, default_config_path, default_profiles_path, format_color, load_app_data, save_app_data,
    AppData, Settings, DIRECTIONS,
};

const SWATCHES: &[(&str, &str)] = &[
    ("Red", "#ff0000"),
    ("Orange", "#ff5000"),
    ("Yellow", "#ffc800"),
    ("Green", "#00ff00"),
    ("Teal", "#00ffa0"),
    ("Cyan", "#00c8ff"),
    ("Blue", "#0028ff"),
    ("Purple", "#8000ff"),
    ("Pink", "#ff0080"),
    ("White", "#ffffff"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Ru,
    En,
}

impl Language {
    pub fn parse_code(s: &str) -> Self {
        match s {
            "en" => Language::En,
            _ => Language::Ru,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::Ru => "ru",
            Language::En => "en",
        }
    }
}

pub struct I18n {
    pub app_title: &'static str,
    pub lang_group: &'static str,
    pub lang_row_title: &'static str,
    pub lang_row_subtitle: &'static str,
    pub preview_group_title: &'static str,
    pub preview_group_desc: &'static str,
    pub profile_group_title: &'static str,
    pub profile_group_desc: &'static str,
    pub profile_row_title: &'static str,
    pub profile_row_subtitle: &'static str,
    pub profile_save: &'static str,
    pub profile_del: &'static str,
    pub profile_dialog_title: &'static str,
    pub profile_dialog_body: &'static str,
    pub cancel: &'static str,
    pub save: &'static str,
    pub master_group_title: &'static str,
    pub power_title: &'static str,
    pub power_subtitle: &'static str,
    pub bright_title: &'static str,
    pub bright_subtitle: &'static str,
    pub thermal_group_title: &'static str,
    pub thermal_group_desc: &'static str,
    pub thermal_title: &'static str,
    pub thermal_polling: &'static str,
    pub thermal_prefix: &'static str,
    pub audio_group_title: &'static str,
    pub audio_group_desc: &'static str,
    pub audio_title: &'static str,
    pub audio_subtitle: &'static str,
    pub zones_group_title: &'static str,
    pub zones_group_desc: &'static str,
    pub zones_switch_title: &'static str,
    pub zones_switch_subtitle: &'static str,
    pub zones_bar_title: &'static str,
    pub zones_bar_subtitle: &'static str,
    pub bar_dialog_title: &'static str,
    pub motion_group_title: &'static str,
    pub speed_title: &'static str,
    pub speed_subtitle: &'static str,
    pub dir_title: &'static str,
    pub dir_subtitle: &'static str,
    pub color_group_title: &'static str,
    pub color_group_desc: &'static str,
    pub color_picker_title: &'static str,
    pub color_picker_subtitle: &'static str,
    pub color_dialog_title: &'static str,
    pub presets_group_title: &'static str,
    pub presets_group_desc: &'static str,
    pub fav_title: &'static str,
    pub fav_subtitle: &'static str,
    pub fav_count_fmt: &'static str,
    pub presets_count_fmt: &'static str,
}

pub fn get_i18n(lang: Language) -> I18n {
    match lang {
        Language::Ru => I18n {
            app_title: "rrgb — Управление подсветкой оборудования",
            lang_group: "Языковые настройки",
            lang_row_title: "Язык интерфейса",
            lang_row_subtitle: "Выберите язык графического интерфейса",
            preview_group_title: "Предпросмотр в реальном времени",
            preview_group_desc: "Интерактивная визуализация подсветки клавиатуры и светового бара",
            profile_group_title: "Профили и пресеты",
            profile_group_desc: "Сохранение, загрузка и удаление пользовательских конфигураций",
            profile_row_title: "Текущий профиль",
            profile_row_subtitle: "Выбор или запись профилей в ~/.config/rrgb/profiles.json",
            profile_save: "Сохранить",
            profile_del: "Удалить",
            profile_dialog_title: "Сохранить пользовательский профиль",
            profile_dialog_body: "Введите уникальное имя для текущей конфигурации подсветки:",
            cancel: "Отмена",
            save: "Сохранить",
            master_group_title: "Главное управление",
            power_title: "Подсветка включена",
            power_subtitle: "Включение и выключение светодиодов клавиатуры и бара",
            bright_title: "Яркость",
            bright_subtitle: "Общий уровень освещенности оборудования",
            thermal_group_title: "Термо-реактивная синхронизация",
            thermal_group_desc: "Плавное отображение температуры CPU/GPU цветом в реальном времени",
            thermal_title: "Термо-реактивный режим",
            thermal_polling: "Температура CPU: опрос датчиков...",
            thermal_prefix: "Температура CPU в реальном времени: ",
            audio_group_title: "Аудио-визуализатор спектра",
            audio_group_desc:
                "Синхронизация подсветки с воспроизводимым звуком (PipeWire / PulseAudio)",
            audio_title: "Реакция на звук и музыку",
            audio_subtitle: "Живой 16-полосный эквалайзер на клавишах и световом баре",
            zones_group_title: "Независимые аппаратные зоны",
            zones_group_desc:
                "Раздельный контроль клавиатуры и фронтального светового бара / логотипа",
            zones_switch_title: "Независимая зона бара",
            zones_switch_subtitle: "Настраивать цвет светового бара отдельно от клавиатуры",
            zones_bar_title: "Цвет светового бара и логотипа",
            zones_bar_subtitle: "Индивидуальная подсветка фронтального светового бара",
            bar_dialog_title: "Выбор цвета светового бара",
            motion_group_title: "Динамика анимации",
            speed_title: "Скорость анимации",
            speed_subtitle: "Частота аппаратных переходов эффекта",
            dir_title: "Направление волны",
            dir_subtitle: "Пространственный вектор распространения волны",
            color_group_title: "Цветовая настройка клавиатуры",
            color_group_desc: "Выберите готовый цвет или откройте цветовую палитру",
            color_picker_title: "Палитра любого цвета",
            color_picker_subtitle: "Нажмите для выбора любого оттенка из RGB спектра",
            color_dialog_title: "Выбор цвета клавиатуры",
            presets_group_title: "Аппаратные режимы подсветки (50)",
            presets_group_desc: "Профили, исполняемые встроенным микроконтроллером клавиатуры",
            fav_title: "Избранные эффекты (★)",
            fav_subtitle: "Быстрый доступ к вашим сохраненным эффектам",
            fav_count_fmt: "Закреплено избранных пресетов: {}",
            presets_count_fmt: "{} ({} пресетов)",
        },
        Language::En => I18n {
            app_title: "rrgb — Hardware Lighting Control",
            lang_group: "Language Settings",
            lang_row_title: "Interface Language",
            lang_row_subtitle: "Select preferred GUI display language",
            preview_group_title: "Real-Time Hardware Preview",
            preview_group_desc:
                "Interactive visualization of active illumination across keyboard and lightbar",
            profile_group_title: "User Profiles and Presets",
            profile_group_desc: "Save, load, and manage custom lighting configurations",
            profile_row_title: "Active Profile",
            profile_row_subtitle:
                "Select or store custom profiles into ~/.config/rrgb/profiles.json",
            profile_save: "Save",
            profile_del: "Delete",
            profile_dialog_title: "Save Custom Profile",
            profile_dialog_body: "Give a unique name for your current lighting configuration:",
            cancel: "Cancel",
            save: "Save",
            master_group_title: "Master Control",
            power_title: "Lighting Enabled",
            power_subtitle: "Toggle keyboard and lightbar LEDs",
            bright_title: "Brightness",
            bright_subtitle: "Overall illumination level",
            thermal_group_title: "Thermal Reactive Hardware Sync",
            thermal_group_desc:
                "Dynamically map hardware CPU/GPU temperatures to live color gradients",
            thermal_title: "Thermal Reactive Mode",
            thermal_polling: "CPU Temperature: polling sensors...",
            thermal_prefix: "Live CPU Temperature: ",
            audio_group_title: "Audio Spectrum Visualizer",
            audio_group_desc:
                "Synchronize illumination with live system audio (PipeWire / PulseAudio)",
            audio_title: "Audio Reactive Mode",
            audio_subtitle: "Live 16-band equalizer across keys and lightbar",
            zones_group_title: "Independent Hardware Zones",
            zones_group_desc:
                "Separate control for per-key keyboard and front lightbar / dragon logo",
            zones_switch_title: "Independent Lightbar Zone",
            zones_switch_subtitle: "Configure front lightbar / dragon logo separately from keys",
            zones_bar_title: "Lightbar and Logo Color",
            zones_bar_subtitle: "Custom dedicated illumination for the front lightbar and logo",
            bar_dialog_title: "Select Lightbar and Logo Color",
            motion_group_title: "Animation Dynamics",
            speed_title: "Animation Speed",
            speed_subtitle: "Frequency of hardware transitions",
            dir_title: "Wave Direction",
            dir_subtitle: "Spatial progression vector",
            color_group_title: "Keyboard Color Customization",
            color_group_desc:
                "Choose preset swatches or open the color picker for any custom RGB value",
            color_picker_title: "Custom Color Picker",
            color_picker_subtitle: "Click to pick any hex or RGB hue from the color spectrum",
            color_dialog_title: "Select Custom Color",
            presets_group_title: "Hardware Animation Profiles (50)",
            presets_group_desc: "Hardware-offloaded profiles executed directly on the onboard MCU",
            fav_title: "Favorites and Pinned (★)",
            fav_subtitle: "Your favorite quick-access effects",
            fav_count_fmt: "{} pinned favorite preset(s)",
            presets_count_fmt: "{} ({} presets)",
        },
    }
}

fn get_category_translation(
    cat_name: &'static str,
    lang: Language,
) -> (&'static str, &'static str) {
    match (cat_name, lang) {
        ("Core", Language::Ru) => ("Базовые", "Классика и основные режимы"),
        ("Sci-Fi", Language::Ru) => ("Научная фантастика", "Киберпанк, Sci-Fi и цифровые эффекты"),
        ("Nature", Language::Ru) => ("Природа", "Природные и атмосферные стихии"),
        ("Space", Language::Ru) => ("Космос", "Астрономия и глубокий космос"),
        ("Ambient", Language::Ru) => ("Атмосферные", "Спокойное настроение и эмбиент"),
        ("Tactical", Language::Ru) => ("Тактические", "Экшен, прицелы и динамика"),
        ("Harmonies", Language::Ru) => ("Гармонии", "Экзотические цветовые спектры"),

        ("Core", Language::En) => ("Core", "Classics and Fundamentals"),
        ("Sci-Fi", Language::En) => ("Sci-Fi", "Cyberpunk, Sci-Fi and Digital"),
        ("Nature", Language::En) => ("Nature", "Nature and Atmospheric Elements"),
        ("Space", Language::En) => ("Space", "Astronomy and Deep Cosmos"),
        ("Ambient", Language::En) => ("Ambient", "Mood, Chill and Ambient"),
        ("Tactical", Language::En) => ("Tactical", "Tactical, Action and Dynamic"),
        ("Harmonies", Language::En) => ("Harmonies", "Exotic Chromatic Harmonies"),
        _ => (cat_name, ""),
    }
}

struct HardwareWorker {
    condvar: Condvar,
    state: Mutex<Option<Settings>>,
}

static HW_WORKER: std::sync::OnceLock<HardwareWorker> = std::sync::OnceLock::new();

fn get_hw_worker() -> &'static HardwareWorker {
    HW_WORKER.get_or_init(|| {
        let worker = HardwareWorker {
            condvar: Condvar::new(),
            state: Mutex::new(None),
        };

        thread::Builder::new()
            .name("rrgb-hw-worker".to_string())
            .spawn(|| {
                let w = get_hw_worker();
                loop {
                    let mut lock = w.state.lock().unwrap();
                    while lock.is_none() {
                        lock = w.condvar.wait(lock).unwrap();
                    }
                    drop(lock);

                    // Debounce rapid user events (sliders, fast clicking)
                    thread::sleep(std::time::Duration::from_millis(30));

                    let mut lock = w.state.lock().unwrap();
                    let target = lock.take();
                    drop(lock);

                    if let Some(s) = target {
                        if let Ok(mut manager) = MultiDeviceManager::load_all() {
                            if s.independent_zones {
                                let mut bar_settings = s.clone();
                                bar_settings.effect = "static".to_string();
                                bar_settings.color =
                                    s.bar_color.clone().unwrap_or_else(|| s.color.clone());
                                let kb_scene = build_scene(&s);
                                let bar_scene = build_scene(&bar_settings);
                                let _ = manager.apply_scenes_zoned(&kb_scene, &bar_scene);
                            } else {
                                let scene = build_scene(&s);
                                let _ = manager.apply_scene(&scene);
                            }
                        }
                    }
                }
            })
            .expect("failed to spawn rrgb hardware worker thread");

        worker
    })
}

fn schedule_hardware_apply(settings: &Settings) {
    let worker = get_hw_worker();
    let mut lock = worker.state.lock().unwrap();
    *lock = Some(settings.clone());
    worker.condvar.notify_one();
}

struct State {
    settings: Settings,
    config_path: std::path::PathBuf,
    profiles_path: std::path::PathBuf,
    app_data: AppData,
    thermal_enabled: bool,
    current_temp: f32,
    audio_enabled: bool,
    audio_vis: Option<AudioVisualizer>,
    audio_bands: [f32; 16],
    language: Language,
    simulator: Simulator,
    bar_simulator: Simulator,
    start_time: Instant,
}

pub fn run_gui() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.mikhaelcat.rrgb")
        .build();

    app.connect_activate(build_ui);
    app.run_with_args(&["rrgb"])
}

fn read_cpu_temp() -> Option<f32> {
    if let Ok(data) = std::fs::read_to_string("/sys/class/thermal/thermal_zone7/temp") {
        if let Ok(milli) = data.trim().parse::<f32>() {
            return Some(milli / 1000.0);
        }
    }
    // Fallback scan
    if let Ok(entries) = std::fs::read_dir("/sys/class/thermal") {
        for entry in entries.flatten() {
            let path = entry.path().join("temp");
            if let Ok(data) = std::fs::read_to_string(&path) {
                if let Ok(milli) = data.trim().parse::<f32>() {
                    let c = milli / 1000.0;
                    if (20.0..=110.0).contains(&c) {
                        return Some(c);
                    }
                }
            }
        }
    }
    None
}

fn temp_to_color(temp_c: f32) -> String {
    let t = ((temp_c - 40.0) / 45.0).clamp(0.0, 1.0);
    let r = ((t * 2.0).min(1.0) * 255.0) as u8;
    let g = (((1.0 - t) * 2.0).min(1.0) * 255.0) as u8;
    let b = (if t < 0.5 {
        (0.5 - t) * 2.0 * 200.0
    } else {
        0.0
    }) as u8;
    format_color((r, g, b))
}

fn build_ui(app: &Application) {
    let config_path = default_config_path();
    let profiles_path = default_profiles_path();
    let initial_settings = settings::load(&config_path);
    let app_data = load_app_data(&profiles_path);
    let initial_lang = Language::parse_code(&app_data.language);

    let init_kb_scene = build_scene(&initial_settings);
    let mut init_bar_settings = initial_settings.clone();
    if initial_settings.independent_zones && initial_settings.bar_color.is_some() {
        init_bar_settings.effect = "static".to_string();
        init_bar_settings.color = initial_settings.bar_color.clone().unwrap();
    }
    let init_bar_scene = build_scene(&init_bar_settings);

    let state = Arc::new(Mutex::new(State {
        settings: initial_settings.clone(),
        config_path,
        profiles_path,
        app_data,
        thermal_enabled: false,
        current_temp: 45.0,
        audio_enabled: false,
        audio_vis: None,
        audio_bands: [0.0; 16],
        language: initial_lang,
        simulator: Simulator::new(init_kb_scene),
        bar_simulator: Simulator::new(init_bar_scene),
        start_time: Instant::now(),
    }));

    let cur_i18n = get_i18n(initial_lang);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(cur_i18n.app_title)
        .default_width(740)
        .default_height(900)
        .build();

    let header = HeaderBar::new();
    let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    content_box.set_margin_top(16);
    content_box.set_margin_bottom(24);
    content_box.set_margin_start(16);
    content_box.set_margin_end(16);

    let clamp = libadwaita::Clamp::builder()
        .maximum_size(760)
        .tightening_threshold(560)
        .child(&content_box)
        .build();

    let scroller = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vexpand(true)
        .child(&clamp)
        .build();

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    main_box.append(&header);
    main_box.append(&scroller);

    // ========================================================
    // LANGUAGE SWITCHER
    // ========================================================
    let lang_group = PreferencesGroup::builder()
        .title(cur_i18n.lang_group)
        .build();

    let lang_row = ActionRow::builder()
        .title(cur_i18n.lang_row_title)
        .subtitle(cur_i18n.lang_row_subtitle)
        .build();

    let lang_dropdown = gtk4::DropDown::from_strings(&["Русский (RU)", "English (EN)"]);
    lang_dropdown.set_selected(if initial_lang == Language::Ru { 0 } else { 1 });
    lang_row.add_suffix(&lang_dropdown);
    lang_group.add(&lang_row);
    content_box.append(&lang_group);

    // ========================================================
    // Interactive Real-Time Animated Preview Canvas
    // ========================================================
    let preview_group = PreferencesGroup::builder()
        .title(cur_i18n.preview_group_title)
        .description(cur_i18n.preview_group_desc)
        .build();

    let drawing_area = gtk4::DrawingArea::builder()
        .content_width(680)
        .content_height(210)
        .hexpand(true)
        .build();

    let state_draw = state.clone();
    let keys = get_keys();
    drawing_area.set_draw_func(move |_area, cr, width, height| {
        let st = state_draw.lock().unwrap();
        let elapsed = st.start_time.elapsed().as_secs_f32();
        let power_on = st.settings.enabled;
        let bright = st.settings.brightness.clamp(0.05, 1.0) as f64;

        // Background chassis
        cr.set_source_rgb(0.08, 0.08, 0.10);
        let _ = cr.paint();

        // Keyboard bounding box
        let pad_x = 18.0;
        let pad_y = 12.0;
        let kb_w = width as f64 - pad_x * 2.0;
        let kb_h = height as f64 - pad_y * 2.0 - 22.0;
        let scale_x = kb_w / WIDTH as f64;
        let scale_y = kb_h / HEIGHT as f64;

        // Base plate
        cr.set_source_rgb(0.12, 0.13, 0.15);
        cr.rectangle(pad_x - 4.0, pad_y - 4.0, kb_w + 8.0, kb_h + 8.0);
        let _ = cr.fill();

        for k in &keys {
            let kx = pad_x + (k.x as f64) * scale_x;
            let ky = pad_y + (k.y as f64) * scale_y;
            let kw = (k.w as f64) * scale_x - 2.0;
            let kh = (k.h as f64) * scale_y - 2.0;

            if power_on {
                let (r, g, b) = if st.audio_enabled {
                    let cx = (k.x + k.w * 0.5) / WIDTH;
                    let cy = (k.y + k.h * 0.5) / HEIGHT;
                    let band_idx = ((cx * 16.0).floor() as usize).min(15);
                    let level = st.audio_bands[band_idx];
                    let height_threshold = 1.0 - cy;
                    if level >= height_threshold * 0.85 {
                        let heat = level.clamp(0.0, 1.0);
                        let r = ((heat * 255.0) as u8).saturating_add(30);
                        let g = ((1.0 - heat * 0.6) * 220.0) as u8;
                        let b = ((1.0 - heat) * 255.0) as u8;
                        (r, g, b)
                    } else {
                        (0, 8, 20)
                    }
                } else {
                    st.simulator.color(k.code, elapsed)
                };
                let rf = (r as f64 / 255.0) * bright;
                let gf = (g as f64 / 255.0) * bright;
                let bf = (b as f64 / 255.0) * bright;
                cr.set_source_rgb(rf, gf, bf);
            } else {
                cr.set_source_rgb(0.15, 0.15, 0.17);
            }
            cr.rectangle(kx, ky, kw, kh);
            let _ = cr.fill();

            // Key cap highlight border
            cr.set_source_rgba(1.0, 1.0, 1.0, 0.12);
            cr.set_line_width(0.7);
            cr.rectangle(kx, ky, kw, kh);
            let _ = cr.stroke();
        }

        // Front Lightbar Zone (Multi-segment gradient / dynamic wave representation)
        let bar_y = pad_y + kb_h + 10.0;
        let bar_w = kb_w * 0.70;
        let bar_x = pad_x + (kb_w - bar_w) / 2.0;
        let bar_h = 7.0;

        let num_bar_segments = 24;
        let seg_w = bar_w / (num_bar_segments as f64);

        // Reference key scancodes mapping horizontally across keyboard width for mirrored waves
        let horiz_keys: &[u8] = &[
            0x29, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
            0x47, 0x48, 0x49, 0x4A, 0x4B, 0x53, 0x54, 0x55, 0x56, 0x57,
        ];

        for i in 0..num_bar_segments {
            let sx = bar_x + (i as f64) * seg_w;
            if power_on {
                let (br, bg, bb) = if st.audio_enabled {
                    // Mirrored stereo equalizer bounce from center to edges
                    let norm_pos = if i < 12 {
                        (11 - i) as f32 / 11.0
                    } else {
                        (i - 12) as f32 / 11.0
                    };
                    let band_idx = ((norm_pos * 15.0).round() as usize).min(15);
                    let level = st.audio_bands[band_idx];
                    let heat = level.clamp(0.0, 1.0);
                    let r = ((heat * 255.0) as u8).saturating_add(40);
                    let g = ((1.0 - heat * 0.5) * 200.0) as u8;
                    let b = ((1.0 - heat) * 255.0) as u8;
                    (r, g, b)
                } else if st.settings.independent_zones {
                    st.bar_simulator.color(0x04, elapsed)
                } else {
                    let sample_key = horiz_keys[i % horiz_keys.len()];
                    st.simulator.color(sample_key, elapsed)
                };
                let brf = (br as f64 / 255.0) * bright;
                let bgf = (bg as f64 / 255.0) * bright;
                let bbf = (bb as f64 / 255.0) * bright;
                cr.set_source_rgb(brf, bgf, bbf);
            } else {
                cr.set_source_rgb(0.18, 0.18, 0.20);
            }
            cr.rectangle(sx, bar_y, seg_w + 0.5, bar_h);
            let _ = cr.fill();
        }

        // Lightbar glow border
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.25);
        cr.set_line_width(0.8);
        cr.rectangle(bar_x, bar_y, bar_w, bar_h);
        let _ = cr.stroke();
    });

    preview_group.add(&drawing_area);
    content_box.append(&preview_group);

    // Continuous 30fps animation ticker for the canvas preview
    let draw_ticker = drawing_area.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(33), move || {
        draw_ticker.queue_draw();
        glib::ControlFlow::Continue
    });

    // Helper to refresh simulators
    let update_simulators = {
        let state = state.clone();
        Rc::new(move || {
            let mut st = state.lock().unwrap();
            let kb_scene = build_scene(&st.settings);
            let mut bar_settings = st.settings.clone();
            if st.settings.independent_zones {
                bar_settings.effect = "static".to_string();
                bar_settings.color = st
                    .settings
                    .bar_color
                    .clone()
                    .unwrap_or_else(|| st.settings.color.clone());
            }
            let bar_scene = build_scene(&bar_settings);
            st.simulator = Simulator::new(kb_scene);
            st.bar_simulator = Simulator::new(bar_scene);
        })
    };

    // ========================================================
    // Custom User Profiles (Save/Load/Delete)
    // ========================================================
    let profile_group = PreferencesGroup::builder()
        .title(cur_i18n.profile_group_title)
        .description(cur_i18n.profile_group_desc)
        .build();

    let profile_row = ActionRow::builder()
        .title(cur_i18n.profile_row_title)
        .subtitle(cur_i18n.profile_row_subtitle)
        .build();

    let profile_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    let profile_dropdown = gtk4::DropDown::from_strings(&["Default"]);
    let save_profile_btn = gtk4::Button::with_label(cur_i18n.profile_save);
    let delete_profile_btn = gtk4::Button::with_label(cur_i18n.profile_del);

    profile_box.append(&profile_dropdown);
    profile_box.append(&save_profile_btn);
    profile_box.append(&delete_profile_btn);
    profile_row.add_suffix(&profile_box);
    profile_group.add(&profile_row);
    content_box.append(&profile_group);

    // Populate profile dropdown list
    let refresh_profile_dropdown = {
        let profile_dropdown = profile_dropdown.clone();
        let state = state.clone();
        Rc::new(move || {
            let st = state.lock().unwrap();
            let mut names = vec!["Default".to_string()];
            let mut keys: Vec<String> = st.app_data.profiles.keys().cloned().collect();
            keys.sort();
            names.extend(keys);
            let refs: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
            let model = gtk4::StringList::new(&refs);
            profile_dropdown.set_model(Some(&model));
        })
    };
    (refresh_profile_dropdown)();

    // ========================================================
    // 1. Master Toggle & Brightness
    // ========================================================
    let general_group = PreferencesGroup::builder()
        .title(cur_i18n.master_group_title)
        .build();
    let power_row = SwitchRow::builder()
        .title(cur_i18n.power_title)
        .subtitle(cur_i18n.power_subtitle)
        .active(initial_settings.enabled)
        .build();

    let brightness_scale = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 5.0, 100.0, 1.0);
    brightness_scale.set_value(initial_settings.brightness as f64 * 100.0);
    brightness_scale.set_hexpand(true);
    brightness_scale.set_size_request(160, -1);

    let brightness_row = ActionRow::builder()
        .title(cur_i18n.bright_title)
        .subtitle(cur_i18n.bright_subtitle)
        .build();
    brightness_row.add_suffix(&brightness_scale);

    general_group.add(&power_row);
    general_group.add(&brightness_row);
    content_box.append(&general_group);

    // ========================================================
    // Thermal Reactive Sync
    // ========================================================
    let thermal_group = PreferencesGroup::builder()
        .title(cur_i18n.thermal_group_title)
        .description(cur_i18n.thermal_group_desc)
        .build();

    let thermal_switch = SwitchRow::builder()
        .title(cur_i18n.thermal_title)
        .subtitle(cur_i18n.thermal_polling)
        .active(false)
        .build();

    thermal_group.add(&thermal_switch);
    content_box.append(&thermal_group);

    // ========================================================
    // Audio Reactive Visualizer (PipeWire / PulseAudio)
    // ========================================================
    let audio_group = PreferencesGroup::builder()
        .title(cur_i18n.audio_group_title)
        .description(cur_i18n.audio_group_desc)
        .build();

    let audio_switch = SwitchRow::builder()
        .title(cur_i18n.audio_title)
        .subtitle(cur_i18n.audio_subtitle)
        .active(false)
        .build();

    audio_group.add(&audio_switch);
    content_box.append(&audio_group);

    // ========================================================
    // Independent Device Zones (Keyboard vs Lightbar)
    // ========================================================
    let zones_group = PreferencesGroup::builder()
        .title(cur_i18n.zones_group_title)
        .description(cur_i18n.zones_group_desc)
        .build();

    let zones_switch = SwitchRow::builder()
        .title(cur_i18n.zones_switch_title)
        .subtitle(cur_i18n.zones_switch_subtitle)
        .active(initial_settings.independent_zones)
        .build();

    let bar_rgb = settings::parse_color(
        initial_settings
            .bar_color
            .as_deref()
            .unwrap_or(&initial_settings.color),
    )
    .unwrap_or((0, 200, 255));
    let bar_rgba = gtk4::gdk::RGBA::new(
        bar_rgb.0 as f32 / 255.0,
        bar_rgb.1 as f32 / 255.0,
        bar_rgb.2 as f32 / 255.0,
        1.0,
    );

    let bar_dialog = gtk4::ColorDialog::builder()
        .title(cur_i18n.bar_dialog_title)
        .with_alpha(false)
        .build();

    let bar_color_btn = gtk4::ColorDialogButton::builder()
        .dialog(&bar_dialog)
        .rgba(&bar_rgba)
        .valign(gtk4::Align::Center)
        .sensitive(initial_settings.independent_zones)
        .build();

    let bar_row = ActionRow::builder()
        .title(cur_i18n.zones_bar_title)
        .subtitle(cur_i18n.zones_bar_subtitle)
        .activatable_widget(&bar_color_btn)
        .build();
    bar_row.add_suffix(&bar_color_btn);

    zones_group.add(&zones_switch);
    zones_group.add(&bar_row);
    content_box.append(&zones_group);

    // ========================================================
    // 2. Motion Group (Speed & Direction)
    // ========================================================
    let motion_group = PreferencesGroup::builder()
        .title(cur_i18n.motion_group_title)
        .build();
    let speed_scale = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 1.0);
    speed_scale.set_value(initial_settings.speed as f64 * 100.0);
    speed_scale.set_hexpand(true);
    speed_scale.set_size_request(160, -1);

    let speed_row = ActionRow::builder()
        .title(cur_i18n.speed_title)
        .subtitle(cur_i18n.speed_subtitle)
        .build();
    speed_row.add_suffix(&speed_scale);

    let dir_dropdown = gtk4::DropDown::from_strings(DIRECTIONS);
    if let Some(pos) = DIRECTIONS
        .iter()
        .position(|&d| d == initial_settings.direction)
    {
        dir_dropdown.set_selected(pos as u32);
    }
    let dir_row = ActionRow::builder()
        .title(cur_i18n.dir_title)
        .subtitle(cur_i18n.dir_subtitle)
        .build();
    dir_row.add_suffix(&dir_dropdown);

    motion_group.add(&speed_row);
    motion_group.add(&dir_row);
    content_box.append(&motion_group);

    // ========================================================
    // 3. Color Customization (Picker & Swatches)
    // ========================================================
    let color_group = PreferencesGroup::builder()
        .title(cur_i18n.color_group_title)
        .description(cur_i18n.color_group_desc)
        .build();

    let init_rgb = settings::parse_color(&initial_settings.color).unwrap_or((0, 255, 160));
    let init_rgba = gtk4::gdk::RGBA::new(
        init_rgb.0 as f32 / 255.0,
        init_rgb.1 as f32 / 255.0,
        init_rgb.2 as f32 / 255.0,
        1.0,
    );

    let color_dialog = gtk4::ColorDialog::builder()
        .title(cur_i18n.color_dialog_title)
        .with_alpha(false)
        .build();

    let color_btn = gtk4::ColorDialogButton::builder()
        .dialog(&color_dialog)
        .rgba(&init_rgba)
        .valign(gtk4::Align::Center)
        .build();

    let picker_row = ActionRow::builder()
        .title(cur_i18n.color_picker_title)
        .subtitle(cur_i18n.color_picker_subtitle)
        .activatable_widget(&color_btn)
        .build();
    picker_row.add_suffix(&color_btn);

    color_group.add(&picker_row);

    let flow = gtk4::FlowBox::builder()
        .selection_mode(gtk4::SelectionMode::None)
        .homogeneous(true)
        .min_children_per_line(5)
        .max_children_per_line(10)
        .row_spacing(10)
        .column_spacing(10)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let update_sim_color = update_simulators.clone();
    for &(label, hex) in SWATCHES {
        let btn = gtk4::Button::with_label(label);
        let st_clone = state.clone();
        let color_btn_ref = color_btn.clone();
        let hex_owned = hex.to_string();
        let update_sim = update_sim_color.clone();
        btn.connect_clicked(move |_| {
            let s_copy = {
                let mut st = st_clone.lock().unwrap();
                st.settings.color = hex_owned.clone();
                let _ = settings::save(&st.settings, &st.config_path);
                st.settings.clone()
            };
            schedule_hardware_apply(&s_copy);
            update_sim();

            if let Ok(rgb) = settings::parse_color(&hex_owned) {
                let rgba = gtk4::gdk::RGBA::new(
                    rgb.0 as f32 / 255.0,
                    rgb.1 as f32 / 255.0,
                    rgb.2 as f32 / 255.0,
                    1.0,
                );
                color_btn_ref.set_rgba(&rgba);
            }
        });
        flow.append(&btn);
    }
    color_group.add(&flow);
    content_box.append(&color_group);

    // ========================================================
    // Hardware Animation Presets & Favorites
    // ========================================================
    let presets_group = PreferencesGroup::builder()
        .title(cur_i18n.presets_group_title)
        .description(cur_i18n.presets_group_desc)
        .build();

    let fav_expander = ExpanderRow::builder()
        .title(cur_i18n.fav_title)
        .subtitle(cur_i18n.fav_subtitle)
        .expanded(true)
        .build();
    presets_group.add(&fav_expander);

    // Radio button group coordinator
    let first_check: Rc<RefCell<Option<gtk4::CheckButton>>> = Rc::new(RefCell::new(None));

    // Refresh Favorites List in fav_expander
    let fav_rows: Rc<RefCell<Vec<ActionRow>>> = Rc::new(RefCell::new(Vec::new()));
    let update_sim_fav = update_simulators.clone();
    let refresh_favorites_view = {
        let fav_expander = fav_expander.clone();
        let state = state.clone();
        let first_check = first_check.clone();
        let fav_rows = fav_rows.clone();
        let update_sim = update_sim_fav.clone();
        Rc::new(move || {
            // Remove existing rows
            for r in fav_rows.borrow().iter() {
                fav_expander.remove(r);
            }
            fav_rows.borrow_mut().clear();

            let st = state.lock().unwrap();
            let favs = st.app_data.favorites.clone();
            let active_eff = st.settings.effect.clone();
            let lang = st.language;
            drop(st);

            let i18n = get_i18n(lang);
            fav_expander.set_subtitle(&i18n.fav_count_fmt.replace("{}", &favs.len().to_string()));

            for fav_id in favs {
                if let Some(eff) = EFFECTS.iter().find(|e| e.id == fav_id) {
                    let check = gtk4::CheckButton::builder()
                        .valign(gtk4::Align::Center)
                        .build();

                    {
                        let mut fc = first_check.borrow_mut();
                        if let Some(ref f) = *fc {
                            check.set_group(Some(f));
                        } else {
                            *fc = Some(check.clone());
                        }
                    }

                    if eff.id == active_eff {
                        check.set_active(true);
                    }

                    let row = ActionRow::builder()
                        .title(format!("★ {}", eff.name))
                        .subtitle(eff.description)
                        .activatable_widget(&check)
                        .build();
                    row.add_prefix(&check);

                    let eff_id = eff.id.to_string();
                    let st_clone = state.clone();
                    let update_sim_inner = update_sim.clone();
                    check.connect_toggled(move |btn| {
                        if btn.is_active() {
                            let s_copy = {
                                let mut st = st_clone.lock().unwrap();
                                st.settings.effect = eff_id.clone();
                                let _ = settings::save(&st.settings, &st.config_path);
                                st.settings.clone()
                            };
                            schedule_hardware_apply(&s_copy);
                            update_sim_inner();
                        }
                    });

                    fav_expander.add_row(&row);
                    fav_rows.borrow_mut().push(row);
                }
            }
        })
    };
    (refresh_favorites_view)();

    struct CategoryWidget {
        raw_name: &'static str,
        expander: ExpanderRow,
    }
    let category_widgets: Rc<RefCell<Vec<CategoryWidget>>> = Rc::new(RefCell::new(Vec::new()));

    let categories = [
        "Core",
        "Sci-Fi",
        "Nature",
        "Space",
        "Ambient",
        "Tactical",
        "Harmonies",
    ];

    let update_sim_cat = update_simulators.clone();
    for cat_name in categories {
        let cat_effects: Vec<&crate::effects::EffectInfo> =
            EFFECTS.iter().filter(|e| e.category == cat_name).collect();

        if cat_effects.is_empty() {
            continue;
        }

        let contains_active = cat_effects.iter().any(|e| e.id == initial_settings.effect);
        let (loc_name, loc_desc) = get_category_translation(cat_name, initial_lang);

        let expander = ExpanderRow::builder()
            .title(format!(
                "{} ({} {})",
                loc_name,
                cat_effects.len(),
                if initial_lang == Language::Ru {
                    "пресетов"
                } else {
                    "presets"
                }
            ))
            .subtitle(loc_desc)
            .expanded(contains_active)
            .build();

        for eff in cat_effects {
            let check = gtk4::CheckButton::builder()
                .valign(gtk4::Align::Center)
                .build();

            {
                let mut fc = first_check.borrow_mut();
                if let Some(ref f) = *fc {
                    check.set_group(Some(f));
                } else {
                    *fc = Some(check.clone());
                }
            }

            if eff.id == initial_settings.effect {
                check.set_active(true);
            }

            let is_fav = state
                .lock()
                .unwrap()
                .app_data
                .favorites
                .contains(&eff.id.to_string());
            let fav_btn = gtk4::Button::builder()
                .label(if is_fav { "★" } else { "☆" })
                .valign(gtk4::Align::Center)
                .has_frame(false)
                .build();

            let row = ActionRow::builder()
                .title(eff.name)
                .subtitle(eff.description)
                .activatable_widget(&check)
                .build();
            row.add_prefix(&check);
            row.add_suffix(&fav_btn);

            let eff_id = eff.id.to_string();
            let st_clone = state.clone();
            let update_sim_inner = update_sim_cat.clone();
            check.connect_toggled(move |btn| {
                if btn.is_active() {
                    let s_copy = {
                        let mut st = st_clone.lock().unwrap();
                        st.settings.effect = eff_id.clone();
                        let _ = settings::save(&st.settings, &st.config_path);
                        st.settings.clone()
                    };
                    schedule_hardware_apply(&s_copy);
                    update_sim_inner();
                }
            });

            // Star / Pin Toggle Logic
            let st_fav = state.clone();
            let eff_id_fav = eff.id.to_string();
            let fav_btn_clone = fav_btn.clone();
            let refresh_favs_action = refresh_favorites_view.clone();
            fav_btn.connect_clicked(move |_| {
                let mut st = st_fav.lock().unwrap();
                let favs = &mut st.app_data.favorites;
                if let Some(pos) = favs.iter().position(|x| x == &eff_id_fav) {
                    favs.remove(pos);
                    fav_btn_clone.set_label("☆");
                } else {
                    favs.push(eff_id_fav.clone());
                    fav_btn_clone.set_label("★");
                }
                let _ = save_app_data(&st.app_data, &st.profiles_path);
                drop(st);
                refresh_favs_action();
            });

            expander.add_row(&row);
        }

        category_widgets.borrow_mut().push(CategoryWidget {
            raw_name: cat_name,
            expander: expander.clone(),
        });
        presets_group.add(&expander);
    }

    content_box.append(&presets_group);

    // ========================================================
    // Connect Language Switcher Change
    // ========================================================
    let st_lang = state.clone();
    let window_lang = window.clone();
    let lang_group_ref = lang_group.clone();
    let lang_row_ref = lang_row.clone();
    let preview_group_ref = preview_group.clone();
    let profile_group_ref = profile_group.clone();
    let profile_row_ref = profile_row.clone();
    let save_btn_ref = save_profile_btn.clone();
    let del_btn_ref = delete_profile_btn.clone();
    let general_group_ref = general_group.clone();
    let power_row_ref = power_row.clone();
    let bright_row_ref = brightness_row.clone();
    let thermal_group_ref = thermal_group.clone();
    let thermal_switch_ref = thermal_switch.clone();
    let audio_group_ref = audio_group.clone();
    let audio_switch_ref = audio_switch.clone();
    let zones_group_ref = zones_group.clone();
    let zones_switch_ref = zones_switch.clone();
    let bar_row_ref = bar_row.clone();
    let motion_group_ref = motion_group.clone();
    let speed_row_ref = speed_row.clone();
    let dir_row_ref = dir_row.clone();
    let color_group_ref = color_group.clone();
    let picker_row_ref = picker_row.clone();
    let presets_group_ref = presets_group.clone();
    let fav_expander_ref = fav_expander.clone();
    let refresh_favs_lang = refresh_favorites_view.clone();
    let category_widgets_ref = category_widgets.clone();

    lang_dropdown.connect_selected_notify(move |dd| {
        let new_lang = if dd.selected() == 0 {
            Language::Ru
        } else {
            Language::En
        };

        {
            let mut st = st_lang.lock().unwrap();
            st.language = new_lang;
            st.app_data.language = new_lang.code().to_string();
            let _ = save_app_data(&st.app_data, &st.profiles_path);
        }

        let i18n = get_i18n(new_lang);
        window_lang.set_title(Some(i18n.app_title));
        lang_group_ref.set_title(i18n.lang_group);
        lang_row_ref.set_title(i18n.lang_row_title);
        lang_row_ref.set_subtitle(i18n.lang_row_subtitle);

        preview_group_ref.set_title(i18n.preview_group_title);
        preview_group_ref.set_description(Some(i18n.preview_group_desc));

        profile_group_ref.set_title(i18n.profile_group_title);
        profile_group_ref.set_description(Some(i18n.profile_group_desc));
        profile_row_ref.set_title(i18n.profile_row_title);
        profile_row_ref.set_subtitle(i18n.profile_row_subtitle);
        save_btn_ref.set_label(i18n.profile_save);
        del_btn_ref.set_label(i18n.profile_del);

        general_group_ref.set_title(i18n.master_group_title);
        power_row_ref.set_title(i18n.power_title);
        power_row_ref.set_subtitle(i18n.power_subtitle);
        bright_row_ref.set_title(i18n.bright_title);
        bright_row_ref.set_subtitle(i18n.bright_subtitle);

        thermal_group_ref.set_title(i18n.thermal_group_title);
        thermal_group_ref.set_description(Some(i18n.thermal_group_desc));
        thermal_switch_ref.set_title(i18n.thermal_title);

        audio_group_ref.set_title(i18n.audio_group_title);
        audio_group_ref.set_description(Some(i18n.audio_group_desc));
        audio_switch_ref.set_title(i18n.audio_title);
        audio_switch_ref.set_subtitle(i18n.audio_subtitle);

        zones_group_ref.set_title(i18n.zones_group_title);
        zones_group_ref.set_description(Some(i18n.zones_group_desc));
        zones_switch_ref.set_title(i18n.zones_switch_title);
        zones_switch_ref.set_subtitle(i18n.zones_switch_subtitle);
        bar_row_ref.set_title(i18n.zones_bar_title);
        bar_row_ref.set_subtitle(i18n.zones_bar_subtitle);

        motion_group_ref.set_title(i18n.motion_group_title);
        speed_row_ref.set_title(i18n.speed_title);
        speed_row_ref.set_subtitle(i18n.speed_subtitle);
        dir_row_ref.set_title(i18n.dir_title);
        dir_row_ref.set_subtitle(i18n.dir_subtitle);

        color_group_ref.set_title(i18n.color_group_title);
        color_group_ref.set_description(Some(i18n.color_group_desc));
        picker_row_ref.set_title(i18n.color_picker_title);
        picker_row_ref.set_subtitle(i18n.color_picker_subtitle);

        presets_group_ref.set_title(i18n.presets_group_title);
        presets_group_ref.set_description(Some(i18n.presets_group_desc));
        fav_expander_ref.set_title(i18n.fav_title);

        for cw in category_widgets_ref.borrow().iter() {
            let cat_effects_count = EFFECTS.iter().filter(|e| e.category == cw.raw_name).count();
            let (loc_name, loc_desc) = get_category_translation(cw.raw_name, new_lang);
            let title_str = format!(
                "{} ({} {})",
                loc_name,
                cat_effects_count,
                if new_lang == Language::Ru {
                    "пресетов"
                } else {
                    "presets"
                }
            );
            cw.expander.set_title(&title_str);
            cw.expander.set_subtitle(loc_desc);
        }

        refresh_favs_lang();
    });

    // ========================================================
    // Connect remaining Event Handlers with Debounced Apply
    // ========================================================
    let st_picker = state.clone();
    let update_sim_picker = update_simulators.clone();
    color_btn.connect_notify_local(Some("rgba"), move |btn, _| {
        let rgba = btn.rgba();
        let r = (rgba.red() * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (rgba.green() * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (rgba.blue() * 255.0).round().clamp(0.0, 255.0) as u8;
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);

        let s_copy = {
            let mut st = st_picker.lock().unwrap();
            st.settings.color = hex;
            let _ = settings::save(&st.settings, &st.config_path);
            st.settings.clone()
        };
        schedule_hardware_apply(&s_copy);
        update_sim_picker();
    });

    let st_bar_color = state.clone();
    let update_sim_bar = update_simulators.clone();
    bar_color_btn.connect_notify_local(Some("rgba"), move |btn, _| {
        let rgba = btn.rgba();
        let r = (rgba.red() * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (rgba.green() * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (rgba.blue() * 255.0).round().clamp(0.0, 255.0) as u8;
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);

        let s_copy = {
            let mut st = st_bar_color.lock().unwrap();
            st.settings.bar_color = Some(hex);
            let _ = settings::save(&st.settings, &st.config_path);
            st.settings.clone()
        };
        schedule_hardware_apply(&s_copy);
        update_sim_bar();
    });

    let st_zones = state.clone();
    let bar_color_btn_ref = bar_color_btn.clone();
    let update_sim_zones = update_simulators.clone();
    zones_switch.connect_notify_local(Some("active"), move |row, _| {
        let active = row.is_active();
        bar_color_btn_ref.set_sensitive(active);
        let s_copy = {
            let mut st = st_zones.lock().unwrap();
            st.settings.independent_zones = active;
            let _ = settings::save(&st.settings, &st.config_path);
            st.settings.clone()
        };
        schedule_hardware_apply(&s_copy);
        update_sim_zones();
    });

    let st_power = state.clone();
    power_row.connect_notify_local(Some("active"), move |row, _| {
        let s_copy = {
            let mut st = st_power.lock().unwrap();
            st.settings.enabled = row.is_active();
            let _ = settings::save(&st.settings, &st.config_path);
            st.settings.clone()
        };
        schedule_hardware_apply(&s_copy);
    });

    let st_bright = state.clone();
    let update_sim_bright = update_simulators.clone();
    brightness_scale.connect_value_changed(move |sc| {
        let s_copy = {
            let mut st = st_bright.lock().unwrap();
            st.settings.brightness = (sc.value() / 100.0) as f32;
            let _ = settings::save(&st.settings, &st.config_path);
            st.settings.clone()
        };
        schedule_hardware_apply(&s_copy);
        update_sim_bright();
    });

    let st_speed = state.clone();
    let update_sim_speed = update_simulators.clone();
    speed_scale.connect_value_changed(move |sc| {
        let s_copy = {
            let mut st = st_speed.lock().unwrap();
            st.settings.speed = (sc.value() / 100.0) as f32;
            let _ = settings::save(&st.settings, &st.config_path);
            st.settings.clone()
        };
        schedule_hardware_apply(&s_copy);
        update_sim_speed();
    });

    let st_dir = state.clone();
    let update_sim_dir = update_simulators.clone();
    dir_dropdown.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        if let Some(&dir) = DIRECTIONS.get(idx) {
            let s_copy = {
                let mut st = st_dir.lock().unwrap();
                st.settings.direction = dir.to_string();
                let _ = settings::save(&st.settings, &st.config_path);
                st.settings.clone()
            };
            schedule_hardware_apply(&s_copy);
            update_sim_dir();
        }
    });

    // Profile Dropdown Selection
    let st_prof_sel = state.clone();
    let color_btn_prof = color_btn.clone();
    let bright_prof = brightness_scale.clone();
    let speed_prof = speed_scale.clone();
    let power_prof = power_row.clone();
    let update_sim_prof = update_simulators.clone();
    profile_dropdown.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        let st = st_prof_sel.lock().unwrap();
        let mut names = vec!["Default".to_string()];
        let mut keys: Vec<String> = st.app_data.profiles.keys().cloned().collect();
        keys.sort();
        names.extend(keys);
        if let Some(name) = names.get(idx) {
            let target_settings = if name == "Default" {
                Settings::default()
            } else if let Some(saved) = st.app_data.profiles.get(name) {
                saved.clone()
            } else {
                return;
            };
            drop(st);

            let s_copy = {
                let mut st_mut = st_prof_sel.lock().unwrap();
                st_mut.settings = target_settings.clone();
                let _ = settings::save(&st_mut.settings, &st_mut.config_path);
                st_mut.settings.clone()
            };
            schedule_hardware_apply(&s_copy);
            update_sim_prof();

            // Sync controls
            power_prof.set_active(target_settings.enabled);
            bright_prof.set_value(target_settings.brightness as f64 * 100.0);
            speed_prof.set_value(target_settings.speed as f64 * 100.0);
            if let Ok(rgb) = settings::parse_color(&target_settings.color) {
                let rgba = gtk4::gdk::RGBA::new(
                    rgb.0 as f32 / 255.0,
                    rgb.1 as f32 / 255.0,
                    rgb.2 as f32 / 255.0,
                    1.0,
                );
                color_btn_prof.set_rgba(&rgba);
            }
        }
    });

    // Save Profile Dialog
    let window_ref = window.clone();
    let st_save_prof = state.clone();
    let refresh_dropdown_save = refresh_profile_dropdown.clone();
    save_profile_btn.connect_clicked(move |_| {
        let st_now = st_save_prof.lock().unwrap();
        let i18n = get_i18n(st_now.language);
        drop(st_now);

        let entry = gtk4::Entry::builder()
            .placeholder_text("Profile name...")
            .margin_top(8)
            .margin_bottom(8)
            .build();

        let dialog = libadwaita::MessageDialog::builder()
            .transient_for(&window_ref)
            .heading(i18n.profile_dialog_title)
            .body(i18n.profile_dialog_body)
            .build();
        dialog.set_extra_child(Some(&entry));
        dialog.add_response("cancel", i18n.cancel);
        dialog.add_response("save", i18n.save);
        dialog.set_response_appearance("save", libadwaita::ResponseAppearance::Suggested);

        let st_dialog = st_save_prof.clone();
        let refresh_cb = refresh_dropdown_save.clone();
        dialog.connect_response(None, move |d, resp| {
            if resp == "save" {
                let text = entry.text().trim().to_string();
                if !text.is_empty() {
                    let mut st = st_dialog.lock().unwrap();
                    let current = st.settings.clone();
                    st.app_data.profiles.insert(text, current);
                    let _ = save_app_data(&st.app_data, &st.profiles_path);
                    drop(st);
                    refresh_cb();
                }
            }
            d.close();
        });
        dialog.present();
    });

    // Delete Profile Button
    let st_del_prof = state.clone();
    let dropdown_del = profile_dropdown.clone();
    let refresh_dropdown_del = refresh_profile_dropdown.clone();
    delete_profile_btn.connect_clicked(move |_| {
        let idx = dropdown_del.selected() as usize;
        let mut st = st_del_prof.lock().unwrap();
        let mut names = vec!["Default".to_string()];
        let mut keys: Vec<String> = st.app_data.profiles.keys().cloned().collect();
        keys.sort();
        names.extend(keys);
        if let Some(name) = names.get(idx) {
            if name != "Default" {
                st.app_data.profiles.remove(name);
                let _ = save_app_data(&st.app_data, &st.profiles_path);
                drop(st);
                refresh_dropdown_del();
            }
        }
    });

    // Thermal Reactive Loop
    let st_thermal = state.clone();
    thermal_switch.connect_notify_local(Some("active"), move |row, _| {
        let active = row.is_active();
        let mut st = st_thermal.lock().unwrap();
        st.thermal_enabled = active;
    });

    let st_timer = state.clone();
    let thermal_label_row = thermal_switch.clone();
    let update_sim_thermal = update_simulators.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(1500), move || {
        if let Some(temp) = read_cpu_temp() {
            let mut st = st_timer.lock().unwrap();
            st.current_temp = temp;
            let i18n = get_i18n(st.language);
            thermal_label_row.set_subtitle(&format!("{}{:.1}°C", i18n.thermal_prefix, temp));
            if st.thermal_enabled {
                let heat_color = temp_to_color(temp);
                st.settings.color = heat_color;
                let s_copy = st.settings.clone();
                drop(st);
                schedule_hardware_apply(&s_copy);
                update_sim_thermal();
            }
        }
        glib::ControlFlow::Continue
    });

    // Audio Reactive Toggle & Loop
    let st_audio = state.clone();
    audio_switch.connect_notify_local(Some("active"), move |row, _| {
        let active = row.is_active();
        let mut st = st_audio.lock().unwrap();
        st.audio_enabled = active;
        if active {
            if st.audio_vis.is_none() {
                let mut vis = AudioVisualizer::new();
                vis.start();
                st.audio_vis = Some(vis);
            }
        } else {
            if let Some(mut vis) = st.audio_vis.take() {
                vis.stop();
            }
            st.audio_bands = [0.0; 16];
            // Restore static/effect profile to hardware
            let s_copy = st.settings.clone();
            drop(st);
            schedule_hardware_apply(&s_copy);
        }
    });

    // Audio animation ticker (30fps) for sampling spectrum and applying to hardware
    let st_audio_ticker = state.clone();
    let mut audio_hw_counter = 0u8;
    glib::timeout_add_local(std::time::Duration::from_millis(33), move || {
        let mut st = st_audio_ticker.lock().unwrap();
        if st.audio_enabled {
            if let Some(ref vis) = st.audio_vis {
                let bands = vis.bands();
                st.audio_bands = bands;
                audio_hw_counter = (audio_hw_counter + 1) % 2; // ~15 FPS hardware update rate
                if audio_hw_counter == 0 {
                    let bright = st.settings.brightness;
                    drop(st);
                    // Update physical device directly without blocking GUI
                    thread::spawn(move || {
                        if let Ok(mut manager) = MultiDeviceManager::load_all() {
                            let scene = build_audio_scene(&bands, bright);
                            let _ = manager.apply_scene(&scene);
                        }
                    });
                }
            }
        }
        glib::ControlFlow::Continue
    });

    window.set_content(Some(&main_box));
    window.present();
}
