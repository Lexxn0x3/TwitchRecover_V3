use reqwest;
use std::error::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn download_segments(base_url: &str, segments: &[String]) -> Result<(), Box<dyn Error>> {
    let base_url = base_url.rsplit_once('/').unwrap().0;

    fs::create_dir_all("temp").await?;
    for segment in segments {
        let segment_url = format!("{}/{}", base_url, segment);
        let response = reqwest::get(&segment_url).await?;

        let filename = format!("temp/{}", segment);
        let mut file = fs::File::create(&filename).await?;
        let bytes = response.bytes().await?;
        file.write_all(&bytes).await?;
        println!("Downloaded: {}", filename);
    }

    Ok(())
}
