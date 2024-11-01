function createContainer() {
  return document.createElement("div");
}

export function getSoundTemplate(sound) {
  let container = createContainer();

  container.className = "container-sound";
  container.id = sound.id;

  container.innerHTML = `
      <div class="sound-name"><h1>${sound.title}</h1></div>
    `;

  return container;
}
