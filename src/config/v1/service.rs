use self::google_cloud_secret_manager::GoogleCloudSecretManagerService;

mod google_cloud_secret_manager;

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug)]
#[serde(tag = "service")]
pub enum Service {
    #[serde(rename = "GoogleCloudSecretManager")]
    GoogleCloudSecretManager(GoogleCloudSecretManagerService),
}
