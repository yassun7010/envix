use std::borrow::Cow;

use self::stage::Stage;

mod env;
mod service;
mod stage;
use crate::env::ExpandEnvs;

/// Envix config version 1.
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug, Default)]
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

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug, Default)]
pub struct ConfigV1Info {
    pub version: ConfigV1Version,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
pub enum ConfigV1Version {
    #[default]
    V1 = 1,
}

impl schemars::JsonSchema for ConfigV1Version {
    fn schema_name() -> String {
        // Exclude the module path to make the name in generated schemas clearer.
        "ConfigV1Version".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        // Include the module, in case a type with the same name is in another module/crate
        Cow::Borrowed(concat!(module_path!(), "::ConfigV1Version"))
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::Integer.into()),
            enum_values: Some(vec![1.into()]),
            ..Default::default()
        })
    }
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
