#[derive(clap::Args)]
pub struct InjectArgs {
    #[arg(long, default_value = "envix.toml")]
    pub config: std::path::PathBuf,

    #[arg(short, long, default_value = ".env")]
    pub envfile: String,

    #[arg(last = true)]
    pub slop: Vec<String>,
}

pub fn inject(args: InjectArgs) -> Result<(), crate::Error> {
    let config = crate::config::from_filepath(args.config)?;
    println!("Config: {:?}", config);

    let option;
    let mut command = if cfg!(target_os = "windows") {
        option = "/C";
        std::process::Command::new("cmd")
    } else {
        option = "-c";
        std::process::Command::new("sh")
    };
    std::env::set_var("FOO", "BBBB");

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
    use clap::Parser;

    #[test]
    fn inject_args() -> crate::tests::Result {
        let App::Inject(args) = App::parse_from(["envix", "inject", "--", "echo", "$FOO"]) else {
            panic!("Expected Inject variant")
        };

        assert_eq!(args.envfile, ".env");
        assert_eq!(args.slop, vec!["echo", "$FOO"]);

        Ok(())
    }

    #[test]
    fn inject_args_with_envfile() -> crate::tests::Result {
        let App::Inject(args) = App::parse_from([
            "envix",
            "inject",
            "--envfile",
            ".env.test",
            "--",
            "echo",
            "$FOO",
        ]) else {
            panic!("Expected Inject variant")
        };

        assert_eq!(args.envfile, ".env.test");
        assert_eq!(args.slop, vec!["echo", "$FOO"]);

        Ok(())
    }
}
