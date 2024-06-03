import {
  getPlaylistPlayback,
  putPlaylistPlayback_pause,
  putPlaylistPlayback_play,
} from "../endpoints.js";

function getPlayPlaybackbar() {
  return document.getElementById("playbackbar-play");
}

export function handlePlayPlaybackbar() {
  getPlayPlaybackbar().addEventListener("click", () => {
    togglePlayPause();
  });
}

async function isPlaying() {
  const playlistPlaybackbar = await getPlaylistPlayback();
  const playing = playlistPlaybackbar.playing;
  return playing;
}

export async function playTrackPlaybackbar() {
  const playPlaybackbar = getPlayPlaybackbar();
  const playButton = playPlaybackbar.children;

  await putPlaylistPlayback_play();

  playButton[0].src = "../icons/pause-icon.svg";
}

async function pauseTrackPlaybackbar() {
  const playPlaybackbar = getPlayPlaybackbar();
  const playButton = playPlaybackbar.children;

  await putPlaylistPlayback_pause();
  playButton[0].src = "../icons/play-icon.svg";
}

export async function togglePlayPause() {
  const playing = await isPlaying();

  if (playing === false) {
    playTrackPlaybackbar();
  } else {
    pauseTrackPlaybackbar();
  }
}
