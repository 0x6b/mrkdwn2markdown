use std::fmt::Display;

use serde::Deserialize;

use crate::types::{text_object::TextObject, BlockType, RichTextElement};

#[derive(Debug, Deserialize, Clone)]
pub struct Block {
    #[serde(rename = "type")]
    pub block_type: BlockType,

    // Rich text
    pub elements: Option<Vec<RichTextElement>>,

    // Section
    pub text: Option<TextObject>,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.block_type {
            BlockType::RichText => {
                if let Some(elements) = &self.elements {
                    for element in elements {
                        write!(f, "{element}")?
                    }
                }
            }
            BlockType::Section => {
                if let Some(text) = &self.text {
                    writeln!(f, "{text}")?
                }
            }
            _ => {}
        }

        Ok(())
    }
}
