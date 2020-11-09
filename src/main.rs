use device::{rwdevice::RWDevice, Readable};
use warp::Filter;

#[tokio::main]
async fn main() {
    let api = warp::path!("api").map(|| {
        let mockup_device = RWDevice::new(
            "mockup_device",
            "fake_plugin",
            "./target/debug/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap();
        let status = mockup_device.get_status().unwrap();
        status.get("data").unwrap().to_string()
    });

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}
