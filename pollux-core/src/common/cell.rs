use std::sync::{Arc};
use tokio::sync::RwLock;
use crate::common::model::{CellId, Metadata};

pub struct Cell {
    metadata: Arc<RwLock<Metadata>>
}

impl Cell {

}