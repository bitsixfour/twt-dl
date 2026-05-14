use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use clap::Parser;
use std::env;
use std::error::Error;
use yt_dlp::Downloader;
use yt_dlp::client::deps::Libraries;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long)]
    url: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

async fn download_url(args: &Args, dwl: &Downloader, url: &String)  {
    let video = downloader.fetch_video_infos(url).await?;
    /* freaing yt-dlp crate doesn't work apparent;y1 */

}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let usr = env::var("USER").unwrap_or_else(|_| "unknown".to_string());


    let lib = PathBuf::from("/home").join(&usr);
    let opt = lib.join("Videos");

    let twtdl= lib.join("twt-dlp");
    let ffmpeg = lib.join("vids");

    let library = Libraries::new(twtdl, ffmpeg);
    let downloader: Downloader = Downloader::builder(library, opt).build.await?;

    let args = Args::parse();
    let vid =  args.url;

    download_url(&args, &downloader, &args.url).await?;
    println!("Good morning saar....");

    Ok(())
}
