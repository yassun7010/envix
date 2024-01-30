#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug, Default)]
pub struct GoogleCloudSecretManagerService {
    /// resource name.
    #[schemars(example = "resource_name_example")]
    pub resource_name: String,
}

fn resource_name_example() -> Vec<String> {
    vec![
        "projects/1234567890/secrets/secret_name".to_owned(),
        "projects/1234567890/secrets/secret_name/versions/2".to_owned(),
    ]
}
