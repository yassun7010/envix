#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Env {
    Value(String),
    GoogleCloudSecretManagerKey(String),
}
