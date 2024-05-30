import { putFade } from "./putFade.js";

export function checkplaybackbarText() {
  const playbackbar = document.querySelector(`.playbackbar-track-name`);

  const textElement = playbackbar.textContent;

  switch (true) {
    case textElement.length > 21 && textElement.length < 40:
      playbackbar.className = `carousel-small-playbackbar playbackbar-track-name `;
      playbackbar.parentElement.classList.remove("no-fade");

      putFade("playbackbar-name-fade", "carousel-small-playbackbar", 20000);
      break;
    case textElement.length > 40 && textElement.length < 90:
      playbackbar.className = `carousel-medium-playbackbar playbackbar-track-name `;
      playbackbar.parentElement.classList.remove("no-fade");

      putFade("playbackbar-name-fade", "carousel-medium-playbackbar", 25000);

      break;
    case textElement.length > 90:
      playbackbar.className = `carousel-large-playbackbar playbackbar-track-name `;
      playbackbar.parentElement.classList.remove("no-fade");

      putFade("playbackbar-name-fade", "carousel-large-playbackbar", 30000);

      break;

    default:
      playbackbar.className = `playbackbar-track-name `;
      playbackbar.parentElement.classList.add("no-fade");
  }
}
