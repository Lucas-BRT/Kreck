function createContainer() {
  return document.createElement("div");
}

export function getTrackTemplate(track) {
  let container = createContainer();

  container.innerHTML = `
    <div class="track-informations ">
     <div class="track-name">${track.title}</div>
     <div class="track-playlist">${track.playlist}</div>
      </div>
      <div class="track-player">  
      <div class="play-button" id="${track.id}">
      <div class="player-button-triangle">
      </div>
     </div>
    </div>`;

  container.className = "container-track";

  return container;
}
