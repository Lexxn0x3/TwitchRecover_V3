use std::error::Error;

pub fn parse_m3u8(content: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut segments = Vec::new();
    for line in content.lines() {
        if line.starts_with("#") {
            continue;
        }
        let segment = line.replace("unmuted", "muted");
        segments.push(segment);
    }
    Ok(segments)
}
