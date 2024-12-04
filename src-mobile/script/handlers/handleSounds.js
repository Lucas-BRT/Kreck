class Sound {
  constructor(title, soundboard, id) {
    this.title = title;
    this.soundboard = soundboard;
    this.id = id;
  }
}

export function convertSounds(jsonSounds) {
  const convertedSounds = [];

  jsonSounds.soundboards.forEach((soundboard) => {
    soundboard.sounds.forEach((soundboardSoundId) => {
      const sound = jsonSounds.sounds.find((s) => s.id === soundboardSoundId);

      if (sound) {
        const convertedSound = new Sound(
          sound.title,
          soundboard.title,
          sound.id
        );
        convertedSounds.push(convertedSound);
      }
    });
  });

  return convertedSounds;
}
