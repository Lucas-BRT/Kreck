use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("The local ip is not avaliable.")]
    LocalIpAddressNotFound,
}
