use clap::Parser;

#[derive(Parser)]
#[command(name = "envix")]
#[command(bin_name = "envix")]
#[command(author, version, about, long_about = None)]
enum App {
    /// Inject environment variables into shell command
    Inject(InjectArgs),
}

#[derive(clap::Args)]
struct InjectArgs {
    #[arg(short, long, default_value = ".env")]
    envfile: String,

    #[arg(last = true)]
    slop: Vec<String>,
}

fn main() {
    match App::parse() {
        App::Inject(args) => inject(args),
    }
}

fn inject(args: InjectArgs) {
    println!("Injecting...");
    println!("Envfile: {:?}", args.envfile);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inject_args() {
        let app = App::parse_from(&["envix", "inject", "--", "echo", "$FOO"]);
        let args = match app {
            App::Inject(args) => args,
        };
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
