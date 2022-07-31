use std::process::ExitCode;

use clap::Parser;
use env_logger::Env;
use log::{debug, error};

use args::Args;

use crate::error::Error;

mod args;
mod io;
mod env;
mod error;


fn main() -> ExitCode {
    let mut args: Args = Args::parse();
    let mut mode = "error";
    if args.debug {
        mode = "debug";
    }
    env_logger::Builder::from_env(Env::default().default_filter_or(mode))
        .format_timestamp(None)
        .init();

    if !args.noenv {
        match env::override_args_with_env(&mut args) {
            Err(Error::InvalidRusteeMode(e)) => {
                // print error and continue
                error!("{}", e);
            }
            _ => ()
        }
    }
    debug!("args: {:?}", args);

    if args.ignore {
        ctrlc::set_handler(|| {}).unwrap_or_else(|err| {
            error!("Could not ignore SIGINT signal: {}", err);
        });
    }

    match io::pipe(args) {
        Ok(_) => (),
        Err(e) => {
            match e {
                Error::FileNotProvided => { error!("{}", "output file not provided") }
                Error::IoError(e) => { error!("IO Error: {}", e) }
                e => { error!("Error: {}", e) }
            }
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
