#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct GoogleCloudSecretMangerService {
    pub project_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct GoogleCloudSecretManagerEnvInfo {
    pub key: String,
    #[serde(default = "latest")]
    pub version: String,
}

fn latest() -> String {
    "latest".to_owned()
}
