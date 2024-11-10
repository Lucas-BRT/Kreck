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
            console.log(`Som parado: ${sound.id}`);
            container.classList.remove("active");
          })
          .catch((err) => {
            console.error(`Falha ao parar som: ${err}`);
          });
      } else {
        try {
          await putPlaySound(sound.id);
          playingSounds.push(sound.id);
          console.log(`Som tocando: ${sound.title}`);
          container.classList.add("active");

          let retries = 0;
          const maxRetries = 5;
          const retryDelay = 1000;

          while (retries < maxRetries) {
            const data = await getSoundboard();
            const track = data.sounds.find((s) => s.id === sound.id);

            if (track && track.duration) {
              console.log(`Duração do som encontrada: ${track.duration}`);
              sound = track;
              break;
            }

            retries++;
            console.log(
              `Aguardando dados de rastreamento... Tentativa ${retries}`
            );
            await new Promise((resolve) => setTimeout(resolve, retryDelay));
          }

          if (sound && sound.duration) {
            const durationInMs = sound.duration * 1000;
            console.log(`Duração do som em ms: ${durationInMs}`);

            setTimeout(async () => {
              if (playingSounds.includes(sound.id)) {
                await putStopSound(sound.id);
                const index = playingSounds.indexOf(sound.id);
                if (index > -1) {
                  playingSounds.splice(index, 1);
                }
                container.classList.remove("active");
                console.log(
                  `Som ${sound.title} interrompido automaticamente após ${sound.duration} segundos`
                );
              }
            }, durationInMs);
          } else {
            console.error(
              "Não foi possível recuperar a duração da faixa após várias tentativas."
            );
          }
        } catch (err) {
          console.error(`Falha ao reproduzir som: ${err}`);
        }
      }
    });
  });
}
