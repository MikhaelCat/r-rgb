#[cfg(test)]
mod tests {
    use r_steelseries::build_scene;
    use r_steelseries::protocol::{
        empty_effect_packet, key_packets, refresh_packet, FEATURE_SIZE, OUTPUT_SIZE,
    };
    use r_steelseries::settings::{format_color, parse_color, Settings};

    #[test]
    fn test_color_parsing() {
        assert_eq!(parse_color("#ff0000").unwrap(), (255, 0, 0));
        assert_eq!(parse_color("00ff00").unwrap(), (0, 255, 0));
        assert_eq!(parse_color("#0000ff").unwrap(), (0, 0, 255));
        assert_eq!(format_color((255, 128, 0)), "#ff8000");
    }

    #[test]
    fn test_packet_sizes() {
        let empty = empty_effect_packet(0);
        assert_eq!(empty.len(), FEATURE_SIZE);
        assert_eq!(empty[0], 0x0B);
        assert_eq!(empty[2], 0x00);

        let refresh = refresh_packet();
        assert_eq!(refresh.len(), OUTPUT_SIZE);
        assert_eq!(refresh[0], 0x09);
    }

    #[test]
    fn test_all_effects_generate_valid_scenes() {
        let effects = [
            "static",
            "breathing",
            "spectrum",
            "rainbow",
            "wave",
            "starlight",
            "reactive",
            "disco",
            "fire",
            "aurora",
            "neon",
            "police",
            "pulse",
            "matrix",
        ];

        for &eff in &effects {
            let s = Settings {
                enabled: true,
                effect: eff.to_string(),
                color: "#00ffff".to_string(),
                highlight: "#ffffff".to_string(),
                speed: 0.5,
                brightness: 1.0,
                direction: "right".to_string(),
                wave_size: 0.5,
            };
            let scene = build_scene(&s);
            assert!(!scene.keys.is_empty(), "Effect {} produced no keys", eff);

            let k_pkts = key_packets(&scene.keys);
            assert!(
                !k_pkts.is_empty(),
                "Effect {} generated no key packets",
                eff
            );
            for pkt in &k_pkts {
                assert_eq!(pkt.len(), FEATURE_SIZE);
                assert_eq!(pkt[0], 0x0E);
            }
        }
    }
}
