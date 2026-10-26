use std::path::Path;

use clap::{Parser, Subcommand};
mod db;
mod music;

/// Betterautoplay is a simple cli tool to improve your apple music experience
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display the currently playing song
    Now,
    /// Shows recent plays and skips
    Status,
    /// Recommend songs based on your listening history
    Recommend {
        /// Number of songs to recommend
        #[arg(short, long, default_value_t = 10)]
        num_songs: usize,
    },
    /// Seed from the current song
    Start,
    /// Test the queue
    TestQueue {
        /// Title of the test song
        #[arg(short, long, default_value = "Heroes")]
        title: String,

        /// Artists of the test song
        #[arg(short, long, default_value = "David Bowie")]
        artist: String,
    },
    /// Get a report of your listening history
    Report {
        /// Number of days to include in the report
        #[arg(short, long, default_value_t = 30)]
        days: u32,
    },
    /// Skip to the next track
    Next,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let db = db::Db::open(Path::new("./data"))?;

    match args.command {
        Commands::Now => {
            let now_playing = music::now_playing()?;
            if let Some(now_playing) = now_playing {
                println!("{}", now_playing);
            } else {
                println!("No song is currently playing");
            }
        }
        Commands::Next => {
            let skipped = music::next_track()?;
            if skipped {
                println!("Skipped to the next track");
            } else {
                println!("Nothing playing");
            }
        }
        Commands::Status => {
            let recent_plays = db.recent_plays(10)?;
            if recent_plays.is_empty() {
                println!("No recent plays found");
            } else {
                println!("Recent plays:");
                for play in recent_plays {
                    println!("{}", play);
                }
            }
        }
        Commands::Recommend { num_songs } => {
            println!("Recommending {num_songs} songs based on your listening history");
        }
        Commands::Start => {
            println!("Starting the music player");
        }
        Commands::TestQueue { artist, title } => {
            println!("Testing the queue with {title} by {artist}");
        }
        Commands::Report { days } => {
            println!("Generating a report for the last {days} days");
        }
    }

    Ok(())
}
