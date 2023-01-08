// Types of arguments for auth handlers
pub mod request;
pub mod response;

use std::collections::VecDeque;

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromPrimitive)]
pub enum TaskStatus {
    Obsoleted,
    Succeeded,
    Failed,
}

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
    pub status: TaskStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateSnapshot {
    pub live: VecDeque<LiveTask>,
    pub finished: Vec<FinishedTask>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WebsocketOpKind {
    OverwriteState(StateSnapshot),
    InsLiveTask {
        value: String,
        id: String,
    },
    RestoreFinishedTask {
        id: String,
    },
    EditLiveTask {
        id: String,
        value: String,
    },
    DelLiveTask {
        id: String,
    },
    MvLiveTask {
        id_del: String,
        id_ins: String,
    },
    FinishLiveTask {
        id: String,
        status: TaskStatus,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsocketOp {
    pub alleged_time: i64,
    pub kind: WebsocketOpKind,
}
