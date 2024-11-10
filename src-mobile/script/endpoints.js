export async function getTracks() {
  const response = await fetch("/tracks");
  const data = await response.json();
  return data;
}

export async function getSounds() {
  const response = await fetch("/sounds");
  const data = await response.json();
  return data;
}
export async function putPlaySound(soundId) {
  const response = await fetch(`/sounds/play/${soundId}`, {
    method: "PUT",
  });
  return response;
}
export async function putStopSound(soundId) {
  const response = await fetch(`/sounds/stop/${soundId}`, { method: "PUT" });
  return response;
}

export async function getPlaylistPlayback() {
  const response = await fetch("/playlist");
  const data = await response.json();
  return data;
}

export async function getSoundboardPlayback() {
  const response = await fetch("/soundboard-playback");
  const data = await response.json();
  return data;
}

export async function putPlayTrack(trackId) {
  const response = await fetch(`/tracks/play/${trackId}`, {
    method: "PUT",
  });
  const data = await response;
  return data;
}

export async function putPlaylistPlayback_play() {
  const response = await fetch("/playlist/play", {
    method: "PUT",
  });
  const data = await response;
  return data;
}

export async function putPlaylistPlayback_pause() {
  const reponse = await fetch("/playlist/pause", { method: "PUT" });
  const data = reponse;
  return data;
}

export async function putPlaylistPlayback_next() {
  const response = await fetch("/playlist/next", { method: "PUT" });
  const data = response;
  return data;
}

export async function putPlaylistPlayback_previous() {
  const response = await fetch("/playlist/previous", {
    method: "PUT",
  });
  const data = response;
  return data;
}
