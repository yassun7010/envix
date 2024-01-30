use super::env::Value;

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug, Default)]
pub struct Stage {
    #[serde(default)]
    pub envs: indexmap::IndexMap<String, Value>,
}
