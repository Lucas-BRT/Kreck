import { getSounds } from "./endpoints.js";
import { convertSounds } from "./handlers/handleSounds.js";

const sounds = await getSounds();
const convertedSounds = convertSounds(sounds);
