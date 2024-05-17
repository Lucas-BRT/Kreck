use kenku_control::*;
use rocket::{fs::FileServer, get, Ignite, Rocket, Shutdown};

#[derive(Default)]
pub struct RocketShutdownHandle(pub Option<Shutdown>);

#[get("/tracks")]
pub async fn getTracks() {}

pub async fn setup_server(controller: Controller) -> Result<Rocket<Ignite>, rocket::Error> {
    let rocket_config = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8000))
        .merge(("workers", 4));

    rocket::custom(rocket_config)
        .manage(controller)
        .mount("/", FileServer::from("../src-mobile"))
        .ignite()
        .await
}
