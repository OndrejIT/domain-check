use clap::Parser;
use log::LevelFilter;
use std::sync::OnceLock;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long)]
    pub error: bool,

    #[arg(long)]
    pub warn: bool,

    #[arg(long)]
    pub info: bool,

    #[arg(long)]
    pub debug: bool,

    #[arg(long)]
    pub trace: bool,
}

impl Args {
    pub fn log_level(&self) -> LevelFilter {
        if self.trace {
            LevelFilter::Trace
        } else if self.debug {
            LevelFilter::Debug
        } else if self.info {
            LevelFilter::Info
        } else if self.warn {
            LevelFilter::Warn
        } else if self.error {
            LevelFilter::Error
        } else {
            LevelFilter::Info
        }
    }
}

static STATIC_ARGS: OnceLock<Args> = OnceLock::new();

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let _ = STATIC_ARGS.set(args);

    Ok(())
}

pub fn get_args() -> &'static Args {
    STATIC_ARGS.get().expect("Args is not initialized")
}

pub use get_args as ARGS;
