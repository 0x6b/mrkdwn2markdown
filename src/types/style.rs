use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Style {
    pub code: Option<bool>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub strike: Option<bool>,
}
