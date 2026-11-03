use criterion::{black_box, criterion_group, criterion_main, Criterion};
use r_steelseries::build_scene;
use r_steelseries::protocol::{empty_effect_packet, key_packets, refresh_packet};
use r_steelseries::settings::Settings;

fn bench_scene_building(c: &mut Criterion) {
    let mut group = c.benchmark_group("effects_and_packets");

    let effects = [
        "static",
        "rainbow",
        "aurora",
        "fire",
        "matrix",
        "starlight",
        "police",
        "pulse",
    ];

    for &eff in &effects {
        let settings = Settings {
            enabled: true,
            effect: eff.to_string(),
            color: "#ff0000".to_string(),
            speed: 0.5,
            brightness: 1.0,
            direction: "right".to_string(),
            wave_size: 0.5,
            ..Default::default()
        };

        group.bench_function(format!("build_scene_{}", eff), |b| {
            b.iter(|| {
                let scene = build_scene(black_box(&settings));
                black_box(scene);
            })
        });
    }

    group.bench_function("packet_serialization", |b| {
        let settings = Settings {
            enabled: true,
            effect: "rainbow".to_string(),
            ..Default::default()
        };
        let scene = build_scene(&settings);

        b.iter(|| {
            let k_pkts = key_packets(black_box(&scene.keys));
            let empty = empty_effect_packet(black_box(0));
            let ref_pkt = refresh_packet();
            black_box((k_pkts, empty, ref_pkt));
        })
    });

    group.finish();
}

criterion_group!(benches, bench_scene_building);
criterion_main!(benches);
