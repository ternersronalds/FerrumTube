
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "ferrumtube")]
#[command(about = "A CLI YouTube video downloader in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Download {
        url: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Download { url } => {
            println!("ferrumtube: Downloading video from: {}", url);
            let status = Command::new("yt-dlp")
                .arg(url)
                .status();
            match status {
                Ok(s) if s.success() => println!("ferrumtube: Download completed."),
                Ok(s) => eprintln!("ferrumtube: yt-dlp exited with status: {}", s),
                Err(e) => eprintln!("ferrumtube: Failed to run yt-dlp: {}", e),
            }
        }
    }
}
