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

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut usr: String = format!("");
    match env::var("USER") {
            Ok(user) => { let usr = user; },
            Err(e) => println!("somehow couldn't get user"),
    }



    let lib = PathBuf::from("/home").join(usr);
    let output_dir = PathBuf::from("output");

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let args = Args::parse();
    println!("Good morning saar....");
    for _ in 0..args.count {
        println!("test...");
    }

    Ok(())
}
