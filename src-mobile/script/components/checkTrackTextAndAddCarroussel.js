import { putFade } from "./putFade.js";

export function checkTrackText() {
  const tracks = document.querySelectorAll(`.content-track-name`);

  tracks.forEach((track) => {
    const textElement = track.textContent;

    switch (true) {
      case textElement.length > 21 && textElement.length < 40:
        track.className = `carousel-small-track content-track-name `;
        putFade("track-name-fade", "carousel-small-track", 20000);
        break;
      case textElement.length > 40 && textElement.length < 90:
        track.className = `carousel-medium-track content-track-name `;
        putFade("track-name-fade", "carousel-medium-track", 25000);

        break;
      case textElement.length > 90:
        track.className = `carousel-large-track content-track-name `;
        putFade("track-name-fade", "carousel-large-track", 30000);

        break;

      default:
        track.className = `content-track-name `;
    }
  });
}
