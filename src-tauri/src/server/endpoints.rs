use kenku_control::{playlist::playback::*, Controller};
use rocket::{get, put, State};
use serde_json::{json, Value};

#[allow(dead_code)]
pub mod tracks {
    use super::*;

    #[get("/tracks")]
    pub async fn get(controller: &State<Controller>) -> Result<Value, ()> {
        match controller.get_playlist().await {
            Ok(tracks) => Ok(json!(tracks)),
            Err(e) => Err(println!("{e}")),
        }
    }

    #[put("/tracks/play/<track_id>")]
    pub async fn play(controller: &State<Controller>, track_id: String) {
        match controller.get_playlist().await {
            Ok(tracks) => {
                for track in tracks.tracks {
                    if track.id == track_id {
                        track.play(controller).await.unwrap();
                    }
                }
            }
            Err(e) => println!("{e}"),
        };
    }
}

#[allow(dead_code)]
pub mod playback {
    use super::*;

    #[get("/playlist")]
    pub async fn get(controller: &State<Controller>) -> Result<Value, ()> {
        match controller.get_playlist_playback().await {
            Ok(playback) => Ok(json!(playback)),
            Err(e) => Err(println!("{e}")),
        }
    }

    #[put("/playlist/pause")]
    pub async fn pause(controller: &State<Controller>) {
        match playback_pause(controller).await {
            Ok(_) => println!("PAUSE PLAYBACK"),
            Err(e) => println!("{e}"),
        }
    }

    #[put("/playlist/play")]
    pub async fn play(controller: &State<Controller>) {
        match playback_play(controller).await {
            Ok(_) => println!("PLAY PLAYBACK"),
            Err(e) => println!("{e}"),
        }
    }

    #[put("/playlist/next")]
    pub async fn next(controller: &State<Controller>) {
        match playback_next(controller).await {
            Ok(_) => println!("NEXT PLAYBACK"),
            Err(e) => println!("{e}"),
        }
    }

    #[put("/playlist/previous")]
    pub async fn previous(controller: &State<Controller>) {
        match playback_previous(controller).await {
            Ok(_) => println!("PREVIOUS PLAYBACK"),
            Err(e) => println!("{e}"),
        }
    }
}

#[allow(dead_code)]
pub mod sounds {
    use super::*;

    #[get("/sounds")]
    pub async fn get(controller: &State<Controller>) -> Result<Value, ()> {
        match controller.get_soundboard().await {
            Ok(sounds) => Ok(json!(sounds)),
            Err(e) => Err(println!("{e}")),
        }
    }

    #[put("/sounds/play/<sound_id>")]
    pub async fn play(controller: &State<Controller>, sound_id: String) {
        match controller.get_soundboard().await {
            Ok(sounds) => {
                for sound in sounds.sounds {
                    if sound.id == sound_id {
                        sound.play(controller).await.unwrap();
                    }
                }
            }
            Err(e) => println!("{e}"),
        }
    }

    #[put("/sounds/stop/<sound_id>")]
    pub async fn stop(controller: &State<Controller>, sound_id: String) {
        match controller.get_soundboard().await {
            Ok(sounds) => {
                for sound in sounds.sounds {
                    if sound.id == sound_id {
                        sound.stop(controller).await.unwrap();
                    }
                }
            }
            Err(e) => println!("{e}"),
        }
    }
}

#[allow(dead_code)]
pub mod soundboard {
    use super::*;
    #[get("/soundboard")]
    pub async fn get(controller: &State<Controller>) -> Result<Value, ()> {
        match controller.get_soundboard_playback().await {
            Ok(soundboard) => Ok(json!(soundboard)),
            Err(e) => Err(println!("{e}")),
        }
    }
}
