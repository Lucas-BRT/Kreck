use kenku_control::*;
use rocket::{fs::FileServer, get, routes, Ignite, Rocket, Shutdown, State};
use serde_json::{json, Value};

#[derive(Default)]
pub struct RocketShutdownHandle(pub Option<Shutdown>);

#[get("/tracks")]
pub async fn get_tracks(controller: &State<Controller>) -> Value {
    let tracks = controller.get_playlist().await.unwrap();

    json!(tracks.tracks)
}

#[get("/sounds")]
pub async fn get_sounds(controller: &State<Controller>) -> Value {
    let sounds = controller.get_soundboard().await.unwrap();

    json!(sounds.sounds)
}

#[get("/playlist-playback")]
pub async fn get_playlist_playback(controller: &State<Controller>) -> Value {
    let playback = controller.get_playlist_playback().await.unwrap();

    json!(playback)
}

#[get("/soundboard-playback")]
pub async fn get_soundboard_playback(controller: &State<Controller>) -> Value {
    let soundboard = controller.get_soundboard_playback().await.unwrap();

    json!(soundboard)
}

pub async fn setup_server(controller: Controller) -> Result<Rocket<Ignite>, rocket::Error> {
    let rocket_config = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8000))
        .merge(("workers", 4));

    rocket::custom(rocket_config)
        .manage(controller)
        .mount("/", FileServer::from("../src-mobile"))
        .mount("/", routes![get_tracks, get_sounds, get_playlist_playback, get_soundboard_playback])
        .ignite()
        .await
}
