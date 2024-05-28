import { getTrackTemplate } from "../templates/TrackTemplate.js";

export function renderTracksList(tracks) {
  const tracksContainer = document.querySelector(".content");

  tracks.forEach((track) => {
    const trackTemplate = getTrackTemplate(track);
    tracksContainer.appendChild(trackTemplate);
  });
}
