use twitch_recover::{VodRecover, VodRecoverOptions};
use chrono::NaiveDateTime;
use std::error::Error;
use crate::downloader::download_segments;
use crate::m3u8::parse_m3u8;
use crate::concat::concat_ts_files;
use reqwest;
use std::process::Command;
use std::fs;
use std::io::{self, Write};

pub async fn recover_from_twitchtracker(url: &str) -> Result<(), Box<dyn Error>> {
    let options = VodRecoverOptions {
        ..Default::default()
    };

    let vod = VodRecover::from_twitchtracker(url).await?;
    let vod_url = vod.get_url(&options).await?;
    println!("Recovered VOD URL: {}", vod_url);

    let m3u8_content = reqwest::get(&vod_url).await?.text().await?;
    let segments = parse_m3u8(&m3u8_content)?;
    download_segments(&vod_url, &segments).await?;
    concat_ts_files("temp", "output.ts", &segments)?;

    remux_to_mp4("temp/output.ts", "output.mp4")?;

    prompt_to_delete_temp_files("temp")?;

    Ok(())
}

pub async fn recover_from_manual(streamer: &str, vod_id: &str, date: &str) -> Result<(), Box<dyn Error>> {
    let timestamp = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M")
        .unwrap()
        .timestamp();

    let options = VodRecoverOptions {
        ..Default::default()
    };

    let vod = VodRecover::from_manual(streamer, vod_id, timestamp);
    let vod_url = vod.get_url(&options).await?;
    println!("Recovered VOD URL: {}", vod_url);

    let m3u8_content = reqwest::get(&vod_url).await?.text().await?;
    let segments = parse_m3u8(&m3u8_content)?;
    download_segments(&vod_url, &segments).await?;
    concat_ts_files("temp", "output.ts", &segments)?;

    remux_to_mp4("temp/output.ts", "output.mp4")?;

    prompt_to_delete_temp_files("temp")?;

    Ok(())
}

pub fn concat_and_remux_existing(dir: &str) -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "ts"))
        .collect::<Vec<_>>();

    let segments = paths
        .iter()
        .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    concat_ts_files(dir, "output.ts", &segments)?;
    remux_to_mp4(&format!("{}/output.ts", dir), &format!("{}/output.mp4", dir))?;

    prompt_to_delete_temp_files(dir)?;

    Ok(())
}

fn remux_to_mp4(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file)
        .arg("-c")
        .arg("copy")
        .arg(output_file)
        .status()?;

    if !status.success() {
        return Err(Box::from("ffmpeg remuxing failed"));
    }

    println!("Successfully remuxed to {}", output_file);
    Ok(())
}

fn prompt_to_delete_temp_files(dir: &str) -> Result<(), Box<dyn Error>> {
    print!("Do you want to delete the temporary files? (yes/no): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().eq_ignore_ascii_case("yes") || input.trim().eq_ignore_ascii_case("y"){
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "ts") {
                fs::remove_file(entry.path())?;
            }
        }
        println!("Temporary files deleted.");
    } else {
        println!("Temporary files not deleted.");
    }

    Ok(())
}
