#[derive(clap::Args)]
pub struct PrintArgs {
    #[arg(long, default_value = "envix.toml")]
    pub config: std::path::PathBuf,

    #[arg(long)]
    pub stage: Option<String>,
}

pub fn print(args: PrintArgs) -> Result<(), crate::Error> {
    let config = crate::config::from_filepath(args.config, args.stage.as_deref())?;

    for (key, value) in config.get_vars(args.stage.as_deref()) {
        println!("{}={}", key, value);
    }

    Ok(())
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
