mod endpoints;

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
                endpoints::tracks::get,
                endpoints::tracks::play,
                endpoints::sounds::get,
                endpoints::sounds::play,
                endpoints::sounds::stop,
                endpoints::playback::get,
                endpoints::playback::play,
                endpoints::playback::pause,
                endpoints::playback::next,
                endpoints::playback::previous,
                endpoints::soundboard::get
            ],
        )
        .ignite()
        .await
}
