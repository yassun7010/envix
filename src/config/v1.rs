#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct ConfigV1 {
    pub envix: ConfigV1Info,
    pub vars: indexmap::IndexMap<String, String>,
    pub stages: indexmap::IndexMap<String, Stage>,
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Stage {
    pub vars: indexmap::IndexMap<String, String>,
}

pub(crate) fn validate_v1(config: &ConfigV1, stage: Option<&str>) -> Result<(), crate::Error> {
    if !config.stages.is_empty() {
        if let Some(stage) = stage {
            println!("Stage: {}", stage);
        }
    }

    Ok(())
}
