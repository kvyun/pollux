use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crdts::VClock;

#[derive(Clone, Copy,  Hash, Serialize, Deserialize, Debug)]
pub struct CellId {
    timestamp: u64,
    id: Uuid,
}

impl CellId {
    pub fn new() -> Self {
        Self::at(Utc::now().timestamp_millis() as u64)
    }

    pub fn at(ts: u64) -> Self {
        Self {
            timestamp: ts,
            id: Uuid::new_v4(),
        }
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl PartialEq<Self> for CellId {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp && self.id == other.id
    }
}

impl Eq for CellId {}

impl PartialOrd for CellId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



impl Ord for CellId {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.timestamp.cmp(&other.timestamp) {
            Ordering::Equal => self.id.cmp(&other.id),
            everything_else => everything_else,
        }
    }
}

impl Default for CellId {
    fn default() -> Self {
        Self::at(0)
    }
}

impl fmt::Display for CellId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}", self.timestamp, self.id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Active,
    Inactive,
    Pending,
    Gone
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Status::Inactive => write!(f, "inactive"),
            Status::Active   => write!(f, "active"),
            Status::Gone     => write!(f, "gone"),
            Status::Pending  => write!(f, "pending")
        }
    }
}

pub struct Metadata {
    pub id: CellId,
    pub state: Status,
    pub endpoint: SocketAddr,
    pub version: VClock<CellId>,
    pub updated: DateTime<Utc>
}

impl Metadata {
    pub fn new(id: CellId, endpoint: SocketAddr) -> Self {
        let mut c = VClock::new();
        c.inc(id);

        Self {
            id,
            endpoint,
            state: Status::Active,
            version: c,
            updated: Utc::now()
        }
    }

    pub fn active(&self) -> bool {
        matches!(self.state, Status::Active | Status::Pending)
    }

    pub fn update(&mut self, state: Status, origin: CellId) {
        self.state = state;
        self.version.inc(origin);
        self.updated = Utc::now();
    }

    pub fn can_update(&self, other: &Metadata) -> bool {
        match self.version.partial_cmp(&other.version) {
            Some(Ordering::Equal)   => false,
            Some(Ordering::Greater) => false,
            Some(Ordering::Less)    => true,
            None                    => {
                match (self.state, other.state) {
                    (Status::Inactive, _)              => false,
                    (Status::Gone, _)                  => false,
                    (Status::Pending, Status::Inactive) => true,
                    (Status::Active, Status::Inactive)  => true,
                    (Status::Active, Status::Pending)   => true,
                    _                                 => false
                }
            }
        }
    }
}