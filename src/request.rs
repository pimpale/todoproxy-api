use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsocketInitMessage {
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HabiticaIntegrationNewProps {
    pub integration_user_id: String,
    pub integration_api_key: String,
    pub active: bool,
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HabiticaIntegrationViewProps {
    pub api_key: String,
}
