use std::ops::Deref;

use serde::Deserialize;

use crate::types::Block;

#[derive(Debug, Deserialize, Clone)]
pub struct Blocks {
    pub blocks: Vec<Block>,
}

impl Deref for Blocks {
    type Target = Vec<Block>;

    fn deref(&self) -> &Self::Target {
        &self.blocks
    }
}
