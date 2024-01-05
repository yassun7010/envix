use std::io::Read;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Config {
    pub envix: EnvixInfo,
    pub vars: indexmap::IndexMap<String, String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct EnvixInfo {
    pub version: ConfigVersion,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ConfigVersion {
    V1 = 1,
}

pub fn from_filepath<P: AsRef<std::path::Path>>(filename: P) -> Result<Config, crate::Error> {
    let filename = filename.as_ref();
    let mut file = std::fs::File::open(filename)
        .map_err(|_| crate::Error::ConfigNotFound(filename.to_owned()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = toml::from_str::<Config>(&contents)?;
    Ok(config)
}
