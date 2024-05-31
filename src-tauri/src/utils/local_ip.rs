use local_ip_address::local_ip;

pub fn get_local_ip() -> Result<String, String> {
    match local_ip() {
        Ok(addres) => Ok(addres.to_string()),
        Err(_) => Err("Failed to get local ip".to_string())
    }
}

