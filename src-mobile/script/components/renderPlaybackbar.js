export function renderPlaybackbar(trackTitle) {
  const playbackbar = document.querySelector(".playbackbar");
  const playbackTrackName = document.querySelector(".playbackbarTrack-name");

  playbackTrackName.textContent = trackTitle;
  playbackbar.style.display = "flex";
}
