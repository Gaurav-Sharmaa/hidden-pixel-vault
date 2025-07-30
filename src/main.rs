use crate::args::Args;
use crate::args::Commands::{Cleanup, Decode, Encode, Print, Remove, Restore, Status};
use crate::commands::{
    cleanup_files, decode, encode, print, remove, restore_original, show_status,
};
use clap::Parser;

mod args;
mod atomic_file;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();

    let result = match &args.command {
        Encode {
            path,
            chunk_type,
            message,
        } => encode(path, chunk_type, message),
        Decode { path, chunk_type } => decode(path, chunk_type),
        Remove { path, chunk_type } => remove(path, chunk_type),
        Print { path } => print(path),
        Restore { path } => restore_original(path),
        Cleanup { path } => cleanup_files(path),
        Status { path } => show_status(path),
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!(" ❌ Error: {}", e);
            eprintln!("💡  Tip: Use 'restore' command if you need to revert changes");
            std::process::exit(1);
        }
    }
}
