use anyhow::{Context, Result, bail};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PlayerState {
    Playing,
    Paused,
}

impl std::fmt::Display for PlayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerState::Playing => write!(f, "▶"),
            PlayerState::Paused => write!(f, "⏸"),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct NowPlaying {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub player_position: f64,
    pub player_state: PlayerState,
}

impl std::fmt::Display for NowPlaying {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} by {} from {} [{} / {}]",
            self.player_state,
            self.title,
            self.artist,
            self.album,
            format_duration(self.player_position),
            format_duration(self.duration)
        )
    }
}

pub fn now_playing() -> Result<Option<NowPlaying>> {
    let now_playing = run_jxa(include_str!("js/now_playing.js"))?;
    serde_json::from_str(&now_playing).context("Failed to parse now playing data")
}

pub fn next_track() -> Result<bool> {
    let skipped = run_jxa(include_str!("js/next_track.js"))?;
    serde_json::from_str(&skipped).context("Failed to parse skipping data")
}

fn run_jxa(script: &str) -> Result<String> {
    let output = std::process::Command::new("osascript")
        .args(["-l", "JavaScript", "-e", script])
        .output()
        .context("Failed to execute osascript command")?;

    if !output.status.success() {
        bail!(
            "osascript failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn format_duration(duration: f64) -> String {
    let total = duration as u32;
    let minutes = total / 60;
    let seconds = total % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
