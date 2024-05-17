use kenku_control::Controller;

pub fn setup_kenku_controller(ip: String, port: u16) -> Option<Controller> {
    let controller = Controller::new(ip, port);
    Some(controller)
}
