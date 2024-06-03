import { renderPlaybackbar } from "./renderPlaybackbar.js";
import { getPlaylistPlayback } from "../endpoints.js";

export async function getPlaybackTrackTitle() {
  const playlist = await getPlaylistPlayback();
  const trackTitle = playlist.track.title;

  renderPlaybackbar(trackTitle);
}
