import { putPlaySound, putStopSound } from "../endpoints.js";

const playingSounds = new Set();

export function handlePlaySound(convertedSounds) {
  convertedSounds.forEach((sound) => {
    const container = document.getElementById(`${sound.id}`);

    container.addEventListener("click", () => {
      if (playingSounds.has(sound.id)) {
        putStopSound(sound.id)
          .then(() => {
            playingSounds.delete(sound.id);
            console.log(`Stopped sound: ${sound.id}`);
            container.classList.remove("active");
          })
          .catch((err) => {
            console.error(`Failed to stop sound: ${err}`);
          });
      } else {
        putPlaySound(sound.id)
          .then(() => {
            playingSounds.add(sound.id);
            console.log(`Playing sound: ${sound.id}`);
            container.classList.add("active");
          })
          .catch((err) => {
            console.error(`Failed to play sound: ${err}`);
          });
      }
    });
  });
}
