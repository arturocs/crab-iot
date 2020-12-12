use criterion::{black_box, criterion_group, criterion_main, Criterion};
use device::{rdevice::RDevice, rwdevice::RWDevice, Readable, Writable};
use serde_json::json;

pub fn criterion_benchmark1(c: &mut Criterion) {
    c.bench_function("get_status", |b| {
        let mockup_device = RDevice::new(
            "mockup_device",
            "fake_plugin",
            "../target/release/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();
        b.iter(|| {
            let status = black_box(mockup_device.get_status(&json!({})).unwrap());
            black_box(status.get("on").unwrap());
        })
    });
}

pub fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("create_device", |b| {
        b.iter(|| {
            black_box(
                RDevice::new(
                    "mockup_device",
                    "fake_plugin",
                    "../target/release/libfake_plugin.so",
                    "127.0.0.1",
                )
                .unwrap(),
            );
        })
    });
}

pub fn criterion_benchmark3(c: &mut Criterion) {
    c.bench_function("set_status", |b| {
        let mut mockup_device = RWDevice::new(
            "mockup_device",
            "fake_plugin",
            "../target/release/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();
        b.iter(|| {
            let status = black_box(mockup_device.set_status(&json!({"on":true})).unwrap());
            black_box(status.get("on").unwrap());
        })
    });
}

pub fn criterion_benchmark4(c: &mut Criterion) {
    c.bench_function("RWDevice::from_json", |b| {
        b.iter(|| {
            let device = RWDevice::from_json(
                r#"{"name":"device",
                "plugin":{
                    "device_name":"plugin",
                    "libary_path":"../target/debug/libfake_plugin.so"
                },
                "ip":"127.0.0.1"}"#,
            )
            .unwrap();
            black_box(device)
        })
    });
}

criterion_group!(
    benches,
    criterion_benchmark1,
    criterion_benchmark2,
    criterion_benchmark3,
    criterion_benchmark4,
);
criterion_main!(benches);
