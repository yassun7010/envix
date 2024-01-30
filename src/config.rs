pub mod v1;
use std::io::Read;

use crate::env::{self, ExpandEnvs};

/// Envix config.
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug)]
#[serde(untagged)]
pub enum Config {
    V1(v1::ConfigV1),
}

impl Config {}

impl ExpandEnvs for Config {
    fn expand_envs(
        &self,
        stage: Option<&str>,
    ) -> Result<indexmap::IndexMap<&str, env::Value>, crate::Error> {
        match self {
            Config::V1(config) => config.expand_envs(stage),
        }
    }
}

pub fn from_filepath<P: AsRef<std::path::Path>>(filename: P) -> Result<Config, crate::Error> {
    let filename = filename.as_ref();
    let mut file = std::fs::File::open(filename)
        .map_err(|_| crate::Error::ConfigNotFound(filename.to_owned()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(toml::from_str::<Config>(&contents)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_schema() -> crate::tests::Result {
        use crate::Config;
        use schemars::schema_for;
        use std::io::Write;

        let mut file =
            std::fs::File::create(crate::tests::repository_root!("schemas/config.schema.json"))?;

        write!(
            file,
            "{}",
            serde_json::to_string_pretty(&schema_for!(Config))?
        )?;
        Ok(())
    }
}
