use std::io::Write;

#[derive(clap::Args)]
pub struct NewArgs {
    #[arg(long, default_value = "envix.toml")]
    pub config: std::path::PathBuf,
}

pub fn new(args: NewArgs) -> Result<(), crate::Error> {
    let config = crate::Config {
        envix: crate::config::EnvixInfo {
            version: crate::config::ConfigVersion::V1,
        },
    };
    if args.config.exists() {
        return Err(crate::Error::ConfigExists(args.config));
    }
    let mut file = std::fs::File::create(args.config)?;
    file.write_all(toml::to_string_pretty(&config)?.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::App;
    use assert_matches::assert_matches;

    #[test]
    fn run_new_command() -> crate::tests::Result {
        assert_matches!(
            App::run_from(["envix", "new"]),
            Err(crate::Error::ConfigExists(_))
        );

        Ok(())
    }
}
