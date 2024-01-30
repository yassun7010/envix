use self::google_cloud_secret_manager::GoogleCloudSecretMangerService;

mod google_cloud_secret_manager;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Service {
    GoogleCloudSecretManager(GoogleCloudSecretMangerService),
}
