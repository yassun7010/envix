use super::env::Env;
use super::service::Service;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Stage {
    #[serde(default)]
    pub envs: indexmap::IndexMap<String, Env>,

    #[serde(default)]
    pub services: indexmap::IndexMap<String, Service>,
}
