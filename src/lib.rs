// Types of arguments for auth handlers
pub mod request;
pub mod response;

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromPrimitive)]
pub enum TaskStatus {
    Obsoleted,
    Succeeded,
    Failed,
}

