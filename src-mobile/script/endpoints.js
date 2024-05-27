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

export async function getPlaylistPlayback() {
  const response = await fetch("/playlist-playback");
  const data = await response.json();
  return data;
}

export async function getSoundboardPlayback() {
  const response = await fetch("/soundboard-playback");
  const data = await response.json();
  return data;
}

export async function putPlayTrack(trackId) {
  const response = await fetch(`/play/${trackId}`, {
    method: "PUT",
  });
  const data = await response;
  return data;
}

export async function putPlaylistPlayback_play() {
  const response = await fetch("/playlist-playback-play", {
    method: "PUT",
  });
  const data = await response;
  console.log(data);
  return data;
}

export async function putPlaylistPlayback_pause() {
  const reponse = await fetch("/playlist-playback-pause", { method: "PUT" });
  const data = reponse;
  return data;
}

export async function putPlaylistPlayback_next() {
  const response = await fetch("/playlist-playback-next", { method: "PUT" });
  const data = response;
  console.log(data);
  return data;
}

export async function putPlaylistPlayback_previous() {
  const response = await fetch("/playlist-playback-previous", {
    method: "PUT",
  });
  const data = response;
  console.log(data);
  return data;
}
