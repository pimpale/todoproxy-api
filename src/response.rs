use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    DecodeError,
    InternalServerError,
    Unauthorized,
    BadRequest,
    NotFound,
    Unknown,
}

impl std::error::Error for AppError {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTask {
    pub id: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishedTask {
    pub id: String,
    pub value: String,
    pub status: super::TaskStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStateCheckpoint {
    pub live: Vec<LiveTask>,
    pub finished: Vec<FinishedTask>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WebsocketServerUpdateMessage {
    OverwriteState(ServerStateCheckpoint ),
    LiveTaskInsNew {
        value: String,
        live_task_id: String,
        position: usize,
    },
    LiveTaskInsRestore {
        finished_task_id: String,
    },
    LiveTaskEdit {
        live_task_id: String,
        value: String,
    },
    LiveTaskDel {
        live_task_id: String,
    },
    LiveTaskDelIns {
        live_task_id_del: String,
        live_task_id_ins: String,
    },
    FinishedTaskPush {
        finished_task_id: String,
        value: String,
        status: super::TaskStatus,
    },
    FinishedTaskPushComplete {
        live_task_id: String,
        finished_task_id: String,
        status: super::TaskStatus,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub service: String,
    pub version_major: i64,
    pub version_minor: i64,
    pub version_rev: i64,
}
