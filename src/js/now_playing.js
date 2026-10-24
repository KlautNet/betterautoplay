function run(argv) {
  const Music = Application("Music");
  if (!Music.running()) return "null";

  const playerState = Music.playerState();

  if (["playing", "paused"].includes(playerState)) {
    const track = Music.currentTrack();
    return JSON.stringify({
      title: track.name(),
      artist: track.artist(),
      album: track.album(),
      duration: track.duration(),
      player_position: Music.playerPosition(),
      player_state: playerState,
    });
  } else {
    return "null";
  }
}
