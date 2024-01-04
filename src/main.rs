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
