use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WebsocketClientOpMessage {
    LiveTaskInsNew {
        value: String,
        position: i64,
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
    FinishedTaskNew {
        live_task_id: String,
        status: super::TaskStatus,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsocketClientInitMessage {
    pub api_key: String,
}
