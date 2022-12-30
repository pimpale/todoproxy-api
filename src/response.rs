use serde::{Deserialize, Serialize};
use derive_more::Display;

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
    pub task_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishedTask {
    pub finished_task_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub value: String,
    pub status: super::TaskStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskUpdate {
    LiveTaskInsNew {
        value: String,
        live_task_id: i64,
        position: i64,
    },
    LiveTaskInsRestore {
        finished_task_id: i64,
    },
    LiveTaskEdit {
        live_task_id: i64,
        value: String,
    },
    LiveTaskDel {
        live_task_id: i64,
    },
    LiveTaskDelIns {
        live_task_id_del: i64,
        live_task_id_ins: i64,
    },
    FinishedTaskPush {
        finished_task_id: i64,
        value: String,
        status: super::TaskStatus,
    },
    FinishedTaskPushComplete {
        live_task_id: i64,
        finished_task_id: i64,
        status: super::TaskStatus,
    },
}

