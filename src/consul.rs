use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use surf;

#[derive(Deserialize, Debug)]
pub struct Service {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Service")]
    pub service: String,
    #[serde(rename = "Address")]
    pub address: String,
}

pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(host: S, port: u16) -> Config {
        Config {
            host: host.into(),
            port,
        }
    }
}
impl Default for Config {
    fn default() -> Config {
        Config {
            host: "localhost".to_owned(),
            port: 8500,
        }
    }
}

#[derive(Serialize)]
#[serde(rename(serialize = "PascalCase"))]
struct ServiceParams {
    name: String,
    address: String,
}

pub async fn register_service(cfg: &Config, addr: String) {
    let params = ServiceParams {
        name: format!("poc-game:{}", addr),
        address: addr,
    };

    let _resp = surf::put(format!(
        "http://{}:{}/v1/agent/service/register",
        cfg.host, cfg.port
    ))
    .body_json(&params)
    .unwrap()
    .recv_string()
    .await
    .unwrap();
}
pub async fn deregister_service(cfg: &Config, addr: String) {
    let _ = surf::put(format!(
        "http://{}:{}/v1/agent/service/deregister/poc-game:{}",
        cfg.host, cfg.port, addr
    ))
    .recv_string()
    .await
    .unwrap();
}

pub async fn get_services(cfg: &Config) -> Vec<Service> {
    let resp: HashMap<String, Service> = surf::get(format!(
        "http://{}:{}/v1/agent/services",
        cfg.host, cfg.port
    ))
    .recv_json()
    .await
    .unwrap();
    resp.into_iter()
        .map(|(_, v)| v)
        .filter(|s| s.service.starts_with("poc-game"))
        .collect()
}
