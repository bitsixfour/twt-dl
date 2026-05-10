use yt_dlp::Downloader;
use yt_dlp::client::deps::Libraries;
use std::path::PathBuf;
use anyhow::{anyhow, Context, Result};
use csv::{Reader, StringRecord};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};


use yt_dlp::Downloader;
use std::path::PathBuf;
use yt_dlp::client::deps::Libraries;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("/home/will/Documents/twtdp");
    let output_dir = PathBuf::from("output");

    let youtube = libraries_dir.join("twt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let downloader = Downloader::builder(libraries, output_dir)
        .build()
        .await?;

    let var: String = format!("https://www.youtube.com/watch?v=gXtp6C-3JKo");
    let video = downloader.fetch_video_infos(url).await?;
    let video_path = downloader.download_video(&video, "my-video.mp4").await?;
    Ok(())
}
