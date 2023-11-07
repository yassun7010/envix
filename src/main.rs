use clap::Parser;

#[derive(Parser)]
#[command(name = "envix")]
#[command(bin_name = "envix")]
#[command(author, version, about, long_about = None)]
enum App {
    /// Inject environment variables into shell command
    Inject(InjectArgs),
    Sync(SyncArgs),
}

#[derive(clap::Args)]
struct InjectArgs {
    #[arg(long)]
    no_cache: bool,
    #[arg(long)]
    scope: Option<String>,
}

#[derive(clap::Args)]
struct SyncArgs {}

fn main() {
    match App::parse() {
        App::Inject(args) => inject(args),
        App::Sync(args) => sync(args),
    }
}

fn inject(args: InjectArgs) {
    println!("Injecting...");
    println!("No cache: {:?}", args.no_cache);
}

fn sync(_args: SyncArgs) {
    println!("Syncing...");
}
