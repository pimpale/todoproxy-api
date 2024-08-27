use serde::{Deserialize, Serialize};

use crate::TaskStatus;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsocketInitMessage {
    pub api_key: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManagedTaskNew {
    pub id: String,
    pub value: String,
    pub source: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManagedTaskFinish {
    pub id: String,
    pub status: TaskStatus
}
