use std::fmt::Display;

use serde::Deserialize;

use crate::types::{BlockType, RichTextElement};

#[derive(Debug, Deserialize, Clone)]
pub struct Block {
    #[serde(rename = "type")]
    pub block_type: BlockType,
    pub elements: Option<Vec<RichTextElement>>,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let BlockType::RichText = &self.block_type {
            match &self.elements {
                Some(elements) => {
                    for element in elements {
                        write!(f, "{element}")?
                    }
                }
                None => {}
            }
        }
        Ok(())
    }
}
