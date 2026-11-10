#[cfg(test)]
mod tests {
    use r_rgb::build_scene;
    use r_rgb::protocol::{
        alc_key_packet, alc_report_0c, empty_effect_packet, key_packets, refresh_packet,
        FEATURE_SIZE, OUTPUT_SIZE, STEELSERIES_MSI_RGB_ALC_MODE, STEELSERIES_MSI_RGB_OPCODE,
    };
    use r_rgb::settings::{format_color, parse_color, Settings};

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
        for eff in r_rgb::EFFECTS {
            let s = Settings {
                enabled: true,
                effect: eff.id.to_string(),
                color: "#00ffff".to_string(),
                highlight: "#ffffff".to_string(),
                speed: 0.5,
                brightness: 1.0,
                direction: "right".to_string(),
                wave_size: 0.5,
                ..Default::default()
            };
            let scene = build_scene(&s);
            assert!(!scene.keys.is_empty(), "Effect {} produced no keys", eff.id);

            let k_pkts = key_packets(&scene.keys);
            assert!(
                !k_pkts.is_empty(),
                "Effect {} generated no key packets",
                eff.id
            );
            for pkt in &k_pkts {
                assert_eq!(pkt.len(), FEATURE_SIZE);
                assert_eq!(pkt[0], 0x0E);
            }
        }
    }

    #[test]
    fn test_alc_packets() {
        let zones = [
            (0u8, (255, 0, 0)),
            (1u8, (0, 255, 0)),
            (2u8, (0, 0, 255)),
            (3u8, (255, 255, 255)),
        ];
        let rep_0c = alc_report_0c(&zones);
        assert_eq!(rep_0c.len(), FEATURE_SIZE);
        assert_eq!(rep_0c[0], STEELSERIES_MSI_RGB_OPCODE);
        assert_eq!(rep_0c[2], STEELSERIES_MSI_RGB_ALC_MODE);
        assert_eq!(rep_0c[4], 0);
        assert_eq!(rep_0c[5], 255);
        assert_eq!(rep_0c[6], 0);
        assert_eq!(rep_0c[7], 0);

        let mut keys = std::collections::HashMap::new();
        for &(z, c) in &zones {
            keys.insert(
                z,
                r_rgb::protocol::Key {
                    color: c,
                    ..Default::default()
                },
            );
        }
        let rep_0e = alc_key_packet(&keys);
        assert_eq!(rep_0e.len(), FEATURE_SIZE);
        assert_eq!(rep_0e[0], 0x0E);
        assert_eq!(rep_0e[2], STEELSERIES_MSI_RGB_ALC_MODE);
    }
}
