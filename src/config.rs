pub mod v1;
use std::io::Read;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Config {
    V1(v1::ConfigV1),
}

pub fn from_filepath<P: AsRef<std::path::Path>>(
    filename: P,
    stage: Option<&str>,
) -> Result<Config, crate::Error> {
    let filename = filename.as_ref();
    let mut file = std::fs::File::open(filename)
        .map_err(|_| crate::Error::ConfigNotFound(filename.to_owned()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = toml::from_str::<Config>(&contents)?;

    validate(&config, stage)?;

    Ok(config)
}

fn validate(config: &Config, stage: Option<&str>) -> Result<(), crate::Error> {
    match config {
        Config::V1(config) => v1::validate_v1(config, stage)?,
    }

    Ok(())
}
