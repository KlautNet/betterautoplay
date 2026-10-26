use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::Connection;

pub struct Db {
    conn: Connection,
}

#[derive(Debug)]
pub struct Play {
    pub id: i64,
    pub persistent_id: String,
    pub title: String,
    pub artist: String,
    pub started_at: i64,
    pub listened: f64,
    pub duration: f64,
    pub skipped: bool,
}

impl Play {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            persistent_id: row.get("persistent_id")?,
            title: row.get("title")?,
            artist: row.get("artist")?,
            started_at: row.get("started_at")?,
            listened: row.get("listened")?,
            duration: row.get("duration")?,
            skipped: row.get("skipped")?,
        })
    }
}

//Display implementation for Play struct
impl std::fmt::Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} by {} ({} seconds listened, {} seconds duration, skipped: {})",
            self.started_at, self.title, self.artist, self.listened, self.duration, self.skipped
        )
    }
}

impl Db {
    pub fn open(dir: &Path) -> Result<Self> {
        let file: std::path::PathBuf = dir.join("autoplay.sqlite");
        std::fs::create_dir_all(dir)?;
        let conn = rusqlite::Connection::open(file)?;

        // Run migrations to create the necessary tables if they don't exist
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS plays (
                id INTEGER PRIMARY KEY,
                persistent_id TEXT NOT NULL,
                title TEXT NOT NULL,
                artist TEXT NOT NULL,
                started_at INTEGER NOT NULL,
                listened REAL NOT NULL,
                duration REAL NOT NULL,
                skipped BOOLEAN NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_plays_started_at ON plays (started_at);",
        )?;

        Ok(Db { conn })
    }
    pub fn record_play(&self, play: &Play) -> Result<()> {
        Ok(())
    }
    pub fn recent_plays(&self, limit: usize) -> Result<Vec<Play>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, persistent_id, title, artist, started_at, listened, duration, skipped FROM plays ORDER BY started_at DESC LIMIT ?1")?;

        let plays_iter = stmt.query_map([i64::try_from(limit)?], |row| Ok(Play::from_row(row)?))?;

        plays_iter
            .collect::<Result<Vec<Play>, rusqlite::Error>>()
            .context("Failed to fetch recent plays from the database")
    }
}
