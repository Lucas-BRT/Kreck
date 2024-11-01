import { getSounds } from "./endpoints.js";
import { convertSounds } from "./handlers/handleSounds.js";
import { renderSoundsList } from "./components/renderSounds.js";

const sounds = await getSounds();
const convertedSounds = convertSounds(sounds);

renderSoundsList(convertedSounds);
