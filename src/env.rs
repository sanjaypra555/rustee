use std::env;
use log::debug;


use crate::Args;
use crate::error::{Error, InvalidRusteeMode};

const CONST_RUSTEE_MODE: &str = "RUSTEE_MODE";
const CONST_APPEND_MODE: &str = "a";
const CONST_UNIQUE_MODE: &str = "u";
const CONST_APPEND_UNIQUE_MODE: &str = "au";

pub fn override_args_with_env(args: &mut Args) -> Result<(), Error> {
    let rustee_mode = env::var(CONST_RUSTEE_MODE)?;
    debug!("RUSTEE_MODE is {}", rustee_mode);
    match rustee_mode.as_str() {
        CONST_APPEND_MODE => args.append = true,
        CONST_UNIQUE_MODE => args.unique = true,
        CONST_APPEND_UNIQUE_MODE => {
            args.append = true;
            args.unique = true;
        }
        _ => return Err(Error::InvalidRusteeMode(InvalidRusteeMode(rustee_mode))),
    };
    Ok(())
}

