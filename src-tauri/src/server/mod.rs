mod endpoints;

use endpoints::*;
use kenku_control::Controller;
use rocket::{fs::FileServer, routes, Ignite, Rocket};

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
