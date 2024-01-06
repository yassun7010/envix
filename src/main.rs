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
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .map_event_format(|e| e.compact().with_target(false).without_time())
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::io::stderr)
        .finish();

    tracing_log::LogTracer::init().unwrap();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    if let Err(err) = App::run() {
        log::error!("{}", err);
        std::process::exit(1);
    }
}

#[cfg(test)]
pub mod tests {
    pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    macro_rules! config {
        ($fname:expr) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/config/", $fname)
        };
    }

    pub(crate) use config;
}
