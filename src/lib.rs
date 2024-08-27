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
    pub managed: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishedTask {
    pub id: String,
    pub value: String,
    pub managed: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateSnapshot {
    pub live: VecDeque<LiveTask>,
    pub finished: VecDeque<FinishedTask>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WebsocketOpKind {
    OverwriteState(StateSnapshot),
    InsLiveTask {
        id: String,
        value: String,
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
    RevLiveTask {
        id1: String,
        id2: String,
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
