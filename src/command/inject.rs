use crate::env::ExpandEnvs;

#[derive(clap::Args)]
pub struct InjectArgs {
    #[arg(long, default_value = "envix.toml")]
    pub config: std::path::PathBuf,

    #[arg(long)]
    pub stage: Option<String>,

    #[arg(last = true)]
    pub slop: Vec<String>,
}

pub fn inject(args: InjectArgs) -> Result<(), crate::Error> {
    let config = crate::config::from_filepath(args.config)?;

    let option;
    let mut command = if cfg!(target_os = "windows") {
        option = "/C";
        std::process::Command::new("cmd")
    } else {
        option = "-c";
        std::process::Command::new("sh")
    };

    for (key, value) in config.expand_envs(args.stage.as_deref())? {
        command.env(key, value.to_revel());
    }

    let output = command
        .args([option, args.slop.join(" ").as_str()])
        .output()?;

    let stdout = trim(&output.stdout);
    let stderr = trim(&output.stderr);

    if !stdout.is_empty() {
        println!("{}", stdout);
    }
    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }

    (!output.status.success()).then(|| std::process::exit(output.status.code().unwrap_or(1)));

    Ok(())
}

fn trim(s: &[u8]) -> String {
    String::from_utf8_lossy(s).to_string().trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::App;
    use assert_matches::assert_matches;
    use clap::Parser;

    #[test]
    fn run_inject_command() -> crate::tests::Result {
        let App::Inject(args) = App::parse_from(["envix", "inject", "--", "echo", "$FOO"]) else {
            panic!("Expected Inject variant")
        };

        assert_eq!(args.config.to_string_lossy(), "envix.toml");
        assert_eq!(args.slop, vec!["echo", "$FOO"]);

        Ok(())
    }

    #[test]
    fn run_inject_command_with_config_option() -> crate::tests::Result {
        let App::Inject(args) = App::parse_from([
            "envix",
            "inject",
            "--config",
            "test.toml",
            "--",
            "echo",
            "$FOO",
        ]) else {
            panic!("Expected Inject variant")
        };

        assert_matches!(args.config.to_str(), Some("test.toml"));
        assert_eq!(args.slop, vec!["echo", "$FOO"]);

        Ok(())
    }
}
