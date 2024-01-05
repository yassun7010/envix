mod command;
mod config;
mod error;

use clap::Parser;
pub use config::Config;
pub use error::Error;

#[derive(Parser)]
#[command(name = "envix")]
#[command(bin_name = "envix")]
#[command(author, version, about, long_about = None)]
pub enum App {
    /// Create a new envix config file
    New(command::NewArgs),

    /// Inject the environment variables into a command
    Inject(command::InjectArgs),

    /// Print the environment variables
    Print(command::PrintArgs),
}

impl App {
    pub fn run() -> Result<(), crate::Error> {
        App::run_from(std::env::args_os())
    }

    pub fn run_from<I, T>(itr: I) -> Result<(), crate::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        match App::parse_from(itr) {
            App::New(args) => command::new(args),
            App::Inject(args) => command::inject(args),
            App::Print(args) => command::print(args),
        }
    }
}

fn main() {
    if let Err(err) = App::run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

#[cfg(test)]
pub mod tests {
    pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;
}
