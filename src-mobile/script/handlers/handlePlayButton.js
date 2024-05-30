import { checkplaybackbarText } from "../components/checkPlaybackbarTextAndAddCarousel.js";
import { renderPlaybackbar } from "../components/renderPlaybackbar.js";
import { putPlayTrack } from "../endpoints.js";
import { playTrackPlaybackbar } from "./handlePlayPlaybackbar.js";

export function handlePlayButton(convertedTrack) {
  convertedTrack.forEach((track) => {
    const button = document.getElementById(`${track.id}`);

    button.addEventListener("click", () => {
      putPlayTrack(track.id);
      renderPlaybackbar(track.title);
      checkplaybackbarText();
      playTrackPlaybackbar();
    });
  });
}
