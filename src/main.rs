mod command;
use clap::Parser;
use command::InjectArgs;

#[derive(Parser)]
#[command(name = "envix")]
#[command(bin_name = "envix")]
#[command(author, version, about, long_about = None)]
pub enum App {
    /// Inject environment variables into shell command
    Inject(InjectArgs),
}

fn main() {
    match App::parse() {
        App::Inject(args) => command::inject(args),
    }
}
