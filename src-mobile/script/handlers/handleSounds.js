class Sound {
  constructor(title, soundboard, id) {
    this.title = title;
    this.soundboard = soundboard;
    this.id = id;
  }
}

export function convertSounds(jsonSounds) {
  let convertedSounds = [];

  const soundboards = jsonSounds.soundboards;
  const sounds = jsonSounds.sounds;

  for (let i = 0; i < soundboards.length; i++) {
    const soundboard = soundboards[i];

    for (
      let soundboard_sound_index = 0;
      soundboard_sound_index < soundboard.sounds.length;
      soundboard_sound_index++
    ) {
      const soundboard_sound = soundboard.sounds[soundboard_sound_index];

      for (let sound_index = 0; sound_index < sounds.length; sound_index++) {
        const sound = sounds[sound_index];

        if (sound.id === soundboard_sound) {
          const convertedSound = new Sound(
            sound.title,
            soundboard.title,
            sound.id
          );

          convertedSounds.push(convertedSound);
          break;
        }
      }
    }
  }

  return convertedSounds;
}
