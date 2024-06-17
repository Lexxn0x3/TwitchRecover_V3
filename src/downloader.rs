use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest;
use std::error::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::path::Path;

pub async fn download_and_concat_segments(base_url: &str, segments: &[String], output_file: &str) -> Result<(), Box<dyn Error>> {
    let base_url = base_url.rsplit_once('/').unwrap().0;

    // Ensure the directory for the output file exists
    if let Some(parent) = Path::new(output_file).parent() {
        fs::create_dir_all(parent).await?;
    }

    let multi_progress = MultiProgress::new();
    let overall_progress = multi_progress.add(ProgressBar::new(segments.len() as u64));
    overall_progress.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg} eta: {eta} [{wide_bar:.cyan/blue}] ").unwrap());

    let mut output = fs::File::create(output_file).await?;

    for segment in segments {
        let segment_url = format!("{}/{}", base_url, segment);
        let response = reqwest::get(&segment_url).await?;

        let bytes = response.bytes().await?;
        output.write_all(&bytes).await?;

        overall_progress.set_message(format!("Downloading {}", segment));
        overall_progress.inc(1);
    }

    overall_progress.finish_with_message("Download complete");

    Ok(())
}
