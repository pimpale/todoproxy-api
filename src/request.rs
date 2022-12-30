use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WebsocketClientOpMessage {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsocketClientInitMessage {
    pub api_key: String,
}
