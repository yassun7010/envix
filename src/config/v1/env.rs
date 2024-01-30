use super::service::Service;

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Service(Service),
}
