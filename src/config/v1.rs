use self::service::Service;

mod env;
mod service;
mod stage;
use crate::env::ExpandEnvs;

/// Envix config version 1.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct ConfigV1 {
    pub envix: ConfigV1Info,

    #[serde(flatten)]
    pub common: Stage,

    #[serde(default)]
    pub stages: indexmap::IndexMap<String, Stage>,
}

impl ExpandEnvs for ConfigV1 {
    fn expand_envs(
        &self,
        _stage: Option<&str>,
    ) -> Result<indexmap::IndexMap<&str, crate::env::Value>, crate::Error> {
        Ok(Default::default())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct ConfigV1Info {
    pub version: ConfigV1Version,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ConfigV1Version {
    V1 = 1,
}

impl Default for ConfigV1Version {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Stage {
    #[serde(default)]
    pub envs: indexmap::IndexMap<String, env::Env>,

    #[serde(default)]
    pub services: indexmap::IndexMap<String, Service>,

    #[serde(default)]
    pub stages: indexmap::IndexMap<String, Stage>,
}

// pub(crate) fn get_stage(config: &ConfigV1, stage: Option<&str>) -> Result<(), Error> {
//     if !config.stages.is_empty() {
//         match stage {
//             Some(stage) => {
//                 if !config.stages.contains_key(stage) {
//                     return Err(Error::StageNotFound(stage.to_owned()));
//                 }
//             }
//             None => {
//                 return Err(Error::StageNotSpecified);
//             }
//         }
//     }

//     Ok(())
// }

// #[derive(thiserror::Error, Debug)]
// pub enum Error {
//     #[error("\"stages.{0}\" does not found in config file.")]
//     StageNotFound(String),

//     #[error("Stage is not specified.")]
//     StageNotSpecified,
// }
