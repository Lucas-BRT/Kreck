import { putPlaylistPlayback_next } from "../endpoints.js";
import { playTrackPlaybackbar } from "./handlePlayPlaybackbar.js";
import { getPlaybackTrackTitle } from "../components/getPlaybackTrackTitle.js";

export function handleNextPlaybackbar() {
  const nextButton = document.getElementById("playbackbar-next");

  nextButton.addEventListener("click", () => {
    nextTrack();
    playTrackPlaybackbar();
  });
}

async function nextTrack() {
  await putPlaylistPlayback_next();
  setTimeout(
    async () => {
      await getPlaybackTrackTitle();
    },

    100
  );
}
