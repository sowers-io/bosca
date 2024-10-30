use tokio::process::Command;
use crate::activity::Error;

pub async fn convert_to_wav(input_path: &str, output_path: &str) -> Result<(), Error> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-vn")  // Disable video
        .arg("-acodec")
        .arg("pcm_s16le")  // Set audio codec to 16-bit PCM
        .arg("-ar")
        .arg("16000")  // Sample rate
        .arg("-ac")
        .arg("2")  // Set to stereo
        .arg("-y")  // Overwrite output file if it exists
        .arg(output_path)
        .output()
        .await?;
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(error.as_ref().to_owned()));
    }
    Ok(())
}