#[macro_use]
extern crate lazy_static;
pub mod handler;
pub mod test_routes;
use actix_web::{middleware, web, App, HttpServer};
use etcd_client::Client;
use simple_logger::SimpleLogger;
use std::{env, io};

use crate::handler::*;
async fn get_etcd_values() -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut client = Client::connect(["localhost:2379"], None).await?;
    let resp = client.get("IP", None).await?;
    let kv = resp.kvs().first().ok_or("Empty keyvalue")?;
    let ip = kv.value_str()?.to_owned();
    let resp = client.get("PORT", None).await?;
    let kv = resp.kvs().first().ok_or("Empty keyvalue")?;
    let port = kv.value_str()?.to_owned();
    Ok((ip, port))
}
#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    SimpleLogger::new().init().unwrap();
    let etcd_values = get_etcd_values().await;
    let (ip, port) = if etcd_values.is_ok() {
        etcd_values.unwrap()
    } else {
        println!("using defaults");
        log::warn!("Using default IP and PORT");
        ("127.0.0.1".to_string(), "8080".to_string())
    };

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/rdevices/forecast/{day}").route(web::get().to(get_fake_forecast)),
            )
            .service(
                web::scope("/rwdevices")
                    .route("/fake_switch", web::post().to(turn_switch))
                    .route("/fake_switch", web::get().to(get_switch_status)),
            )
    })
    .bind(&format!("{}:{}", ip, port))?
    .run()
    .await
}
