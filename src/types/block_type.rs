use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    RichText,
    Section,

    Actions, // not supported
    Context, // not supported
    Divider, // not supported
    File,    // not supported
    Header,  // not supported
    Image,   // not supported
    Input,   // not supported
    Video,   // not supported
}
