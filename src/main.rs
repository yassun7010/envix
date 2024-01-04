mod command;
mod error;

pub use error::Error;

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

impl App {
    pub fn run() -> Result<(), crate::Error> {
        match App::parse() {
            App::Inject(args) => command::inject(args),
        }
    }
}

fn main() {
    if let Err(err) = App::run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
