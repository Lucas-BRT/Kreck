export async function getTracks() {
  return await fetch("/tracks")
    .then(async (res) => await res.json())
    .then((data) => data);
}

export function getPlaylistPlayback() {
  fetch("/playlist-playback")
    .then((res) => res.json())
    .then((data) => console.log(data));
}

export function putPlaylistPlaybacb_play() {
  fetch("/playlist-playback", (method = "PUT"))
    .then((res) => res.json())
    .then((data) => console.log(data));
}

export function putPlaylistPlayback_pause() {
  fetch("/playlist-playback")
    .then((res) => res.json())
    .then((data) => console.log(data));
}

export function postPlaylistPlayback_next() {
  fetch("/playlist-playback")
    .then((res) => res.json())
    .then((data) => console.log(data));
}

export function postPlaylistPlayback_previous() {
  fetch("/playlist-playback")
    .then((res) => res.json())
    .then((data) => console.log(data));
}
