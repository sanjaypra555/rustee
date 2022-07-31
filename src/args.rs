use std::path::PathBuf;

use clap::Parser;

/// Pipe stdin to stdout and file
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Append the output to the file rather than overwriting
    #[clap(short, long)]
    pub append: bool,

    /// Ignore the SIGINT signal
    #[clap(short, long)]
    pub ignore: bool,

    /// Pipe only unique lines (won't make existing lines unique in file)
    #[clap(short, long)]
    pub unique: bool,

    /// Don't consider RUSTEE_MODE environment variable
    #[clap(short, long)]
    pub noenv: bool,

    /// Debug
    #[clap(short, long)]
    pub debug: bool,

    /// Output file
    #[clap(index = 1)]
    pub file: Option<PathBuf>,
}


