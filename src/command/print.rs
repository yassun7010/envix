#[derive(clap::Args)]
pub struct PrintArgs {
    /// Path to the envix config file
    #[arg(long, default_value = "envix.toml")]
    pub config: std::path::PathBuf,

    /// Format style
    #[arg(long, default_value = "pretty")]
    pub format: PrintFormat,

    /// Stage name
    #[arg(long)]
    pub stage: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub enum PrintFormat {
    Pretty,
    Env,
}

impl clap::ValueEnum for PrintFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[PrintFormat::Pretty, PrintFormat::Env]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            PrintFormat::Pretty => Some(clap::builder::PossibleValue::new("pretty")),
            PrintFormat::Env => Some(clap::builder::PossibleValue::new("env")),
        }
    }
}

impl std::str::FromStr for PrintFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pretty" => Ok(Self::Pretty),
            "env" => Ok(Self::Env),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

pub fn print(args: PrintArgs) -> Result<(), crate::Error> {
    let config = crate::config::from_filepath(args.config, args.stage.as_deref())?;
    let stage = args.stage.as_deref();

    match args.format {
        PrintFormat::Pretty => print_pretty(config, stage),
        PrintFormat::Env => print_env(config, stage),
    }

    Ok(())
}

fn print_pretty(config: crate::Config, stage: Option<&str>) {
    for (key, value) in config.get_vars(stage) {
        println!("{} = {}", key, value);
    }
    for (key, _) in config.get_secrets(stage) {
        println!("{} = *****************************", key);
    }
}

fn print_env(config: crate::Config, stage: Option<&str>) {
    for (key, value) in config.get_vars(stage) {
        println!("{}={}", key, value);
    }
    for (key, _) in config.get_secrets(stage) {
        println!("{}=*****************************", key);
    }
}

#[cfg(test)]
mod tests {
    use crate::App;
    use clap::Parser;

    #[test]
    fn print_args() -> crate::tests::Result {
        let App::Print(args) = App::parse_from(["envix", "print"]) else {
            panic!("Expected Inject variant")
        };

        assert_eq!(args.config.to_string_lossy(), "envix.toml");

        Ok(())
    }
}
