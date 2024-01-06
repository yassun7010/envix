pub mod v1;
use std::io::Read;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Config {
    V1(v1::ConfigV1),
}

impl Config {
    pub fn get_vars<'a>(&'a self, stage: Option<&str>) -> impl Iterator<Item = (&'a str, &'a str)> {
        match self {
            Config::V1(config) => {
                let mut vars: indexmap::IndexMap<&str, &str> = config
                    .common
                    .vars
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();

                if let Some(stage) = stage {
                    if let Some(stage) = config.stages.get(stage) {
                        for (k, v) in stage.vars.iter() {
                            vars.insert(k.as_str(), v.as_str());
                        }
                    }
                }
                vars.into_iter()
            }
        }
    }

    pub fn get_secrets<'a>(
        &'a self,
        stage: Option<&str>,
    ) -> impl Iterator<Item = (&'a str, &'a str)> {
        match self {
            Config::V1(config) => {
                let mut secrets: indexmap::IndexMap<&str, &str> = config
                    .common
                    .secrets
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();

                if let Some(stage) = stage {
                    if let Some(stage) = config.stages.get(stage) {
                        for (k, v) in stage.secrets.iter() {
                            secrets.insert(k.as_str(), v.as_str());
                        }
                    }
                }
                secrets.into_iter()
            }
        }
    }

    pub fn get_vars_and_secrets<'a>(
        &'a self,
        stage: Option<&str>,
    ) -> indexmap::IndexMap<&'a str, &'a str> {
        let mut vars: indexmap::IndexMap<&str, &str> = self.get_vars(stage).collect();

        for (k, _) in self.get_secrets(stage) {
            if vars.contains_key(k) {
                log::warn!("Secret \"{}\" is already defined in vars", k)
            }
            vars.insert(k, "*****************************");
        }

        vars
    }
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
