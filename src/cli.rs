use clap::{Parser, Subcommand};

/// Twitch VOD Recover CLI
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Recover VOD from Twitchtracker URL
    Twitchtracker {
        /// URL from Twitchtracker
        #[arg(value_parser)]
        url: String,
    },
    /// Manually recover VOD
    Manual {
        /// Streamer name
        #[arg(value_parser)]
        streamer: String,
        /// VOD ID
        #[arg(value_parser)]
        vod_id: String,
        /// Date in format YYYY-MM-DD HH:MM
        #[arg(value_parser)]
        date: String,
    },
    /// Concat and remux existing TS files in a directory
    Concat {
        /// Directory containing TS files
        #[arg(value_parser)]
        dir: String,
    },
}
