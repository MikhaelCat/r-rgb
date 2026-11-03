use std::cell::RefCell;
use std::rc::Rc;
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
use crate::layout::{get_keys, HEIGHT, WIDTH};
use crate::settings::{
    self, default_config_path, default_profiles_path, format_color, load_app_data, parse_color,
    save_app_data, AppData, Settings, DIRECTIONS,
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

struct State {
    settings: Settings,
    config_path: std::path::PathBuf,
    profiles_path: std::path::PathBuf,
    app_data: AppData,
    thermal_enabled: bool,
    current_temp: f32,
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
        if settings.independent_zones && settings.bar_color.is_some() {
            let mut bar_settings = settings.clone();
            bar_settings.effect = "static".to_string();
            bar_settings.color = settings
                .bar_color
                .clone()
                .unwrap_or_else(|| settings.color.clone());
            let kb_scene = build_scene(settings);
            let bar_scene = build_scene(&bar_settings);
            let _ = manager.apply_scenes_zoned(&kb_scene, &bar_scene);
        } else {
            let scene = build_scene(settings);
            let _ = manager.apply_scene(&scene);
        }
    }
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

    let state = Arc::new(Mutex::new(State {
        settings: initial_settings.clone(),
        config_path,
        profiles_path,
        app_data,
        thermal_enabled: false,
        current_temp: 45.0,
    }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("rrgb — Hardware Lighting Control")
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
    // FEATURE 2: Interactive Live Preview Canvas (DrawingArea)
    // ========================================================
    let preview_group = PreferencesGroup::builder()
        .title("Real-Time Hardware Preview")
        .description(
            "Interactive visualization of active illumination across keyboard and lightbar",
        )
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
        let (r, g, b) = parse_color(&st.settings.color).unwrap_or((0, 255, 160));
        let bright = st.settings.brightness.clamp(0.05, 1.0);
        let rf = (r as f64 / 255.0) * bright as f64;
        let gf = (g as f64 / 255.0) * bright as f64;
        let bf = (b as f64 / 255.0) * bright as f64;
        let power_on = st.settings.enabled;

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
                cr.set_source_rgb(rf * 0.9, gf * 0.9, bf * 0.9);
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

        // Front Lightbar Zone
        let bar_y = pad_y + kb_h + 10.0;
        let bar_w = kb_w * 0.70;
        let bar_x = pad_x + (kb_w - bar_w) / 2.0;
        let bar_h = 7.0;

        let (br, bg, bb) = if st.settings.independent_zones {
            if let Some(ref bc) = st.settings.bar_color {
                parse_color(bc).unwrap_or((r, g, b))
            } else {
                (r, g, b)
            }
        } else {
            (r, g, b)
        };

        if power_on {
            let brf = (br as f64 / 255.0) * bright as f64;
            let bgf = (bg as f64 / 255.0) * bright as f64;
            let bbf = (bb as f64 / 255.0) * bright as f64;
            cr.set_source_rgb(brf, bgf, bbf);
        } else {
            cr.set_source_rgb(0.18, 0.18, 0.20);
        }
        cr.rectangle(bar_x, bar_y, bar_w, bar_h);
        let _ = cr.fill();

        // Lightbar glow border
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.25);
        cr.set_line_width(0.8);
        cr.rectangle(bar_x, bar_y, bar_w, bar_h);
        let _ = cr.stroke();
    });

    preview_group.add(&drawing_area);
    content_box.append(&preview_group);

    // ========================================================
    // FEATURE 3: Custom User Profiles (Save/Load/Delete)
    // ========================================================
    let profile_group = PreferencesGroup::builder()
        .title("User Profiles & Presets")
        .description("Save, load, and manage custom lighting configurations")
        .build();

    let profile_row = ActionRow::builder()
        .title("Active Profile")
        .subtitle("Select or store custom profiles into ~/.config/rrgb/profiles.json")
        .build();

    let profile_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    let profile_dropdown = gtk4::DropDown::from_strings(&["Default"]);
    let save_profile_btn = gtk4::Button::with_label("Save");
    let delete_profile_btn = gtk4::Button::with_label("Delete");

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

    // ========================================================
    // FEATURE 4: Thermal Reactive Sync
    // ========================================================
    let thermal_group = PreferencesGroup::builder()
        .title("Thermal Reactive Hardware Sync")
        .description("Dynamically map hardware CPU/GPU temperatures to live color gradients")
        .build();

    let thermal_switch = SwitchRow::builder()
        .title("Thermal Reactive Mode")
        .subtitle("Current CPU Temperature: polling...")
        .active(false)
        .build();

    thermal_group.add(&thermal_switch);
    content_box.append(&thermal_group);

    // ========================================================
    // FEATURE 5: Independent Device Zones (Keyboard vs Lightbar)
    // ========================================================
    let zones_group = PreferencesGroup::builder()
        .title("Independent Hardware Zones")
        .description("Separate control for per-key keyboard and front lightbar / dragon logo")
        .build();

    let zones_switch = SwitchRow::builder()
        .title("Independent Lightbar Zone")
        .subtitle("Configure front lightbar / dragon logo separately from the keyboard keys")
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
        .title("Lightbar & Logo Color")
        .with_alpha(false)
        .build();

    let bar_color_btn = gtk4::ColorDialogButton::builder()
        .dialog(&bar_dialog)
        .rgba(&bar_rgba)
        .valign(gtk4::Align::Center)
        .sensitive(initial_settings.independent_zones)
        .build();

    let bar_row = ActionRow::builder()
        .title("Lightbar & Logo Color")
        .subtitle("Custom dedicated illumination for the front lightbar and dragon logo")
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

    // ========================================================
    // 3. Color Customization (Picker & Swatches)
    // ========================================================
    let color_group = PreferencesGroup::builder()
        .title("Keyboard Color Customization")
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
        let draw_ref = drawing_area.clone();
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
            draw_ref.queue_draw();
        });
        flow.append(&btn);
    }
    color_group.add(&flow);
    content_box.append(&color_group);

    // ========================================================
    // FEATURE 1: Live Search & Filter Bar
    // ========================================================
    let presets_group = PreferencesGroup::builder()
        .title("Hardware Animation Profiles (50)")
        .description("Hardware-offloaded profiles executed directly on the onboard MCU")
        .build();

    let search_entry = gtk4::SearchEntry::builder()
        .placeholder_text("Search effects by name, tag, or description...")
        .margin_bottom(8)
        .build();
    presets_group.add(&search_entry);

    // ========================================================
    // FEATURE 6: Favorites / Pinned Effects
    // ========================================================
    let fav_expander = ExpanderRow::builder()
        .title("Favorites & Pinned (★)")
        .subtitle("Your favorite quick-access effects")
        .expanded(true)
        .build();
    presets_group.add(&fav_expander);

    // Radio button group coordinator
    let first_check: Rc<RefCell<Option<gtk4::CheckButton>>> = Rc::new(RefCell::new(None));

    // Refresh Favorites List in fav_expander
    let fav_rows: Rc<RefCell<Vec<ActionRow>>> = Rc::new(RefCell::new(Vec::new()));
    let refresh_favorites_view = {
        let fav_expander = fav_expander.clone();
        let state = state.clone();
        let first_check = first_check.clone();
        let fav_rows = fav_rows.clone();
        let draw_ref = drawing_area.clone();
        Rc::new(move || {
            // Remove existing rows
            for r in fav_rows.borrow().iter() {
                fav_expander.remove(r);
            }
            fav_rows.borrow_mut().clear();

            let st = state.lock().unwrap();
            let favs = st.app_data.favorites.clone();
            let active_eff = st.settings.effect.clone();
            drop(st);

            fav_expander.set_subtitle(&format!("{} pinned favorite preset(s)", favs.len()));

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
                    let draw = draw_ref.clone();
                    check.connect_toggled(move |btn| {
                        if btn.is_active() {
                            let mut st = st_clone.lock().unwrap();
                            st.settings.effect = eff_id.clone();
                            let _ = settings::save(&st.settings, &st.config_path);
                            let s_copy = st.settings.clone();
                            thread::spawn(move || apply_hardware(&s_copy));
                            draw.queue_draw();
                        }
                    });

                    fav_expander.add_row(&row);
                    fav_rows.borrow_mut().push(row);
                }
            }
        })
    };
    (refresh_favorites_view)();

    // Effect Row Representation for filtering & dynamic favorites
    struct EffectRowWidget {
        id: &'static str,
        name: &'static str,
        desc: &'static str,
        cat: &'static str,
        row: ActionRow,
    }

    let all_effect_widgets: Rc<RefCell<Vec<EffectRowWidget>>> = Rc::new(RefCell::new(Vec::new()));
    type CategoryExpander = (ExpanderRow, Vec<ActionRow>);
    let category_expanders: Rc<RefCell<Vec<CategoryExpander>>> = Rc::new(RefCell::new(Vec::new()));

    let categories = [
        ("Core", "Classics and Fundamentals"),
        ("Sci-Fi", "Cyberpunk, Sci-Fi and Digital"),
        ("Nature", "Nature and Atmospheric Elements"),
        ("Space", "Astronomy and Deep Cosmos"),
        ("Ambient", "Mood, Chill and Ambient"),
        ("Tactical", "Tactical, Action and Dynamic"),
        ("Harmonies", "Exotic Chromatic Harmonies"),
    ];

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

        let mut row_list = Vec::new();

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
            let draw_ref = drawing_area.clone();
            check.connect_toggled(move |btn| {
                if btn.is_active() {
                    let mut st = st_clone.lock().unwrap();
                    st.settings.effect = eff_id.clone();
                    let _ = settings::save(&st.settings, &st.config_path);
                    let s_copy = st.settings.clone();
                    thread::spawn(move || apply_hardware(&s_copy));
                    draw_ref.queue_draw();
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
            row_list.push(row.clone());

            all_effect_widgets.borrow_mut().push(EffectRowWidget {
                id: eff.id,
                name: eff.name,
                desc: eff.description,
                cat: cat_name,
                row,
            });
        }

        category_expanders
            .borrow_mut()
            .push((expander.clone(), row_list));
        presets_group.add(&expander);
    }

    content_box.append(&presets_group);

    // Search filter callback
    let widgets_ref = all_effect_widgets.clone();
    let expanders_ref = category_expanders.clone();
    search_entry.connect_search_changed(move |entry| {
        let query = entry.text().to_lowercase().trim().to_string();
        let widgets = widgets_ref.borrow();
        let expanders = expanders_ref.borrow();

        for w in widgets.iter() {
            if query.is_empty() {
                w.row.set_visible(true);
            } else {
                let matches = w.name.to_lowercase().contains(&query)
                    || w.desc.to_lowercase().contains(&query)
                    || w.cat.to_lowercase().contains(&query)
                    || w.id.to_lowercase().contains(&query);
                w.row.set_visible(matches);
            }
        }

        if !query.is_empty() {
            for (expander, rows) in expanders.iter() {
                let has_visible = rows.iter().any(|r| r.is_visible());
                expander.set_visible(has_visible);
                if has_visible {
                    expander.set_expanded(true);
                }
            }
        } else {
            for (expander, _) in expanders.iter() {
                expander.set_visible(true);
            }
        }
    });

    // ========================================================
    // Connect remaining Event Handlers
    // ========================================================
    let st_picker = state.clone();
    let draw_picker = drawing_area.clone();
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
        draw_picker.queue_draw();
    });

    let st_bar_color = state.clone();
    let draw_bar = drawing_area.clone();
    bar_color_btn.connect_notify_local(Some("rgba"), move |btn, _| {
        let rgba = btn.rgba();
        let r = (rgba.red() * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (rgba.green() * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (rgba.blue() * 255.0).round().clamp(0.0, 255.0) as u8;
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);

        let mut st = st_bar_color.lock().unwrap();
        st.settings.bar_color = Some(hex);
        let _ = settings::save(&st.settings, &st.config_path);
        let s_copy = st.settings.clone();
        thread::spawn(move || apply_hardware(&s_copy));
        draw_bar.queue_draw();
    });

    let st_zones = state.clone();
    let bar_color_btn_ref = bar_color_btn.clone();
    let draw_zones = drawing_area.clone();
    zones_switch.connect_notify_local(Some("active"), move |row, _| {
        let active = row.is_active();
        bar_color_btn_ref.set_sensitive(active);
        let mut st = st_zones.lock().unwrap();
        st.settings.independent_zones = active;
        let _ = settings::save(&st.settings, &st.config_path);
        let s_copy = st.settings.clone();
        thread::spawn(move || apply_hardware(&s_copy));
        draw_zones.queue_draw();
    });

    let st_power = state.clone();
    let draw_power = drawing_area.clone();
    power_row.connect_notify_local(Some("active"), move |row, _| {
        let mut st = st_power.lock().unwrap();
        st.settings.enabled = row.is_active();
        let _ = settings::save(&st.settings, &st.config_path);
        let s_copy = st.settings.clone();
        thread::spawn(move || apply_hardware(&s_copy));
        draw_power.queue_draw();
    });

    let st_bright = state.clone();
    let draw_bright = drawing_area.clone();
    brightness_scale.connect_value_changed(move |sc| {
        let mut st = st_bright.lock().unwrap();
        st.settings.brightness = (sc.value() / 100.0) as f32;
        let _ = settings::save(&st.settings, &st.config_path);
        let s_copy = st.settings.clone();
        thread::spawn(move || apply_hardware(&s_copy));
        draw_bright.queue_draw();
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

    // Profile Dropdown Selection
    let st_prof_sel = state.clone();
    let color_btn_prof = color_btn.clone();
    let bright_prof = brightness_scale.clone();
    let speed_prof = speed_scale.clone();
    let power_prof = power_row.clone();
    let draw_prof = drawing_area.clone();
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

            let mut st_mut = st_prof_sel.lock().unwrap();
            st_mut.settings = target_settings.clone();
            let _ = settings::save(&st_mut.settings, &st_mut.config_path);
            let s_copy = st_mut.settings.clone();
            thread::spawn(move || apply_hardware(&s_copy));

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
            draw_prof.queue_draw();
        }
    });

    // Save Profile Dialog
    let window_ref = window.clone();
    let st_save_prof = state.clone();
    let refresh_dropdown_save = refresh_profile_dropdown.clone();
    save_profile_btn.connect_clicked(move |_| {
        let entry = gtk4::Entry::builder()
            .placeholder_text("Enter profile name...")
            .margin_top(8)
            .margin_bottom(8)
            .build();

        let dialog = libadwaita::MessageDialog::builder()
            .transient_for(&window_ref)
            .heading("Save Custom Profile")
            .body("Give a unique name for your current lighting configuration:")
            .build();
        dialog.set_extra_child(Some(&entry));
        dialog.add_response("cancel", "Cancel");
        dialog.add_response("save", "Save");
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
    let draw_timer = drawing_area.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(1500), move || {
        if let Some(temp) = read_cpu_temp() {
            let mut st = st_timer.lock().unwrap();
            st.current_temp = temp;
            thermal_label_row.set_subtitle(&format!("Live CPU Temperature: {:.1}°C", temp));
            if st.thermal_enabled {
                let heat_color = temp_to_color(temp);
                st.settings.color = heat_color;
                let s_copy = st.settings.clone();
                thread::spawn(move || apply_hardware(&s_copy));
                draw_timer.queue_draw();
            }
        }
        glib::ControlFlow::Continue
    });

    window.set_content(Some(&main_box));
    window.present();
}
