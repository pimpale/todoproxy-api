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
    InvalidBase64,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpKind {
    LiveTaskInsNew {
        value: String,
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
    FinishedTaskNew {
        live_task_id: i64,
        status: super::TaskStatus,
    },
}

pub struct DoOpProps {
    pub api_key: String,
    pub op: OpKind,
}
