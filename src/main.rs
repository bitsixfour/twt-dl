use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let libraries_dir = PathBuf::from("/home/will/Documents/twtdp");
    let output_dir = PathBuf::from("output");
    let yt_dlp = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");
    let url = "https://www.youtube.com/watch?v=gXtp6C-3JKo";

    ensure_exists(&yt_dlp, "yt-dlp binary").await?;
    ensure_exists(&ffmpeg, "ffmpeg binary").await?;
    fs::create_dir_all(&output_dir)
        .await
        .with_context(|| format!("failed to create output directory at {}", output_dir.display()))?;

    let title = fetch_title(&yt_dlp, url).await?;
    let output_path = output_dir.join("my-video.mp4");
    download_video(&yt_dlp, &ffmpeg, url, &output_path).await?;

    println!("Downloaded \"{title}\" to {}", output_path.display());
    Ok(())
}

async fn ensure_exists(path: &Path, label: &str) -> Result<()> {
    let exists = fs::try_exists(path)
        .await
        .with_context(|| format!("failed to check {label} at {}", path.display()))?;
    if !exists {
        bail!("{label} not found at {}", path.display());
    }

    Ok(())
}

async fn fetch_title(yt_dlp: &Path, url: &str) -> Result<String> {
    let output = Command::new(yt_dlp)
        .args(["--print", "title", "--no-playlist", "--skip-download", url])
        .output()
        .await
        .with_context(|| format!("failed to run {} for title lookup", yt_dlp.display()))?;

    if !output.status.success() {
        bail!(
            "yt-dlp title lookup failed with status {}",
            output.status
        );
    }

    let title = String::from_utf8(output.stdout).context("yt-dlp returned non-UTF8 title output")?;
    Ok(title.trim().to_owned())
}

async fn download_video(yt_dlp: &Path, ffmpeg: &Path, url: &str, output_path: &Path) -> Result<()> {
    let ffmpeg_location = ffmpeg.parent().unwrap_or_else(|| Path::new("."));
    let status = Command::new(yt_dlp)
        .arg("--ffmpeg-location")
        .arg(ffmpeg_location)
        .arg("--no-playlist")
        .arg("--output")
        .arg(output_path)
        .arg(url)
        .status()
        .await
        .with_context(|| format!("failed to run {} for download", yt_dlp.display()))?;

    if !status.success() {
        bail!("yt-dlp download failed with status {status}");
    }

    Ok(())
}
