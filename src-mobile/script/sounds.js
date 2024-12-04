import { getSounds } from "./endpoints.js";
import { convertSounds } from "./handlers/handleSounds.js";
import { renderSoundsList } from "./components/renderSounds.js";
import { handlePlaySound } from "./handlers/handlePlaySounds.js";

const sounds = await getSounds();
const convertedSounds = convertSounds(sounds);

renderSoundsList(convertedSounds);
handlePlaySound(convertedSounds);
