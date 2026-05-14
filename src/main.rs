use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    url: Option<String>,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn parse_url<'a>(args: &'a Args) -> &'a str {
    &args.url
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let usr = env::var("USER").unwrap_or_else(|_| "unknown".to_string());


    let lib = PathBuf::from("/home").join(&usr);
    let opt = PathBuf::from("output");

    let youtube = lib.join("yt-dlp");
    let ffmpeg = lib.join("vids");

    let args = Args::parse();
    let vid: &str = parse_url(&args);


    println!("Good morning saar....");
    if let Some(url) = args.name {
        j
    }

    Ok(())
}
