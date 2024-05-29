use kenku_control::*;
use rocket::{fs::FileServer, get, put, routes, Ignite, Rocket, Shutdown, State};
use serde_json::{json, Value};

#[derive(Default)]
pub struct RocketShutdownHandle(pub Option<Shutdown>);

#[get("/tracks")]
pub async fn get_tracks(controller: &State<Controller>) -> Result<Value, ()> {
    match controller.get_playlist().await {
        Ok(tracks) => Ok(json!(tracks)),
        Err(e) => Err(println!("{e}"))
    }
}

#[get("/sounds")]
pub async fn get_sounds(controller: &State<Controller>) -> Result<Value, ()> {
    match controller.get_soundboard().await {
        Ok(sounds) => Ok(json!(sounds)),
        Err(e) => Err(println!("{e}"))
    }
}

#[get("/playlist-playback")]
pub async fn get_playlist_playback(controller: &State<Controller>) -> Result<Value, ()> {
    match controller.get_playlist_playback().await {
        Ok(playback) => Ok(json!(playback)),
        Err(e) => Err(println!("{e}"))
    }
}

#[get("/soundboard-playback")]
pub async fn get_soundboard_playback(controller: &State<Controller>) -> Result<Value, ()> {
    match controller.get_soundboard_playback().await {
        Ok(soundboard) => Ok(json!(soundboard)),
        Err(e) => Err(println!("{e}"))
    }
}

#[put("/play/<track_id>")]
pub async fn play_track(controller: &State<Controller>, track_id: String) {
    let tracks = controller.get_playlist().await.unwrap().tracks;

    for track in tracks {
        if track.id == track_id {
            track.play(&controller).await.unwrap();
        }
    }
}

#[put("/playlist-playback-pause")]
pub async fn pause_playback(controller: &State<Controller>) {
    playlist::playback::playback_pause(&controller)
        .await
        .unwrap();
}

#[put("/playlist-playback-play")]
pub async fn play_playback(controller: &State<Controller>) {
    playlist::playback::playback_play(&controller)
        .await
        .unwrap();
}

#[put("/playlist-playback-next")]
pub async fn next_playback(controller: &State<Controller>) {
    playlist::playback::playback_next(&controller)
        .await
        .unwrap();
}

#[put("/playlist-playback-previous")]
pub async fn previous_playback(controller: &State<Controller>) {
    playlist::playback::playback_previous(&controller)
        .await
        .unwrap();
}

pub async fn setup_server(controller: Controller) -> Result<Rocket<Ignite>, rocket::Error> {
    let rocket_config = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8000))
        .merge(("workers", 4));

    rocket::custom(rocket_config)
        .manage(controller)
        .mount("/", FileServer::from("../src-mobile"))
        .mount(
            "/",
            routes![
                get_tracks,
                get_sounds,
                get_playlist_playback,
                get_soundboard_playback,
                play_track,
                pause_playback,
                play_playback,
                next_playback,
                previous_playback
            ],
        )
        .ignite()
        .await
}
