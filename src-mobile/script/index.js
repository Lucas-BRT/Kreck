import { renderTracksList } from "./components/renderTracks.js";
import { getTracks } from "./endpoints.js";
import { handlePlayButton } from "./handlers/handlePlayButton.js";
import { convertTracks } from "./handlers/handleTracks.js";
import { handlePlayPlaybackbar } from "./handlers/handlePlayPlaybackbar.js";
import { handleNextPlaybackbar } from "./handlers/handleNextPlaybackbar.js";
import { handlePreviousPlaybackbar } from "./handlers/handlePreviousPlaybackbar.js";
import { checkText } from "./components/checkTextAndAddCarroussel.js";

if ('serviceWorker' in navigator) {
  window.addEventListener("load", () => {
    navigator.serviceWorker.register('/sw.js').then(function(registration) {
      console.log('ServiceWorker registration successful with scope: ', registration.scope);
    }, function(error) {
      console.log('ServiceWorker registration failed: ', error);
    });
  })
}

const tracks = await getTracks();
const convertedTracks = convertTracks(tracks);

renderTracksList(convertedTracks);
handlePlayButton(convertedTracks);
handlePlayPlaybackbar();
handleNextPlaybackbar();
handlePreviousPlaybackbar();
checkText("track-name");
