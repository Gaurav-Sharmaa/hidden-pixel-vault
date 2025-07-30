use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "A secure PNG steganography tool with atomic operations"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode a secret message into a PNG file
    Encode {
        path: String,
        chunk_type: String,
        message: String,
    },
    /// Decode a secret message from a PNG file
    Decode { path: String, chunk_type: String },
    /// Remove a chunk from a PNG file
    Remove { path: String, chunk_type: String },
    /// Print all available chunks in a PNG file
    Print { path: String },
    /// Restore original file from backup
    Restore { path: String },
    /// Clean up backup and temporary files
    Cleanup { path: String },
    /// Show file status and backup information
    Status { path: String },
}
