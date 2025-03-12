use serde::{Deserialize, Serialize};
use crate::common::model::{Metadata, CellId};

pub enum GossipPayload {
    Join(Metadata),
    Leave(CellId),
    Heartbeat(CellId)
}