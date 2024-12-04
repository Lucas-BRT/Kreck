import { getSoundTemplate } from "../templates/SoundTemplate.js";

export function renderSoundsList(sounds) {
  const soundsContainer = document.querySelector(".content");
  soundsContainer.innerHTML = "";

  const closeButton = document.createElement("div");
  closeButton.className = "container-sound";
  closeButton.id = "container-close";
  closeButton.innerHTML = `<div class ="sound-name"><h1><a href="./index.html">X</a></h1></div>`;
  soundsContainer.prepend(closeButton);

  sounds.forEach((sound) => {
    const soundTemplate = getSoundTemplate(sound);
    soundsContainer.appendChild(soundTemplate);
  });
}
