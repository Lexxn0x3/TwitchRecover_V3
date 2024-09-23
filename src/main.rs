mod cli;
mod logger;
mod recover;
mod m3u8;
mod downloader;
mod concat;

use clap::Parser;
use cli::{Cli, Commands};
use recover::{recover_from_manual, recover_from_twitchtracker, concat_and_remux_existing};
use tokio;

#[tokio::main]
async fn main() {
    logger::init();
    let args = Cli::parse();

    match args.command {
        Commands::Twitchtracker { url } => {
            if let Err(e) = recover_from_twitchtracker(&url).await {
                log::error!("Error recovering VOD: {}", e);
            }
        }
        Commands::Manual { streamer, vod_id, date } => {
            if let Err(e) = recover_from_manual(&streamer, &vod_id, &date).await {
                log::error!("Error recovering VOD: {}", e);
            }
        }
        Commands::Concat { dir } => {
            if let Err(e) = concat_and_remux_existing(&dir).await {
                log::error!("Error concatenating and remuxing: {}", e);
            }
        }
    }
}
