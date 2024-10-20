mod config;
#[cfg(target_family = "unix")]
mod daemon;
mod error;
mod proxy;
mod serve;
mod update;

use clap::{Args, Parser, Subcommand};
pub use error::Error;
use std::path::PathBuf;

#[cfg(target_family = "unix")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Opt {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run server
    Run(ConfigPath),
    /// Start server daemon
    #[cfg(target_family = "unix")]
    Start(ConfigPath),
    /// Restart server daemon
    #[cfg(target_family = "unix")]
    Restart(ConfigPath),
    /// Stop server daemon
    #[cfg(target_family = "unix")]
    Stop,
    /// Show the server daemon log
    #[cfg(target_family = "unix")]
    Log,
    /// Show the server daemon process
    #[cfg(target_family = "unix")]
    PS,
    /// Generate config template file (toml format file)
    GT(ConfigPath),
    /// Self update
    Update,
}

#[derive(Args)]
pub struct ConfigPath {
    /// Configuration filepath
    #[clap(default_value = "duckai.yaml")]
    pub config_path: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    match opt.commands {
        Commands::Run(args) => serve::run(args.config_path),
        #[cfg(target_family = "unix")]
        Commands::Start(args) => daemon::start(args.config_path),
        #[cfg(target_family = "unix")]
        Commands::Restart(args) => daemon::restart(args.config_path),
        #[cfg(target_family = "unix")]
        Commands::Stop => daemon::stop(),
        #[cfg(target_family = "unix")]
        Commands::PS => daemon::status(),
        #[cfg(target_family = "unix")]
        Commands::Log => daemon::log(),
        Commands::GT(path) => config::generate_template(path.config_path),
        Commands::Update => update::update(),
    }
}
