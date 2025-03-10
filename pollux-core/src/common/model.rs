use std::fmt;
use std::fmt::Formatter;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};


#[derive(Clone, Copy, Eq, Hash, Serialize, Deserialize, PartialEq, Debug)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Active,
    Inactive,
    Pending,
    Gone
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            State::Inactive => write!(f, "inactive"),
            State::Active   => write!(f, "active"),
            State::Gone     => write!(f, "gone"),
            State::Pending  => write!(f, "pending")
        }
    }
}

pub struct Metadata {
    pub id: NodeId,
    pub state: State,
    pub endpoint: SocketAddr,
    pub version: u64,
    pub updated: DateTime<Utc>
}