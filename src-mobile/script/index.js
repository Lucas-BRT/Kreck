import { renderTracksList } from "./components/renderTracks.js";
import { getTracks } from "./endpoints.js";
import { handlePlayButton } from "./handlers/handlePlayButton.js";
import { convertTracks } from "./handlers/handleTracks.js";
import { handlePlayPlaybackbar } from "./handlers/handlePlayPlaybackbar.js";
import { handleNextPlaybackbar } from "./handlers/handleNextPlaybackbar.js";
import { handlePreviousPlaybackbar } from "./handlers/handlePreviousPlaybackbar.js";
import { checkText } from "./components/checkTextAndAddCarroussel.js";

const tracks = await getTracks();
const convertedTracks = convertTracks(tracks);

renderTracksList(convertedTracks);
handlePlayButton(convertedTracks);
handlePlayPlaybackbar();
handleNextPlaybackbar();
handlePreviousPlaybackbar();
checkText("track-name");
