use kenku_control::{playlist::playback::*, Controller};
use rocket::{get, put, State};
use serde_json::{json, Value};

#[get("/soundboard")]
pub async fn get(controller: &State<Controller>) -> Result<Value, ()> {
    match controller.get_soundboard_playback().await {
        Ok(soundboard) => Ok(json!(soundboard)),
        Err(e) => Err(println!("{e}")),
    }
}
