use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsocketInitMessage {
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsocketOpMessage(super::WebsocketOp);
