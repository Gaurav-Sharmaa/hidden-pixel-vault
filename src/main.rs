use crate::args::Args;
use crate::args::Commands::{Decode, Encode, Print, Remove};
use crate::commands::{decode, encode, print, remove};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Encode {
            path,
            chunk_type,
            message,
        } => encode(path, chunk_type, message),

        Decode { path, chunk_type } => decode(path, chunk_type),

        Remove { path, chunk_type } => remove(path, chunk_type),

        Print { path } => {
            print(path);
        }
    }
    Ok(())
}
