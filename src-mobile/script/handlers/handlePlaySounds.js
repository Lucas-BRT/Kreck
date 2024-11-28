import { putPlaySound, putStopSound, getSoundboard } from "../endpoints.js";

const playingSounds = [];

export function handlePlaySound(convertedSounds) {
  convertedSounds.forEach((sound) => {
    const container = document.getElementById(`${sound.id}`);

    container.addEventListener("click", async () => {
      const isPlaying = playingSounds.includes(sound.id);

      if (isPlaying) {
        putStopSound(sound.id)
          .then(() => {
            const index = playingSounds.indexOf(sound.id);
            if (index > -1) {
              playingSounds.splice(index, 1);
            }
            console.log(`Som parado: ${sound.title}`);
            container.classList.remove("active");
          })
          .catch((err) => {
            console.error(`Falha ao parar o som: ${err}`);
          });
      } else {
        try {
          await putPlaySound(sound.id);
          playingSounds.push(sound.id);
          console.log(`Tocando som: ${sound.title}`);
          container.classList.add("active");

          const delay = 1000;
          await new Promise((resolve) => setTimeout(resolve, delay));

          const data = await getSoundboard();
          const track = data.sounds.find((s) => s.id === sound.id);

          if (track && track.duration) {
            sound = track;
            const durationInMs = sound.duration * 1000;

            setTimeout(async () => {
              if (playingSounds.includes(sound.id)) {
                await putStopSound(sound.id);
                const index = playingSounds.indexOf(sound.id);
                if (index > -1) {
                  playingSounds.splice(index, 1);
                }
                console.log(`Som parado automaticamente: ${sound.title}`);
                container.classList.remove("active");
              }
            }, durationInMs);
          } else {
            console.error("Falha ao recuperar a duração do som.");
          }
        } catch (err) {
          console.error(`Falha ao tocar o som: ${err}`);
        }
      }
    });
  });
}
