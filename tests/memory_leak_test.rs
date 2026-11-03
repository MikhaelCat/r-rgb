use r_steelseries::build_scene;
use r_steelseries::protocol::{empty_effect_packet, key_packets, refresh_packet};
use r_steelseries::settings::Settings;

#[test]
fn test_loop_memory_stability() {
    let effects = [
        "static", "breathing", "spectrum", "rainbow", "wave", "starlight",
        "reactive", "disco", "fire", "aurora", "neon", "police", "pulse", "matrix",
    ];

    // Simulate 20,000 scene generations and packet serializations
    for i in 0..20_000 {
        let eff = effects[i % effects.len()];
        let s = Settings {
            enabled: true,
            effect: eff.to_string(),
            color: "#ff8000".to_string(),
            speed: 0.5,
            brightness: 1.0,
            direction: "right".to_string(),
            wave_size: 0.5,
            ..Default::default()
        };
        let scene = build_scene(&s);
        let _pkts = key_packets(&scene.keys);
        let _empty = empty_effect_packet(0);
        let _ref = refresh_packet();
    }
}
