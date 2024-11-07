use kenku_control::{playlist::playback::*, Controller};
use rocket::{get, put, State};
use serde_json::{json, Value};

#[get("/tracks")]
pub async fn get(controller: &State<Controller>) -> Result<Value, ()> {
    match controller.get_playlist().await {
        Ok(tracks) => Ok(json!(tracks)),
        Err(e) => Err(println!("{e}")),
    }
}
