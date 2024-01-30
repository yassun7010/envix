use crate::env::ExpandEnvs;

#[derive(clap::Args)]
pub struct PrintArgs {
    /// Path to the envix config file
    #[arg(long, default_value = "envix.toml")]
    pub config: std::path::PathBuf,

    /// Format style
    #[arg(long, default_value = "env")]
    pub format: PrintFormat,

    /// Stage name
    #[arg(long)]
    pub stage: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub enum PrintFormat {
    Env,
    Json,
}

impl clap::ValueEnum for PrintFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[PrintFormat::Env, PrintFormat::Json]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            PrintFormat::Env => Some(clap::builder::PossibleValue::new("env")),
            PrintFormat::Json => Some(clap::builder::PossibleValue::new("json")),
        }
    }
}

impl std::str::FromStr for PrintFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "env" => Ok(Self::Env),
            "json" => Ok(Self::Json),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

pub fn print(args: PrintArgs) -> Result<(), crate::Error> {
    let config = crate::config::from_filepath(args.config)?;
    let vars_and_secrets = config.expand_envs(args.stage.as_deref())?;

    match args.format {
        PrintFormat::Env => print_env(vars_and_secrets),
        PrintFormat::Json => print_json(vars_and_secrets),
    }

    Ok(())
}

fn print_json(vars_and_secrets: indexmap::IndexMap<&str, crate::env::Value>) {
    println!(
        "{}",
        serde_json::to_string_pretty(&vars_and_secrets).unwrap()
    );
}

fn print_env(vars_and_secrets: indexmap::IndexMap<&str, crate::env::Value>) {
    for (key, value) in vars_and_secrets {
        println!("{}={}", key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::App;
    use clap::Parser;
    use rstest::rstest;

    #[test]
    fn run_print_command() -> crate::tests::Result {
        let App::Print(args) = App::parse_from(["envix", "print"]) else {
            panic!("Expected Inject variant")
        };

        assert_eq!(args.config.to_string_lossy(), "envix.toml");
        // assert_matches!(
        //     print(args),
        //     Err(crate::Error::ConfigV1(
        //         crate::config::v1::Error::StageNotSpecified
        //     ))
        // );

        Ok(())
    }

    #[rstest(
        config_filepath => [
            crate::tests::config!("envix.toml"),
            crate::tests::config!("envix_only_secrets.toml"),
            crate::tests::config!("envix_only_vars.toml"),
        ]
    )]
    fn run_print_command_with_config_option(config_filepath: &str) -> crate::tests::Result {
        let App::Print(args) = App::parse_from(["envix", "print", "--config", config_filepath])
        else {
            panic!("Expected Inject variant")
        };

        assert_eq!(args.config.to_string_lossy(), config_filepath);

        print(args)?;

        Ok(())
    }

    #[test]
    fn run_print_command_with_wrong_config_option() -> crate::tests::Result {
        let _result = App::run_from([
            "envix",
            "print",
            "--config",
            crate::tests::config!("envix_only_vars_with_stage.toml"),
        ]);

        // assert_matches!(
        //     result,
        //     Err(crate::Error::ConfigV1(
        //         crate::config::v1::Error::StageNotSpecified
        //     ))
        // );

        Ok(())
    }
}
