use rocket::{fs::FileServer, Ignite, Rocket, Shutdown};

#[derive(Default)]
pub struct RocketShutdownHandle(pub Option<Shutdown>);

pub async fn setup_server() -> Result<Rocket<Ignite>, rocket::Error> {
    let rocket_config = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8000))
        .merge(("workers", 4));

    rocket::custom(rocket_config)
        .mount("/", FileServer::from("../src"))
        .ignite()
        .await
}
