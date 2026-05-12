use anyhow::Result;
use std::path::PathBuf;
use yt_dlp::client::deps::Libraries;
use yt_dlp::Downloader;


#[tokio::main]
async fn main() -> Result<()> {
    let libraries_dir = PathBuf::from("/home/will/Documents/twtdp");
    let output_dir = PathBuf::from("output");

    let yt_dlp = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(yt_dlp, ffmpeg);
    let downloader = Downloader::builder(libraries, output_dir)
        .build()
        .await?;

    let url = "https://www.youtube.com/watch?v=gXtp6C-3JKo";
    let video = downloader.fetch_video_infos(url).await?;
    let _video_path = downloader.download_video(&video, "my-video.mp4").await?;
    Ok(())
}
