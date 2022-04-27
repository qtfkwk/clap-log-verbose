use clap::Parser;
use clap_log_verbose::hello;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Verbosity
    #[clap(short, parse(from_occurrences))]
    verbose: u8,
}

/// Hello
fn main() {
    let args = Args::parse();

    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", match args.verbose {
        0 => "error",
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    }));

    hello();
}
