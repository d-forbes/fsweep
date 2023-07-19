mod api;

use ::clap::{Args, Parser};
use std::process;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "fsweep - a simple CLI to keep files and folders decluttered")]
struct Cli {
    /// Sets the input directory to use
    #[clap(short, long)]
    directory: String,
}

#[derive(Args)]
struct Sweep {
    /// The directory to sweep
    string: Option<String>,
}

fn main() {
    let args = Cli::parse();

    // Use function from the api module
    if let Err(e) = api::fsweep::move_files_to_new_folder(&args.directory) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
