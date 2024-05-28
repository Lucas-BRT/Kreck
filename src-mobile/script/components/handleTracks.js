class Track {
  constructor(title, playlist_title, id) {
    this.title = title;
    this.playlist = playlist_title;
    this.id = id;
  }
}

function convertTracks(jsonTracks) {
  let convertedTracks = [];

  const playlists = jsonTracks.playlists;
  const tracks = jsonTracks.tracks;

  for (let i = 0; i < playlists.length; i++) {
    const playlist = playlists[i];

    for (
      let playlist_track_index = 0;
      playlist_track_index < playlist.tracks.length;
      playlist_track_index++
    ) {
      const playlist_track = playlist.tracks[playlist_track_index];

      for (let track_index = 0; track_index < tracks.length; track_index++) {
        const track = tracks[track_index];

        if (track.id === playlist_track) {
          const convertedTrack = new Track(
            track.title,
            playlist.title,
            track.id
          );

          convertedTracks.push(convertedTrack);
          break;
        }
      }
    }
  }

  return convertedTracks;
}
