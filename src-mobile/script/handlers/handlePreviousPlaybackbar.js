import { getPlaybackTrackTitle } from "../components/getPlaybackTrackTitle.js";
import { putPlaylistPlayback_previous } from "../endpoints.js";
import { playTrackPlaybackbar } from "./handlePlayPlaybackbar.js";

export function handlePreviousPlaybackbar() {
  const nextButton = document.getElementById("playbackbar-previous");
  nextButton.addEventListener("click", () => {
    previousTrack();
    playTrackPlaybackbar();
  });
}

async function previousTrack() {
  await putPlaylistPlayback_previous();
  setTimeout(
    async () => {
      await getPlaybackTrackTitle();
    },

    30
  );
}
