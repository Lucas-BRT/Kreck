use kenku_control::Controller;
use rocket::{get, State};
use serde_json::{json, Value};

#[get("/soundboard")]
pub async fn get(controller: &State<Controller>) -> Result<Value, ()> {
    match controller.get_soundboard_playback().await {
        Ok(soundboard) => Ok(json!(soundboard)),
        Err(e) => Err(println!("{e}")),
    }
}
