function run(argv) {
  const Music = Application("Music");
  if (!Music.running()) return "false";

  const playerState = Music.playerState();

  if (!["playing", "paused"].includes(playerState)) return "false";
  Music.nextTrack();
  return "true";
}
