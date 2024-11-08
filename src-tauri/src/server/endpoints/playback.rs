use kenku_control::{playlist::playback::*, Controller};
use rocket::{get, put, State};
use serde_json::{json, Value};

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
