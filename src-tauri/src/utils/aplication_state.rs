use rocket::Shutdown;

#[derive(Default)]
pub struct RocketShutdownHandle(pub Option<Shutdown>);

