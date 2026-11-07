use clap::Parser;
use cli::Cli;
use harfcraft_core::constants::{APP_NAME, APP_VERSION};

fn main() {
    let args = Cli::parse();
    logging::init();
    tracing::info!("Starting {} v{}", APP_NAME, APP_VERSION);
    if let Some(path) = &args.path {
        tracing::info!("Opening {} ", path.to_string_lossy());
    }
    application::run(args.path);
    tracing::info!("Closing {} v{}", APP_NAME, APP_VERSION);
}
