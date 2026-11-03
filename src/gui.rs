use std::sync::{Arc, Mutex};
use std::thread;

use gtk4::glib;
use gtk4::prelude::*;
use libadwaita::prelude::*;
use libadwaita::{
    ActionRow, Application, ApplicationWindow, ExpanderRow, HeaderBar, PreferencesGroup, SwitchRow,
};

use crate::device::MultiDeviceManager;
use crate::effects::{build_scene, EFFECTS};
use crate::settings::{self, default_config_path, Settings, DIRECTIONS};

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

struct State {
    settings: Settings,
    config_path: std::path::PathBuf,
}

pub fn run_gui() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.mikhaelcat.rrgb")
        .build();

    app.connect_activate(build_ui);
    app.run_with_args(&["rrgb"])
}

fn apply_hardware(settings: &Settings) {
    if let Ok(mut manager) = MultiDeviceManager::load_all() {
        let scene = build_scene(settings);
        let _ = manager.apply_scene(&scene);
    }
}

fn build_ui(app: &Application) {
    let config_path = default_config_path();
    let initial_settings = settings::load(&config_path);

    let state = Arc::new(Mutex::new(State {
        settings: initial_settings.clone(),
        config_path,
    }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("rrgb — Hardware Lighting Control")
        .default_width(720)
        .default_height(860)
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

    // 1. Master Toggle & Brightness
    let general_group = PreferencesGroup::builder().title("Master Control").build();
    let power_row = SwitchRow::builder()
        .title("Lighting Enabled")
        .subtitle("Toggle keyboard and lightbar LEDs")
        .active(initial_settings.enabled)
        .build();

    let brightness_scale = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 5.0, 100.0, 1.0);
    brightness_scale.set_value(initial_settings.brightness as f64 * 100.0);
    brightness_scale.set_hexpand(true);
    brightness_scale.set_size_request(160, -1);

    let brightness_row = ActionRow::builder()
        .title("Brightness")
        .subtitle("Overall illumination level")
        .build();
    brightness_row.add_suffix(&brightness_scale);

    general_group.add(&power_row);
    general_group.add(&brightness_row);
    content_box.append(&general_group);

    // 2. Motion Group (Speed & Direction)
    let motion_group = PreferencesGroup::builder()
        .title("Animation Dynamics")
        .build();
    let speed_scale = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 1.0);
    speed_scale.set_value(initial_settings.speed as f64 * 100.0);
    speed_scale.set_hexpand(true);
    speed_scale.set_size_request(160, -1);

    let speed_row = ActionRow::builder()
        .title("Animation Speed")
        .subtitle("Frequency of hardware transitions")
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
        .title("Wave Direction")
        .subtitle("Spatial progression vector")
        .build();
    dir_row.add_suffix(&dir_dropdown);

    motion_group.add(&speed_row);
    motion_group.add(&dir_row);
    content_box.append(&motion_group);

    // 3. Color Swatches Palette & Custom Color Chooser
    let color_group = PreferencesGroup::builder()
        .title("Color Customization")
        .description("Choose preset swatches or open the color picker for any custom RGB value")
        .build();

    let init_rgb = settings::parse_color(&initial_settings.color).unwrap_or((0, 255, 160));
    let init_rgba = gtk4::gdk::RGBA::new(
        init_rgb.0 as f32 / 255.0,
        init_rgb.1 as f32 / 255.0,
        init_rgb.2 as f32 / 255.0,
        1.0,
    );

    let color_dialog = gtk4::ColorDialog::builder()
        .title("Select Custom Color")
        .with_alpha(false)
        .build();

    let color_btn = gtk4::ColorDialogButton::builder()
        .dialog(&color_dialog)
        .rgba(&init_rgba)
        .valign(gtk4::Align::Center)
        .build();

    let picker_row = ActionRow::builder()
        .title("Custom Color Picker")
        .subtitle("Click to pick any hex or RGB hue from the color spectrum")
        .activatable_widget(&color_btn)
        .build();
    picker_row.add_suffix(&color_btn);

    let st_picker = state.clone();
    color_btn.connect_notify_local(Some("rgba"), move |btn, _| {
        let rgba = btn.rgba();
        let r = (rgba.red() * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (rgba.green() * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (rgba.blue() * 255.0).round().clamp(0.0, 255.0) as u8;
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);

        let mut st = st_picker.lock().unwrap();
        st.settings.color = hex;
        let _ = settings::save(&st.settings, &st.config_path);
        let s_copy = st.settings.clone();
        thread::spawn(move || apply_hardware(&s_copy));
    });

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

    for &(label, hex) in SWATCHES {
        let btn = gtk4::Button::with_label(label);
        let st_clone = state.clone();
        let color_btn_ref = color_btn.clone();
        let hex_owned = hex.to_string();
        btn.connect_clicked(move |_| {
            let mut st = st_clone.lock().unwrap();
            st.settings.color = hex_owned.clone();
            let _ = settings::save(&st.settings, &st.config_path);
            let s_copy = st.settings.clone();
            thread::spawn(move || apply_hardware(&s_copy));

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

    // 4. Categorized Hardware Animation Presets (All 50 Presets in Collapsible Groups)
    let categories = [
        ("Core", "Classics and Fundamentals"),
        ("Sci-Fi", "Cyberpunk, Sci-Fi and Digital"),
        ("Nature", "Nature and Atmospheric Elements"),
        ("Space", "Astronomy and Deep Cosmos"),
        ("Ambient", "Mood, Chill and Ambient"),
        ("Tactical", "Tactical, Action and Dynamic"),
        ("Harmonies", "Exotic Chromatic Harmonies"),
    ];

    let presets_group = PreferencesGroup::builder()
        .title("Hardware Animation Profiles (50)")
        .description("Hardware-offloaded profiles executed directly on the onboard MCU")
        .build();

    let mut first_check: Option<gtk4::CheckButton> = None;

    for (cat_name, cat_desc) in categories {
        let cat_effects: Vec<&crate::effects::EffectInfo> =
            EFFECTS.iter().filter(|e| e.category == cat_name).collect();

        if cat_effects.is_empty() {
            continue;
        }

        let contains_active = cat_effects.iter().any(|e| e.id == initial_settings.effect);

        let expander = ExpanderRow::builder()
            .title(format!("{} ({} presets)", cat_name, cat_effects.len()))
            .subtitle(cat_desc)
            .expanded(contains_active)
            .build();

        for eff in cat_effects {
            let check = gtk4::CheckButton::builder()
                .valign(gtk4::Align::Center)
                .build();
            if let Some(ref f) = first_check {
                check.set_group(Some(f));
            } else {
                first_check = Some(check.clone());
            }

            if eff.id == initial_settings.effect {
                check.set_active(true);
            }

            let row = ActionRow::builder()
                .title(eff.name)
                .subtitle(eff.description)
                .activatable_widget(&check)
                .build();
            row.add_prefix(&check);

            let eff_id = eff.id.to_string();
            let st_clone = state.clone();
            check.connect_toggled(move |btn| {
                if btn.is_active() {
                    let mut st = st_clone.lock().unwrap();
                    st.settings.effect = eff_id.clone();
                    let _ = settings::save(&st.settings, &st.config_path);
                    let s_copy = st.settings.clone();
                    thread::spawn(move || apply_hardware(&s_copy));
                }
            });

            expander.add_row(&row);
        }

        presets_group.add(&expander);
    }

    content_box.append(&presets_group);

    // Event Bindings
    let st_power = state.clone();
    power_row.connect_notify_local(Some("active"), move |row, _| {
        let mut st = st_power.lock().unwrap();
        st.settings.enabled = row.is_active();
        let _ = settings::save(&st.settings, &st.config_path);
        let s_copy = st.settings.clone();
        thread::spawn(move || apply_hardware(&s_copy));
    });

    let st_bright = state.clone();
    brightness_scale.connect_value_changed(move |sc| {
        let mut st = st_bright.lock().unwrap();
        st.settings.brightness = (sc.value() / 100.0) as f32;
        let _ = settings::save(&st.settings, &st.config_path);
        let s_copy = st.settings.clone();
        thread::spawn(move || apply_hardware(&s_copy));
    });

    let st_speed = state.clone();
    speed_scale.connect_value_changed(move |sc| {
        let mut st = st_speed.lock().unwrap();
        st.settings.speed = (sc.value() / 100.0) as f32;
        let _ = settings::save(&st.settings, &st.config_path);
        let s_copy = st.settings.clone();
        thread::spawn(move || apply_hardware(&s_copy));
    });

    let st_dir = state.clone();
    dir_dropdown.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        if let Some(&dir) = DIRECTIONS.get(idx) {
            let mut st = st_dir.lock().unwrap();
            st.settings.direction = dir.to_string();
            let _ = settings::save(&st.settings, &st.config_path);
            let s_copy = st.settings.clone();
            thread::spawn(move || apply_hardware(&s_copy));
        }
    });

    window.set_content(Some(&main_box));
    window.present();
}
