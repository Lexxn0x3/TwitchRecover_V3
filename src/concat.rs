use std::fs::File;
use std::io::{self, Write, BufReader, BufWriter};
use std::path::Path;

pub fn concat_ts_files(dir: &str, output_file: &str, segments: &[String]) -> io::Result<()> {
    let output_path = Path::new(dir).join(output_file);
    let mut output = BufWriter::new(File::create(&output_path)?);

    for segment in segments {
        let segment_path = Path::new(dir).join(segment);
        let mut input = BufReader::new(File::open(segment_path)?);

        io::copy(&mut input, &mut output)?;
    }

    println!("Successfully concatenated to {}", output_file);
    Ok(())
}
