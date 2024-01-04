#[derive(clap::Args)]
pub struct InjectArgs {
    #[arg(short, long, default_value = ".env")]
    pub envfile: String,

    #[arg(last = true)]
    pub slop: Vec<String>,
}

pub fn inject(args: InjectArgs) {
    println!("Injecting...");
    println!("Envfile: {:?}", args.envfile);
}

#[cfg(test)]
mod tests {
    use crate::App;
    use clap::Parser;

    #[test]
    fn inject_args() {
        let app = App::parse_from(&["envix", "inject", "--", "echo", "$FOO"]);
        let args = match app {
            App::Inject(args) => args,
        };

        assert_eq!(args.envfile, ".env");
        assert_eq!(args.slop, vec!["echo", "$FOO"]);
    }

    #[test]
    fn inject_args_with_envfile() {
        let app = App::parse_from(&[
            "envix",
            "inject",
            "--envfile",
            ".env.test",
            "--",
            "echo",
            "$FOO",
        ]);
        let args = match app {
            App::Inject(args) => args,
        };

        assert_eq!(args.envfile, ".env.test");
        assert_eq!(args.slop, vec!["echo", "$FOO"]);
    }
}
